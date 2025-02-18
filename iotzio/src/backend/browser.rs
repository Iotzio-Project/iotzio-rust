#![forbid(unsafe_code)]

use crate::communication::Version;
use crate::iotzio::{USB_PRODUCT_ID, USB_PRODUCT_NAME_PREFIX, USB_USAGE_ID, USB_USAGE_PAGE, USB_VENDOR_ID};
use crate::IotzioInfo;
use async_std::channel::{unbounded, Receiver};
use async_std::task::block_on;
use js_sys::wasm_bindgen::prelude::wasm_bindgen;
use js_sys::wasm_bindgen::JsValue;
use js_sys::Promise;
use serde::{Deserialize, Serialize};
use std::any::type_name;
use std::fmt::Debug;
use std::format;
use std::hash::Hash;
use std::hash::{DefaultHasher, Hasher};
use std::sync::Arc;
use wasm_bindgen::closure::Closure;
use wasm_bindgen_futures::JsFuture;
use web_sys::wasm_bindgen::JsCast;
use web_sys::{window, Hid, HidDevice, HidDeviceRequestOptions, HidInputReportEvent};

#[derive(Serialize, Deserialize, Debug)]
struct HidDeviceRequest {
    #[serde(rename = "vendorId")]
    pub vendor_id: Option<u16>,
    #[serde(rename = "productId")]
    pub product_id: Option<u16>,
    #[serde(rename = "usagePage")]
    pub usage_page: Option<u16>,
    #[serde(rename = "usage")]
    pub usage_id: Option<u16>,
}

pub async fn list_connected_boards(parse_version: fn(&str) -> Option<Version>) -> Result<Vec<IotzioInfo>, DeviceError> {
    let api = get_web_hid_api()?;

    let js_options = serde_wasm_bindgen::to_value(&[HidDeviceRequest {
        vendor_id: Some(USB_VENDOR_ID),
        product_id: Some(USB_PRODUCT_ID),
        usage_page: Some(USB_USAGE_PAGE),
        usage_id: Some(USB_USAGE_ID),
    }])
    .map_err(|_| "Failed to serialize HidDeviceRequest.")?;

    let options = HidDeviceRequestOptions::new(&js_options);

    let js_devices = promise_to_future(api.request_device(&options)).await?;

    let devices = cast::<js_sys::Array>(&js_devices)?
        .iter()
        .filter_map(|x| {
            let hid_device = match cast::<HidDevice>(&x) {
                Ok(x) => x,
                Err(_) => return None,
            };

            if !hid_device.product_name().starts_with(USB_PRODUCT_NAME_PREFIX) {
                return None;
            }

            let runtime_identifier = {
                let mut hasher = DefaultHasher::new();

                to_string(&x).hash(&mut hasher);

                hasher.finish()
            };

            parse_version(&hid_device.product_name()).map(|version| IotzioInfo {
                device_info: DeviceInfo { hid_device_object: x }.into(),
                version,
                serial_number: None,
                runtime_identifier,
            })
        })
        .collect();

    Ok(devices)
}

#[derive(Debug)]
pub struct DeviceInfo {
    hid_device_object: JsValue,
}

impl DeviceInfo {
    pub async fn open(&self) -> Result<(DeviceReader, DeviceWriter), DeviceError> {
        is_valid_object(&self.hid_device_object)?;

        let hid_device = cast::<HidDevice>(&self.hid_device_object)?;

        if hid_device.opened() {
            promise_to_future(hid_device.close().into()).await?;
        }

        let input_channel = setup_read_closure(&hid_device);

        promise_to_future(hid_device.open().into()).await?;

        let inner = Arc::new(BackendDevice {
            hid_device_object: self.hid_device_object.clone(),
            input_channel,
        });

        let reader = DeviceReader { inner: inner.clone() };

        let writer = DeviceWriter { inner };

        Ok((reader, writer))
    }
}

#[derive(Debug)]
struct BackendDevice {
    hid_device_object: JsValue,
    input_channel: Receiver<HidInputReportEvent>,
}

impl Drop for BackendDevice {
    fn drop(&mut self) {
        match cast::<HidDevice>(&self.hid_device_object) {
            Ok(x) => x.set_oninputreport(None),
            Err(_) => {}
        };

        let js_hid_device = self.hid_device_object.clone();

        block_on(async move {
            match cast::<HidDevice>(&js_hid_device) {
                Ok(x) => {
                    _ = promise_to_future(x.close()).await;
                }
                Err(_) => {}
            };
        })
    }
}

#[derive(Debug)]
pub struct DeviceReader {
    inner: Arc<BackendDevice>,
}

impl DeviceReader {
    pub async fn read_input_report(&mut self, buf: &mut [u8]) -> Result<usize, DeviceError> {
        if buf.len() == 0 {
            return Err("HID input buffer overflow.".to_string());
        }

        match self.inner.input_channel.recv().await {
            Err(_) => Err("Input channel closed.".to_string()),
            Ok(e) => {
                let data_view = e.data();

                buf[0] = e.report_id();

                let report_count = data_view.byte_length();
                let report_offset = data_view.byte_offset();

                if report_count == 0 {
                    return Ok(1);
                }

                let report_buffer = &mut buf[1..];

                if report_count > report_buffer.len() {
                    return Err("HID input buffer overflow.".to_string());
                }

                for (buffer, index) in report_buffer[..report_count].iter_mut().zip(0..report_count) {
                    *buffer = data_view.get_uint8(index + report_offset);
                }

                Ok(1 + report_count)
            }
        }
    }
}

#[derive(Debug)]
pub struct DeviceWriter {
    inner: Arc<BackendDevice>,
}

impl DeviceWriter {
    pub async fn write_output_report(&mut self, buf: &mut [u8]) -> Result<(), DeviceError> {
        let hid_device = cast::<HidDevice>(&self.inner.hid_device_object)?;

        let js_promise = hid_device
            .send_report_with_u8_slice(buf[0], &mut buf[1..])
            .map_err(|x| to_string(&x))?;

        promise_to_future(js_promise).await?;

        Ok(())
    }
}

pub type DeviceError = String;

fn get_web_hid_api() -> Result<Hid, String> {
    let window = window().ok_or("Failed to get Window object.".to_string())?;

    let hid_api = window.navigator().hid();

    match hid_api.is_null() || hid_api.is_undefined() {
        true => Err("WebHID is not supported by this environment.".to_string()),
        false => Ok(hid_api),
    }
}

#[inline]
fn setup_read_closure(hid_device: &HidDevice) -> Receiver<HidInputReportEvent> {
    let (tx, rx) = unbounded::<HidInputReportEvent>();

    let closure = Closure::wrap(Box::new(move |e: HidInputReportEvent| {
        _ = tx.send_blocking(e);
    }) as Box<dyn FnMut(HidInputReportEvent)>);

    hid_device.set_oninputreport(Some(closure.as_ref().unchecked_ref()));

    closure.forget();

    rx
}

#[inline]
fn cast<T: JsCast>(value: &JsValue) -> Result<&T, String> {
    value.dyn_ref::<T>().ok_or(format!(
        "Failed to cast JavaScript object to type {0}.",
        type_name::<T>()
    ))
}

#[inline]
fn is_valid_object(value: &JsValue) -> Result<(), String> {
    if value.is_null() {
        return Err("JavaScript object is null.".to_string());
    }

    if value.is_undefined() {
        return Err("JavaScript object is undefined.".to_string());
    }

    Ok(())
}

#[inline]
async fn promise_to_future(promise: Promise) -> Result<JsValue, String> {
    JsFuture::from(promise)
        .await
        .map_err(|x| format!("Failed to await JavaScript promise: {0}.", to_string(&x)))
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = String)]
    pub fn to_string(value: &JsValue) -> String;
}
