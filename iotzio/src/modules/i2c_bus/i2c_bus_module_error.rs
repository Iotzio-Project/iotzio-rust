use crate::communication::FatalError;
use crate::modules::ModuleError;
use crate::peripherals::i2c::I2cError;
use serde::{Deserialize, Serialize};
use thiserror_no_std::Error;

/// I2C bus module error.
#[non_exhaustive]
#[cfg_attr(feature = "_defmt", derive(defmt::Format))]
#[cfg_attr(any(feature = "_ffi-blocking", feature = "_ffi-async"), derive(uniffi::Error))]
#[derive(Serialize, Deserialize, Error, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum I2cBusModuleError {
    /// Requested I2c bus frequency is too high.
    #[error("Requested I2c bus frequency is too high.")]
    FrequencyTooHigh,

    /// Requested I2c bus frequency is too low.
    #[error("Requested I2c bus frequency is too low.")]
    FrequencyTooLow,

    /// I2C error.
    #[error("{error}")]
    I2cErrorWrapper { error: I2cError },

    /// Module error.
    #[error("{error}")]
    ModuleErrorWrapper { error: ModuleError },

    /// Fatal error.
    #[error("{error}")]
    FatalErrorWrapper { error: FatalError },
}

impl From<I2cError> for I2cBusModuleError {
    fn from(value: I2cError) -> Self {
        I2cBusModuleError::I2cErrorWrapper { error: value }
    }
}

impl From<ModuleError> for I2cBusModuleError {
    fn from(value: ModuleError) -> Self {
        I2cBusModuleError::ModuleErrorWrapper { error: value }
    }
}

impl From<FatalError> for I2cBusModuleError {
    fn from(value: FatalError) -> Self {
        I2cBusModuleError::FatalErrorWrapper { error: value }
    }
}

#[cfg(feature = "embedded-hal")]
impl embedded_hal::i2c::Error for I2cBusModuleError {
    fn kind(&self) -> embedded_hal::i2c::ErrorKind {
        match self {
            I2cBusModuleError::I2cErrorWrapper {
                error: I2cError::AbortNoAcknowledge,
            } => embedded_hal::i2c::ErrorKind::NoAcknowledge(embedded_hal::i2c::NoAcknowledgeSource::Unknown),
            I2cBusModuleError::I2cErrorWrapper {
                error: I2cError::AbortArbitrationLoss,
            } => embedded_hal::i2c::ErrorKind::ArbitrationLoss,
            _ => embedded_hal::i2c::ErrorKind::Other,
        }
    }
}
