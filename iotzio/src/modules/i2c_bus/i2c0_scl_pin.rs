use crate::peripherals::gpio::GpioPin;
use derive_more::Display;
use serde::{Deserialize, Serialize};

/// Number of a pin suitable for SCL using I2C0.
#[cfg_attr(any(feature = "_uniffi-blocking", feature = "_uniffi-async"), derive(uniffi::Enum))]
#[cfg_attr(feature = "_defmt", derive(defmt::Format))]
#[derive(Serialize, Deserialize, Display, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum I2c0SclPin {
    Pin1,
    Pin5,
    Pin9,
    Pin13,
    Pin17,
    Pin21,
}

impl TryFrom<u8> for I2c0SclPin {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(I2c0SclPin::Pin1),
            5 => Ok(I2c0SclPin::Pin5),
            9 => Ok(I2c0SclPin::Pin9),
            13 => Ok(I2c0SclPin::Pin13),
            17 => Ok(I2c0SclPin::Pin17),
            21 => Ok(I2c0SclPin::Pin21),
            _ => Err(()),
        }
    }
}

impl From<I2c0SclPin> for u8 {
    fn from(pin: I2c0SclPin) -> u8 {
        match pin {
            I2c0SclPin::Pin1 => 1,
            I2c0SclPin::Pin5 => 5,
            I2c0SclPin::Pin9 => 9,
            I2c0SclPin::Pin13 => 13,
            I2c0SclPin::Pin17 => 17,
            I2c0SclPin::Pin21 => 21,
        }
    }
}

impl Into<GpioPin> for I2c0SclPin {
    fn into(self) -> GpioPin {
        match self {
            I2c0SclPin::Pin1 => GpioPin::Pin1,
            I2c0SclPin::Pin5 => GpioPin::Pin5,
            I2c0SclPin::Pin9 => GpioPin::Pin9,
            I2c0SclPin::Pin13 => GpioPin::Pin13,
            I2c0SclPin::Pin17 => GpioPin::Pin17,
            I2c0SclPin::Pin21 => GpioPin::Pin21,
        }
    }
}
