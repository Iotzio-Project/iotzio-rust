use crate::communication::FatalError;
use crate::modules::ModuleError;
use serde::{Deserialize, Serialize};
use thiserror_no_std::Error;

#[non_exhaustive]
#[cfg_attr(feature = "_defmt", derive(defmt::Format))]
#[cfg_attr(any(feature = "_ffi-blocking", feature = "_ffi-async"), derive(uniffi::Error))]
#[derive(Serialize, Deserialize, Error, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum InputPinModuleError {
    #[error("{error}")]
    ModuleErrorWrapper { error: ModuleError },
    #[error("{error}")]
    FatalErrorWrapper { error: FatalError },
}

impl From<ModuleError> for InputPinModuleError {
    fn from(value: ModuleError) -> Self {
        InputPinModuleError::ModuleErrorWrapper { error: value }
    }
}

impl From<FatalError> for InputPinModuleError {
    fn from(value: FatalError) -> Self {
        InputPinModuleError::FatalErrorWrapper { error: value }
    }
}

#[cfg(feature = "embedded-hal")]
impl embedded_hal::digital::Error for InputPinModuleError {
    fn kind(&self) -> embedded_hal::digital::ErrorKind {
        embedded_hal::digital::ErrorKind::Other
    }
}
