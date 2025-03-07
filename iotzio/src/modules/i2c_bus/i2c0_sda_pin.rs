use crate::peripherals::gpio::GpioPin;
use derive_more::Display;
use serde::{Deserialize, Serialize};

/// Number of a pin suitable for SDA using I2C0.
#[cfg_attr(any(feature = "_ffi-blocking", feature = "_ffi-async"), derive(uniffi::Enum))]
#[cfg_attr(feature = "_defmt", derive(defmt::Format))]
#[derive(Serialize, Deserialize, Display, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum I2c0SdaPin {
    Pin0,
    Pin4,
    Pin8,
    Pin12,
    Pin16,
    Pin20,
}

impl TryFrom<u8> for I2c0SdaPin {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(I2c0SdaPin::Pin0),
            4 => Ok(I2c0SdaPin::Pin4),
            8 => Ok(I2c0SdaPin::Pin8),
            12 => Ok(I2c0SdaPin::Pin12),
            16 => Ok(I2c0SdaPin::Pin16),
            20 => Ok(I2c0SdaPin::Pin20),
            _ => Err(()),
        }
    }
}

impl From<I2c0SdaPin> for u8 {
    fn from(pin: I2c0SdaPin) -> u8 {
        match pin {
            I2c0SdaPin::Pin0 => 0,
            I2c0SdaPin::Pin4 => 4,
            I2c0SdaPin::Pin8 => 8,
            I2c0SdaPin::Pin12 => 12,
            I2c0SdaPin::Pin16 => 16,
            I2c0SdaPin::Pin20 => 20,
        }
    }
}

impl Into<GpioPin> for I2c0SdaPin {
    fn into(self) -> GpioPin {
        match self {
            I2c0SdaPin::Pin0 => GpioPin::Pin0,
            I2c0SdaPin::Pin4 => GpioPin::Pin4,
            I2c0SdaPin::Pin8 => GpioPin::Pin8,
            I2c0SdaPin::Pin12 => GpioPin::Pin12,
            I2c0SdaPin::Pin16 => GpioPin::Pin16,
            I2c0SdaPin::Pin20 => GpioPin::Pin20,
        }
    }
}
