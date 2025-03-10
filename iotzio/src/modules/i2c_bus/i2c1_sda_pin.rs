use crate::peripherals::gpio::GpioPin;
use derive_more::Display;
use serde::{Deserialize, Serialize};

/// Number of a pin suitable for SDA using I2C1.
#[cfg_attr(any(feature = "_ffi-blocking", feature = "_ffi-async"), derive(uniffi::Enum))]
#[cfg_attr(feature = "_defmt", derive(defmt::Format))]
#[derive(Serialize, Deserialize, Display, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum I2c1SdaPin {
    /// Pin 2
    Pin2,
    /// Pin 6
    Pin6,
    /// Pin 10
    Pin10,
    /// Pin 14
    Pin14,
    /// Pin 18
    Pin18,
    /// Pin 26
    Pin26,
}

impl TryFrom<u8> for I2c1SdaPin {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            2 => Ok(I2c1SdaPin::Pin2),
            6 => Ok(I2c1SdaPin::Pin6),
            10 => Ok(I2c1SdaPin::Pin10),
            14 => Ok(I2c1SdaPin::Pin14),
            18 => Ok(I2c1SdaPin::Pin18),
            26 => Ok(I2c1SdaPin::Pin26),
            _ => Err(()),
        }
    }
}

impl From<I2c1SdaPin> for u8 {
    fn from(pin: I2c1SdaPin) -> u8 {
        match pin {
            I2c1SdaPin::Pin2 => 2,
            I2c1SdaPin::Pin6 => 6,
            I2c1SdaPin::Pin10 => 10,
            I2c1SdaPin::Pin14 => 14,
            I2c1SdaPin::Pin18 => 18,
            I2c1SdaPin::Pin26 => 26,
        }
    }
}

impl Into<GpioPin> for I2c1SdaPin {
    fn into(self) -> GpioPin {
        match self {
            I2c1SdaPin::Pin2 => GpioPin::Pin2,
            I2c1SdaPin::Pin6 => GpioPin::Pin6,
            I2c1SdaPin::Pin10 => GpioPin::Pin10,
            I2c1SdaPin::Pin14 => GpioPin::Pin14,
            I2c1SdaPin::Pin18 => GpioPin::Pin18,
            I2c1SdaPin::Pin26 => GpioPin::Pin26,
        }
    }
}
