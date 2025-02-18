use crate::peripherals::gpio::GpioPin;
use derive_more::Display;
use serde::{Deserialize, Serialize};

/// Number of a pin suitable for SCL using I2C1.
#[cfg_attr(any(feature = "_uniffi-blocking", feature = "_uniffi-async"), derive(uniffi::Enum))]
#[cfg_attr(feature = "_defmt", derive(defmt::Format))]
#[derive(Serialize, Deserialize, Display, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum I2c1SclPin {
    Pin3,
    Pin7,
    Pin11,
    Pin15,
    Pin19,
    Pin27,
}

impl TryFrom<u8> for I2c1SclPin {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            3 => Ok(I2c1SclPin::Pin3),
            7 => Ok(I2c1SclPin::Pin7),
            11 => Ok(I2c1SclPin::Pin11),
            15 => Ok(I2c1SclPin::Pin15),
            19 => Ok(I2c1SclPin::Pin19),
            27 => Ok(I2c1SclPin::Pin27),
            _ => Err(()),
        }
    }
}

impl From<I2c1SclPin> for u8 {
    fn from(pin: I2c1SclPin) -> u8 {
        match pin {
            I2c1SclPin::Pin3 => 3,
            I2c1SclPin::Pin7 => 7,
            I2c1SclPin::Pin11 => 11,
            I2c1SclPin::Pin15 => 15,
            I2c1SclPin::Pin19 => 19,
            I2c1SclPin::Pin27 => 27,
        }
    }
}

impl Into<GpioPin> for I2c1SclPin {
    fn into(self) -> GpioPin {
        match self {
            I2c1SclPin::Pin3 => GpioPin::Pin3,
            I2c1SclPin::Pin7 => GpioPin::Pin7,
            I2c1SclPin::Pin11 => GpioPin::Pin11,
            I2c1SclPin::Pin15 => GpioPin::Pin15,
            I2c1SclPin::Pin19 => GpioPin::Pin19,
            I2c1SclPin::Pin27 => GpioPin::Pin27,
        }
    }
}
