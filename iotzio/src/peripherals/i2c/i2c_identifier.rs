use derive_more::Display;
use serde::{Deserialize, Serialize};

/// Represents an I2C bus.
#[cfg_attr(any(feature = "_ffi-blocking", feature = "_ffi-async"), derive(uniffi::Enum))]
#[cfg_attr(feature = "_defmt", derive(defmt::Format))]
#[derive(Serialize, Deserialize, Display, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum I2cIdentifier {
    /// I2C Bus 0
    I2c0,
    /// I2C Bus 1
    I2c1,
}

impl TryFrom<u8> for I2cIdentifier {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(I2cIdentifier::I2c0),
            1 => Ok(I2cIdentifier::I2c1),
            _ => Err(()),
        }
    }
}

impl Into<u8> for I2cIdentifier {
    fn into(self) -> u8 {
        match self {
            I2cIdentifier::I2c0 => 0,
            I2cIdentifier::I2c1 => 1,
        }
    }
}
