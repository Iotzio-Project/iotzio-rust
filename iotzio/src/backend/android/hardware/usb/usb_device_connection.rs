#![forbid(unsafe_code)]

use crate::backend::android::hardware::usb::{UsbEndpoint, UsbInterface};
use crate::backend::android::utils;
use crate::backend::DeviceError;
use jni::objects::{GlobalRef, JByteArray, JObject, JValue};
use jni::sys::jint;
use jni::JNIEnv;
use std::ops::Deref;

#[derive(Debug, Clone)]
pub struct UsbDeviceConnection {
    instance: GlobalRef,
}

impl Deref for UsbDeviceConnection {
    type Target = JObject<'static>;

    fn deref(&self) -> &Self::Target {
        self.instance.as_obj()
    }
}

impl UsbDeviceConnection {
    pub fn new(instance: &JObject, env: &mut JNIEnv) -> Result<Self, DeviceError> {
        let instance = env.new_global_ref(instance)?;

        let class = env.find_class("android/hardware/usb/UsbDeviceConnection")?;

        match !instance.as_obj().is_null() && env.is_instance_of(instance.as_obj(), class)? {
            true => Ok(Self { instance }),
            false => Err(DeviceError::JavaWrapperError {
                error_message: "instanceof check for android/hardware/usb/UsbDeviceConnection failed ",
            }),
        }
    }

    pub fn claim_interface(
        &self,
        env: &mut JNIEnv,
        usb_interface: &UsbInterface,
        force: bool,
    ) -> Result<bool, DeviceError> {
        let result = utils::call_method(
            env,
            self.instance.as_obj(),
            "claimInterface",
            "(Landroid/hardware/usb/UsbInterface;Z)Z",
            &[usb_interface.deref().into(), force.into()],
        )?
        .z()?;

        Ok(result)
    }

    pub fn bulk_transfer_out(
        &self,
        env: &mut JNIEnv,
        endpoint: &UsbEndpoint,
        buffer: &[u8],
        timeout: i32,
    ) -> Result<usize, DeviceError> {
        let buffer_obj = env.byte_array_from_slice(buffer)?;

        let result = utils::call_method(
            env,
            self.instance.as_obj(),
            "bulkTransfer",
            "(Landroid/hardware/usb/UsbEndpoint;[BIII)I",
            &[
                JValue::from(endpoint.deref()),
                JValue::from(&JObject::from(buffer_obj)),
                JValue::from(0 as jint),            // Offset
                JValue::from(buffer.len() as jint), // Length
                JValue::from(timeout),
            ],
        )?
        .i()?;

        match result {
            x if x < 0 => Err(DeviceError::IOError {
                error_message: "Failed to write to USB device endpoint.",
            }),
            x => Ok(x as _),
        }
    }

    pub fn bulk_transfer_in(
        &self,
        env: &mut JNIEnv,
        endpoint: &UsbEndpoint,
        buffer: &mut Vec<u8>,
        timeout: i32,
    ) -> Result<usize, DeviceError> {
        let byte_array = env.new_global_ref(env.byte_array_from_slice(buffer)?)?;

        let result = utils::call_method(
            env,
            self.instance.as_obj(),
            "bulkTransfer",
            "(Landroid/hardware/usb/UsbEndpoint;[BIII)I",
            &[
                JValue::from(endpoint.deref()),
                JValue::from(byte_array.as_obj()),
                JValue::from(0 as jint),            // Offset
                JValue::from(buffer.len() as jint), // Length
                JValue::from(timeout),
            ],
        )?
        .i()?;

        let byte_array_ref: &JByteArray = byte_array.as_obj().into();

        *buffer = env.convert_byte_array(byte_array_ref)?;

        match result {
            x if x < 0 => Err(DeviceError::IOError {
                error_message: "Failed to read from USB device endpoint.",
            }),
            x => Ok(x as _),
        }
    }

    pub fn release_interface(&self, env: &mut JNIEnv, usb_interface: &UsbInterface) -> Result<bool, DeviceError> {
        let result = utils::call_method(
            env,
            self.instance.as_obj(),
            "releaseInterface",
            "(Landroid/hardware/usb/UsbInterface)Z",
            &[usb_interface.deref().into()],
        )?
        .z()?;

        Ok(result)
    }

    pub fn close(&self, env: &mut JNIEnv) -> Result<(), DeviceError> {
        let result = utils::call_method(env, self.instance.as_obj(), "close", "(V)V", &[])?.v()?;

        Ok(result)
    }
}
