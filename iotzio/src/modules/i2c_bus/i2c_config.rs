use crate::modules::i2c_bus::{I2c0SclPin, I2c0SdaPin, I2c1SclPin, I2c1SdaPin};
use serde::{Deserialize, Serialize};

cfg_if::cfg_if! {
    if #[cfg(feature = "_host")] {
        use std::fmt;
    }
    else {
        use core::fmt;
    }
}

/// I2C bus configuration.
#[cfg_attr(any(feature = "_ffi-blocking", feature = "_ffi-async"), derive(uniffi::Enum))]
#[cfg_attr(feature = "_defmt", derive(defmt::Format))]
#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum I2cConfig {
    /// Use I2C bus 0
    I2c0 {
        /// SCL pin to use
        scl: I2c0SclPin,
        /// SDA pin to use
        sda: I2c0SdaPin,
        /// Frequency to want to use in Hertz. If no value is passed, 100 kHz is used.
        requested_frequency_hz: Option<u32>,
    },
    /// Use I2C bus 1
    I2c1 {
        /// SCL pin to use
        scl: I2c1SclPin,
        /// SDA pin to use
        sda: I2c1SdaPin,
        /// Frequency to want to use in Hertz. If no value is passed, 100 kHz is used.
        requested_frequency_hz: Option<u32>,
    },
}

impl fmt::Display for I2cConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
