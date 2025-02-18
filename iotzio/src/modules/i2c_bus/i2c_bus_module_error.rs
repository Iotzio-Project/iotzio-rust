use crate::communication::FatalError;
use crate::modules::ModuleError;
use crate::peripherals::i2c::I2cError;
use serde::{Deserialize, Serialize};
use thiserror_no_std::Error;

/// I2C module error
#[non_exhaustive]
#[cfg_attr(feature = "_defmt", derive(defmt::Format))]
#[cfg_attr(any(feature = "_uniffi-blocking", feature = "_uniffi-async"), derive(uniffi::Error))]
#[derive(Serialize, Deserialize, Error, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum I2cBusModuleError {
    #[error("Requested I2c bus frequency is too high.")]
    FrequencyTooHigh,
    #[error("Requested I2c bus frequency is too low.")]
    FrequencyTooLow,
    #[error("{error}")]
    I2cBusErrorWrapper { error: I2cError },
    #[error("{error}")]
    ModuleErrorWrapper { error: ModuleError },
    #[error("{error}")]
    FatalErrorWrapper { error: FatalError },
}

impl From<I2cError> for I2cBusModuleError {
    fn from(value: I2cError) -> Self {
        I2cBusModuleError::I2cBusErrorWrapper { error: value }
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
            I2cBusModuleError::I2cBusErrorWrapper {
                error: I2cError::AbortNoAcknowledge,
            } => embedded_hal::i2c::ErrorKind::NoAcknowledge(embedded_hal::i2c::NoAcknowledgeSource::Unknown),
            I2cBusModuleError::I2cBusErrorWrapper {
                error: I2cError::AbortArbitrationLoss,
            } => embedded_hal::i2c::ErrorKind::ArbitrationLoss,
            _ => embedded_hal::i2c::ErrorKind::Other,
        }
    }
}
