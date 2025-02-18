#![forbid(unsafe_code)]

use crate::communication::Version;
use crate::iotzio::{USB_PRODUCT_ID, USB_PRODUCT_NAME_PREFIX, USB_USAGE_ID, USB_USAGE_PAGE, USB_VENDOR_ID};
use crate::IotzioInfo;
use async_hid::SerialNumberExt;
use futures_lite::StreamExt;
use std::fmt::{Debug, Formatter};
use std::hash::{DefaultHasher, Hash, Hasher};
use std::sync::Arc;

pub async fn list_connected_boards(parse_version: fn(&str) -> Option<Version>) -> Result<Vec<IotzioInfo>, DeviceError> {
    let all_devices = async_hid::DeviceInfo::enumerate().await?;

    let vec = all_devices
        .filter_map(|x| {
            match x.vendor_id == USB_VENDOR_ID
                && x.product_id == USB_PRODUCT_ID
                && x.usage_page == USB_USAGE_PAGE
                && x.usage_id == USB_USAGE_ID
                && x.name.starts_with(USB_PRODUCT_NAME_PREFIX)
            {
                true => {
                    let serial_number = x.serial_number().map(|x| x.to_string());

                    let runtime_identifier = {
                        let mut hasher = DefaultHasher::new();

                        x.id.hash(&mut hasher);

                        hasher.finish()
                    };

                    parse_version(&x.name).map(|version| IotzioInfo {
                        device_info: DeviceInfo { inner: x }.into(),
                        version,
                        serial_number,
                        runtime_identifier,
                    })
                }
                false => None,
            }
        })
        .collect::<Vec<_>>()
        .await;

    Ok(vec)
}

#[derive(Debug)]
pub struct DeviceInfo {
    inner: async_hid::DeviceInfo,
}

impl DeviceInfo {
    pub async fn open(&self) -> Result<(DeviceReader, DeviceWriter), DeviceError> {
        let inner = Arc::new(self.inner.open(async_hid::AccessMode::ReadWrite).await?);

        Ok((DeviceReader { inner: inner.clone() }, DeviceWriter { inner }))
    }
}

pub struct DeviceReader {
    inner: Arc<async_hid::Device>,
}

impl Debug for DeviceReader {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("inner: Not implemented.")
    }
}

impl DeviceReader {
    pub async fn read_input_report(&mut self, buffer: &mut [u8]) -> Result<usize, DeviceError> {
        self.inner.read_input_report(buffer).await
    }
}

pub struct DeviceWriter {
    inner: Arc<async_hid::Device>,
}

impl Debug for DeviceWriter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("inner: Not implemented.")
    }
}

impl DeviceWriter {
    pub async fn write_output_report(&mut self, buffer: &mut [u8]) -> Result<(), DeviceError> {
        self.inner.write_output_report(buffer).await
    }
}

pub type DeviceError = async_hid::HidError;
