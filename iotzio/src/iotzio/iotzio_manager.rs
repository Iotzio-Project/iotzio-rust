use crate::iotzio::iotzio_info::IotzioInfo;
use crate::iotzio::iotzio_service;
use crate::InitializationError;
use std::marker::PhantomData;

#[cfg_attr(any(feature = "_ffi-blocking", feature = "_ffi-async"), derive(uniffi::Object))]
#[derive(Debug, Default)]
pub struct IotzioManager {
    phantom_data: PhantomData<()>,
}

#[cfg_attr(any(feature = "_ffi-blocking", feature = "_ffi-async"), uniffi::export)]
impl IotzioManager {
    #[inline]
    #[cfg_attr(any(feature = "_ffi-blocking", feature = "_ffi-async"), uniffi::constructor)]
    pub fn new() -> IotzioManager {
        IotzioManager {
            phantom_data: PhantomData,
        }
    }
}

#[cfg(all(not(target_family = "wasm"), not(feature = "_ffi-blocking")))]
impl IotzioManager {
    #[inline]
    pub fn list_connected_boards(&self) -> Result<Vec<IotzioInfo>, InitializationError> {
        async_std::task::block_on(iotzio_service::list_connected_boards())
    }
}

#[cfg(all(not(target_family = "wasm"), feature = "_ffi-blocking"))]
#[uniffi::export]
impl IotzioManager {
    #[inline]
    pub fn list_connected_boards(&self) -> Result<Vec<std::sync::Arc<IotzioInfo>>, InitializationError> {
        async_std::task::block_on(iotzio_service::list_connected_boards())
            .map(|x| x.into_iter().map(std::sync::Arc::new).collect())
    }
}

#[cfg(not(feature = "_ffi-async"))]
impl IotzioManager {
    #[inline]
    pub async fn list_connected_boards_async(&self) -> Result<Vec<IotzioInfo>, InitializationError> {
        iotzio_service::list_connected_boards().await
    }
}

#[cfg(feature = "_ffi-async")]
#[uniffi::export]
impl IotzioManager {
    #[inline]
    pub async fn list_connected_boards_async(&self) -> Result<Vec<std::sync::Arc<IotzioInfo>>, InitializationError> {
        iotzio_service::list_connected_boards()
            .await
            .map(|x| x.into_iter().map(std::sync::Arc::new).collect())
    }
}
