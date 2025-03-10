use derive_more::Display;
use serde::{Deserialize, Serialize};

/// I2C bus number.
#[cfg_attr(any(feature = "_ffi-blocking", feature = "_ffi-async"), derive(uniffi::Enum))]
#[cfg_attr(feature = "_defmt", derive(defmt::Format))]
#[derive(Serialize, Deserialize, Display, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum I2cBusNumber {
    /// I2C Bus 0
    I2c0,
    /// I2C Bus 1
    I2c1,
}

impl TryFrom<u8> for I2cBusNumber {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(I2cBusNumber::I2c0),
            1 => Ok(I2cBusNumber::I2c1),
            _ => Err(()),
        }
    }
}

impl Into<u8> for I2cBusNumber {
    fn into(self) -> u8 {
        match self {
            I2cBusNumber::I2c0 => 0,
            I2cBusNumber::I2c1 => 1,
        }
    }
}
