use crate::communication::FatalError;
use crate::modules::ModuleError;
use serde::{Deserialize, Serialize};
use thiserror_no_std::Error;

/// Output pin module error.
#[non_exhaustive]
#[cfg_attr(feature = "_defmt", derive(defmt::Format))]
#[cfg_attr(any(feature = "_ffi-blocking", feature = "_ffi-async"), derive(uniffi::Error))]
#[derive(Serialize, Deserialize, Error, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum OutputPinModuleError {
    /// Module error.
    #[error("{error}")]
    ModuleErrorWrapper { error: ModuleError },

    /// Fatal error.
    #[error("{error}")]
    FatalErrorWrapper { error: FatalError },
}

impl From<ModuleError> for OutputPinModuleError {
    fn from(value: ModuleError) -> Self {
        OutputPinModuleError::ModuleErrorWrapper { error: value }
    }
}

impl From<FatalError> for OutputPinModuleError {
    fn from(value: FatalError) -> Self {
        OutputPinModuleError::FatalErrorWrapper { error: value }
    }
}

#[cfg(feature = "embedded-hal")]
impl embedded_hal::digital::Error for OutputPinModuleError {
    fn kind(&self) -> embedded_hal::digital::ErrorKind {
        embedded_hal::digital::ErrorKind::Other
    }
}
