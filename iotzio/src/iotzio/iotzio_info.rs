use crate::backend::DeviceInfo;
use crate::communication::Version;
use crate::iotzio::iotzio_service;
use crate::{InitializationError, Iotzio};

/// Iotzio info represents an Iotzio board that is connected to the host, but not opened.
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
    /// The persistent and unique serial number of the Iotzio device. It may be not available in this unopened device state when targeting Android or Webassembly.
    #[inline]
    pub fn serial_number(&self) -> Option<String> {
        self.serial_number.clone()
    }

    /// The semantic version of the Iotzio device.
    #[inline]
    pub fn version(&self) -> Version {
        self.version
    }

    /// The runtime identifier of the Iotzio device. As long as the device remains connected, the runtime identifier stays consistent. However, it may potentially change after the physical USB connection is reestablished.
    #[inline]
    pub fn runtime_identifier(&self) -> u64 {
        self.runtime_identifier
    }
}

#[cfg(not(target_family = "wasm"))]
#[cfg_attr(feature = "_ffi-blocking", uniffi::export)]
impl IotzioInfo {
    /// Opens the Iotzio device.
    #[inline]
    pub fn open(&self) -> Result<Iotzio, InitializationError> {
        async_std::task::block_on(iotzio_service::new_iotzio(self))
    }
}

#[cfg_attr(feature = "_ffi-async", uniffi::export)]
impl IotzioInfo {
    /// Opens the Iotzio device.
    #[inline]
    pub async fn open_async(&self) -> Result<Iotzio, InitializationError> {
        iotzio_service::new_iotzio(self).await
    }
}
