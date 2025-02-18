#![forbid(unsafe_code)]

use crate::backend::android::content::Context;
use crate::backend::android::hardware::usb::{
    UsbConstants, UsbDevice, UsbDeviceConnection, UsbEndpoint, UsbInterface, UsbManager,
};
use crate::backend::android::os::BuildVersion;
use crate::backend::android::{get_android_context, request_permissions};
use crate::communication::Version;
use crate::iotzio::{USB_MANUFACTURER_NAME, USB_PRODUCT_ID, USB_PRODUCT_NAME_PREFIX, USB_VENDOR_ID};
use crate::IotzioInfo;
use async_oneshot::oneshot;
use async_std::channel::bounded;
use jni::{JNIEnv, JavaVM};
use std::hash::{DefaultHasher, Hash, Hasher};
use std::sync::Arc;
use std::thread;
use thiserror_no_std::Error;

pub async fn list_connected_boards(parse_version: fn(&str) -> Option<Version>) -> Result<Vec<IotzioInfo>, DeviceError> {
    let (java_vm, context_object) = get_android_context()?;

    let mut env = java_vm.attach_current_thread()?;

    let context = Context::new(&context_object, &mut env)?;

    let usb_manager = context.get_usb_manager(&mut env)?;

    let hash_map = usb_manager.get_device_list(&mut env)?;

    let serial_number_restricted = BuildVersion::current(&mut env)? >= BuildVersion::Q;

    Ok(hash_map
        .into_values()
        .into_iter()
        .filter_map(|x| {
            get_iotzio_info(
                x,
                &mut env,
                parse_version,
                serial_number_restricted,
                &java_vm,
                &context,
                &usb_manager,
            )
            .ok()
            .flatten()
        })
        .collect())
}

fn get_iotzio_info(
    usb_device: UsbDevice,
    env: &mut JNIEnv,
    parse_version: fn(&str) -> Option<Version>,
    serial_number_restricted: bool,
    java_vm: &Arc<JavaVM>,
    context: &Context,
    usb_manager: &UsbManager,
) -> Result<Option<IotzioInfo>, DeviceError> {
    let product_name = usb_device.get_product_name(env)?;

    if product_name
        .map(|x| x.starts_with(USB_PRODUCT_NAME_PREFIX))
        .unwrap_or(false)
        && usb_device
            .get_manufacturer_name(env)?
            .map(|x| USB_MANUFACTURER_NAME.eq(&x))
            .unwrap_or(false)
        && usb_device.get_vendor_id(env)? == USB_VENDOR_ID
        && usb_device.get_product_id(env)? == USB_PRODUCT_ID
    {
        let mut serial_number = None;

        if !serial_number_restricted || usb_manager.has_permission(env, &usb_device)? {
            serial_number = usb_device.get_serial_number(env)?
        }

        let runtime_identifier = {
            let mut hasher = DefaultHasher::new();

            usb_device.get_device_id(env)?.hash(&mut hasher);

            hasher.finish()
        };

        return Ok(usb_device
            .get_product_name(env)?
            .and_then(|x| parse_version(&x))
            .map(|version| IotzioInfo {
                device_info: DeviceInfo {
                    device_info: usb_device,
                    java_vm: java_vm.clone(),
                    manager: usb_manager.clone(),
                    context: context.clone(),
                }
                .into(),
                version,
                serial_number,
                runtime_identifier,
            }));
    }

    Ok(None)
}

#[derive(Debug)]
pub struct DeviceInfo {
    java_vm: Arc<JavaVM>,
    context: Context,
    device_info: UsbDevice,
    manager: UsbManager,
}

impl DeviceInfo {
    pub async fn open(&self) -> Result<(DeviceReader, DeviceWriter), DeviceError> {
        let (tx, rx) = oneshot::<Result<(), DeviceError>>();

        request_permissions(&self.java_vm, &self.context, &self.manager, &self.device_info, tx)?;

        _ = rx.await.map_err(|_| DeviceError::IOError {
            error_message: "Failed to request USB device permissions.",
        })??;

        open_usb_device(&self.java_vm, &self.manager, &self.device_info)
    }
}

fn open_usb_device(
    java_vm: &Arc<JavaVM>,
    manager: &UsbManager,
    device_info: &UsbDevice,
) -> Result<(DeviceReader, DeviceWriter), DeviceError> {
    let mut env = java_vm.attach_current_thread()?;

    let device = match manager.open_device(&mut env, device_info)? {
        None => {
            return Err(DeviceError::IOError {
                error_message: "Failed to open USB device.",
            })
        }
        Some(x) => x,
    };

    let interface = device_info.get_interface(&mut env, 0)?;

    match device.claim_interface(&mut env, &interface, true)? {
        true => {}
        false => {
            return Err(DeviceError::IOError {
                error_message: "Failed to claim USB device interface.",
            })
        }
    }

    let mut input: Option<UsbEndpoint> = None;
    let mut output: Option<UsbEndpoint> = None;

    for index in 0..interface.get_endpoint_count(&mut env)? {
        let endpoint = interface.get_endpoint(&mut env, index)?;

        match (endpoint.get_type(&mut env)?, endpoint.get_direction(&mut env)?) {
            (UsbConstants::USB_ENDPOINT_XFER_INT, UsbConstants::USB_DIR_IN) => input = Some(endpoint),
            (UsbConstants::USB_ENDPOINT_XFER_INT, UsbConstants::USB_DIR_OUT) => output = Some(endpoint),
            (_, _) => {}
        }
    }

    match (input, output) {
        (Some(input), Some(output)) => {
            let data = DeviceData {
                device,
                interface,
                input,
                output,
                java_vm: java_vm.clone(),
            };

            let thread_data = Arc::new(data.clone());
            let device_data = Arc::new(data);

            let reader = new_device_reader(java_vm.clone(), thread_data.clone(), device_data.clone())?;
            let writer = new_device_writer(java_vm.clone(), thread_data, device_data)?;

            Ok((reader, writer))
        }
        (_, _) => Err(DeviceError::IOError {
            error_message: "Failed to find USB device endpoints.",
        }),
    }
}

#[derive(Debug, Clone)]
struct DeviceData {
    device: UsbDeviceConnection,
    interface: UsbInterface,
    input: UsbEndpoint,
    output: UsbEndpoint,
    java_vm: Arc<JavaVM>,
}

impl Drop for DeviceData {
    fn drop(&mut self) {
        let mut env = match self.java_vm.attach_current_thread() {
            Ok(x) => x,
            Err(_) => return,
        };

        _ = self.device.release_interface(&mut env, &self.interface);
        _ = self.device.close(&mut env);
    }
}

#[derive(Debug)]
pub struct DeviceReader {
    sender: async_std::channel::Sender<DeviceReaderChannelEntry>,
    _data: Arc<DeviceData>,
}

#[derive(Debug)]
struct DeviceReaderChannelEntry {
    buffer: Vec<u8>,
    sender: async_oneshot::Sender<Result<(Vec<u8>, usize), DeviceError>>,
}

fn new_device_reader(
    java_vm: Arc<JavaVM>,
    thread_data: Arc<DeviceData>,
    device_data: Arc<DeviceData>,
) -> Result<DeviceReader, DeviceError> {
    let (tx, rx) = bounded::<DeviceReaderChannelEntry>(1);

    thread::spawn(move || {
        let mut env = match java_vm.attach_current_thread_as_daemon() {
            Ok(x) => x,
            Err(_) => {
                rx.close();
                return;
            }
        };

        loop {
            match rx.recv_blocking() {
                Ok(mut entry) => {
                    let result = thread_data
                        .device
                        .bulk_transfer_in(&mut env, &thread_data.input, &mut entry.buffer, 0)
                        .map(|x| (entry.buffer, x));

                    _ = entry.sender.send(result); // Error is ok. read_input_report future was dropped.
                }
                Err(_) => break,
            }
        }

        drop(thread_data);
        drop(env);
    });

    Ok(DeviceReader {
        sender: tx,
        _data: device_data,
    })
}

impl DeviceReader {
    pub async fn read_input_report(&mut self, buffer: &mut [u8]) -> Result<usize, DeviceError> {
        let (tx, rx) = oneshot();

        let entry = DeviceReaderChannelEntry {
            buffer: buffer.to_vec(),
            sender: tx,
        };

        self.sender.send(entry).await.map_err(|_| DeviceError::IOError {
            error_message: "IO Thread Reader is terminated.",
        })?;

        let (vec, length) = rx.await.map_err(|_| DeviceError::IOError {
            error_message: "IO Thread Reader is terminated.",
        })??;

        (&mut buffer[..length]).copy_from_slice(&vec[..length]);

        Ok(length)
    }
}

#[derive(Debug)]
pub struct DeviceWriter {
    sender: async_std::channel::Sender<DeviceWriterChannelEntry>,
    _data: Arc<DeviceData>,
}

#[derive(Debug)]
struct DeviceWriterChannelEntry {
    buffer: Vec<u8>,
    sender: async_oneshot::Sender<Result<(), DeviceError>>,
}

fn new_device_writer(
    java_vm: Arc<JavaVM>,
    thread_data: Arc<DeviceData>,
    device_data: Arc<DeviceData>,
) -> Result<DeviceWriter, DeviceError> {
    let (tx, rx) = bounded::<DeviceWriterChannelEntry>(1);

    thread::spawn(move || {
        let mut env = match java_vm.attach_current_thread_as_daemon() {
            Ok(x) => x,
            Err(_) => {
                rx.close();
                return;
            }
        };

        loop {
            match rx.recv_blocking() {
                Ok(mut entry) => {
                    let result = thread_data
                        .device
                        .bulk_transfer_out(&mut env, &thread_data.output, &mut entry.buffer, 0)
                        .map(|_| ());

                    _ = entry.sender.send(result); // Error is ok. write_output_report future was dropped.
                }
                Err(_) => break,
            }
        }

        drop(thread_data);
        drop(env);
    });

    Ok(DeviceWriter {
        sender: tx,
        _data: device_data,
    })
}

impl DeviceWriter {
    pub async fn write_output_report(&mut self, buffer: &mut [u8]) -> Result<(), DeviceError> {
        let (tx, rx) = oneshot();

        let entry = DeviceWriterChannelEntry {
            buffer: buffer.to_vec(),
            sender: tx,
        };

        self.sender.send(entry).await.map_err(|_| DeviceError::IOError {
            error_message: "IO Thread Writer is terminated.",
        })?;

        rx.await.map_err(|_| DeviceError::IOError {
            error_message: "IO Thread Writer is terminated.",
        })??;

        Ok(())
    }
}

#[non_exhaustive]
#[derive(Error, Debug)]
pub enum DeviceError {
    #[error("JNI Error: {error}")]
    JniError { error: jni::errors::Error },
    #[error("{class}: {message}")]
    JavaException { class: String, message: String },
    #[error("{error_message}")]
    JavaWrapperError { error_message: &'static str },
    #[error("{error_message}")]
    NdkContextError { error_message: &'static str },
    #[error("IO Error: {error_message}")]
    IOError { error_message: &'static str },
    #[error("User did not grant device access permission.")]
    NoPermissionGranted,
}

impl From<jni::errors::Error> for DeviceError {
    fn from(value: jni::errors::Error) -> Self {
        DeviceError::JniError { error: value }
    }
}
