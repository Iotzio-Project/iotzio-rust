use crate::communication::FatalError;
use thiserror_no_std::Error;

#[non_exhaustive]
#[cfg_attr(any(feature = "_uniffi-blocking", feature = "_uniffi-async"), derive(uniffi::Error))]
#[derive(Error, Debug)]
pub enum InitializationError {
    #[error("Iotzio board is already opened.")]
    DeviceAlreadyInUseError,
    #[error("{error_message}")]
    DeviceOpenError { error_message: String },
    #[error("Mismatching Iotzio protocol version: Driver version is {driver}, but Iotzio board has version {board}.")]
    MismatchingProtocolVersion { driver: u16, board: u16 },
    #[error("{error}")]
    FatalErrorWrapper { error: FatalError },
}

impl From<FatalError> for InitializationError {
    fn from(value: FatalError) -> Self {
        InitializationError::FatalErrorWrapper { error: value }
    }
}
