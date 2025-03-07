use crate::communication::ProtocolError;
use serde::{Deserialize, Serialize};
use thiserror_no_std::Error;

cfg_if::cfg_if! {
    if #[cfg(feature = "_std")] {
        pub type MessageString = String;
    }
    else {
        pub type MessageString = heapless::String<50>;
    }
}

#[non_exhaustive]
#[cfg_attr(any(feature = "_ffi-blocking", feature = "_ffi-async"), derive(uniffi::Error))]
#[cfg_attr(feature = "_defmt", derive(defmt::Format))]
#[derive(Serialize, Deserialize, Error, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum FatalError {
    #[error("{error_message}")]
    HostWriteError { error_message: MessageString },
    #[error("{error_message}")]
    HostReadError { error_message: MessageString },
    #[error("{error}")]
    HostProtocolError { error: ProtocolError },
    #[error("{error_message}")]
    DeviceWriteError { error_message: MessageString },
    #[error("{error_message}")]
    DeviceReadError { error_message: MessageString },
    #[error("{error}")]
    DeviceProtocolError { error: ProtocolError },
    #[error("Socket is already closed.")]
    DeviceClosed,
}

#[cfg(feature = "_std")]
impl From<ProtocolError> for FatalError {
    fn from(value: ProtocolError) -> Self {
        FatalError::HostProtocolError { error: value }
    }
}

#[cfg(feature = "_std")]
impl From<postcard::Error> for FatalError {
    fn from(value: postcard::Error) -> Self {
        FatalError::HostProtocolError {
            error: ProtocolError::from(value),
        }
    }
}

#[cfg(feature = "_std")]
impl FatalError {
    #[inline]
    pub fn read_error(error_message: MessageString) -> FatalError {
        FatalError::HostReadError { error_message }
    }

    #[inline]
    pub fn write_error(error_message: MessageString) -> FatalError {
        FatalError::HostWriteError { error_message }
    }
}

#[cfg(not(feature = "_std"))]
impl From<ProtocolError> for FatalError {
    fn from(value: ProtocolError) -> Self {
        FatalError::DeviceProtocolError { error: value }
    }
}

#[cfg(not(feature = "_std"))]
impl From<postcard::Error> for FatalError {
    fn from(value: postcard::Error) -> Self {
        FatalError::DeviceProtocolError {
            error: ProtocolError::from(value),
        }
    }
}

#[cfg(not(feature = "_std"))]
impl FatalError {
    #[inline]
    pub fn read_error(error_message: MessageString) -> FatalError {
        FatalError::DeviceReadError { error_message }
    }

    #[inline]
    pub fn write_error(error_message: MessageString) -> FatalError {
        FatalError::DeviceWriteError { error_message }
    }
}
