use crate::backend::DeviceInfo;
use crate::communication::Version;
use crate::iotzio::iotzio_service;
use crate::{InitializationError, Iotzio};

#[cfg_attr(any(feature = "_ffi-blocking", feature = "_ffi-async"), derive(uniffi::Object))]
#[derive(Debug)]
pub struct IotzioInfo {
    pub(crate) device_info: Box<DeviceInfo>,
    pub(crate) version: Version,
    pub(crate) serial_number: Option<String>,
    pub(crate) runtime_identifier: u64,
}

#[cfg_attr(any(feature = "_ffi-blocking", feature = "_ffi-async"), uniffi::export)]
impl IotzioInfo {
    #[inline]
    pub fn serial_number(&self) -> Option<String> {
        self.serial_number.clone()
    }

    #[inline]
    pub fn version(&self) -> Version {
        self.version
    }

    #[inline]
    pub fn runtime_identifier(&self) -> u64 {
        self.runtime_identifier
    }
}

#[cfg(not(target_family = "wasm"))]
#[cfg_attr(feature = "_ffi-blocking", uniffi::export)]
impl IotzioInfo {
    #[inline]
    pub fn open(&self) -> Result<Iotzio, InitializationError> {
        async_std::task::block_on(iotzio_service::new_iotzio(self))
    }
}

#[cfg_attr(feature = "_ffi-async", uniffi::export)]
impl IotzioInfo {
    #[inline]
    pub async fn open_async(&self) -> Result<Iotzio, InitializationError> {
        iotzio_service::new_iotzio(self).await
    }
}
