use crate::communication::FatalError;
use thiserror_no_std::Error;

/// Error that can occur while opening an Iotzio device.
#[non_exhaustive]
#[cfg_attr(any(feature = "_ffi-blocking", feature = "_ffi-async"), derive(uniffi::Error))]
#[derive(Error, Debug)]
pub enum InitializationError {
    /// Iotzio board is already opened.
    #[error("Iotzio board is already opened.")]
    DeviceAlreadyInUseError,

    /// Failed to open the device. Maybe it was unplugged or there are insufficient permissions to open it.
    #[error("{error_message}")]
    DeviceOpenError { error_message: String },

    /// Mismatching Iotzio protocol version: Library version differs from Iotzio board version.
    #[error("Mismatching Iotzio protocol version: Library version is {driver}, but Iotzio board has version {board}.")]
    MismatchingProtocolVersion { driver: u16, board: u16 },

    /// A fatal error occurred.
    #[error("{error}")]
    FatalErrorWrapper { error: FatalError },
}

impl From<FatalError> for InitializationError {
    fn from(value: FatalError) -> Self {
        InitializationError::FatalErrorWrapper { error: value }
    }
}
