use crate::communication::ProtocolError;
use serde::{Deserialize, Serialize};
use thiserror_no_std::Error;

cfg_if::cfg_if! {
    if #[cfg(feature = "_host")] {
        pub type MessageString = String;
    }
    else {
        pub type MessageString = heapless::String<50>;
    }
}

/// A fatal error means that communication with the Iotzio board can no longer proceed. Typically, this occurs when the physical USB connection has been lost. IotzioInfo must be opened again for further interaction.
#[non_exhaustive]
#[cfg_attr(any(feature = "_ffi-blocking", feature = "_ffi-async"), derive(uniffi::Error))]
#[cfg_attr(feature = "_defmt", derive(defmt::Format))]
#[derive(Serialize, Deserialize, Error, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum FatalError {
    /// Host encountered an error writing data.
    #[error("{error_message}")]
    HostWriteError { error_message: MessageString },

    /// Host encountered an error reading data.
    #[error("{error_message}")]
    HostReadError { error_message: MessageString },

    /// Host encountered a protocol error.
    #[error("{error}")]
    HostProtocolError { error: ProtocolError },

    /// Device encountered an error writing data.
    #[error("{error_message}")]
    DeviceWriteError { error_message: MessageString },

    /// Device encountered an error reading data.
    #[error("{error_message}")]
    DeviceReadError { error_message: MessageString },

    /// Device encountered a protocol error.
    #[error("{error}")]
    DeviceProtocolError { error: ProtocolError },

    /// Device is already closed.
    #[error("Device is already closed.")]
    DeviceClosed,
}

#[cfg(feature = "_host")]
impl From<ProtocolError> for FatalError {
    fn from(value: ProtocolError) -> Self {
        FatalError::HostProtocolError { error: value }
    }
}

#[cfg(feature = "_host")]
impl From<postcard::Error> for FatalError {
    fn from(value: postcard::Error) -> Self {
        FatalError::HostProtocolError {
            error: ProtocolError::from(value),
        }
    }
}

#[cfg(feature = "_host")]
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

#[cfg(not(feature = "_host"))]
impl From<ProtocolError> for FatalError {
    fn from(value: ProtocolError) -> Self {
        FatalError::DeviceProtocolError { error: value }
    }
}

#[cfg(not(feature = "_host"))]
impl From<postcard::Error> for FatalError {
    fn from(value: postcard::Error) -> Self {
        FatalError::DeviceProtocolError {
            error: ProtocolError::from(value),
        }
    }
}

#[cfg(not(feature = "_host"))]
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
