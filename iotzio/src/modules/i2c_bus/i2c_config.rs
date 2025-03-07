use crate::modules::i2c_bus::{I2c0SclPin, I2c0SdaPin, I2c1SclPin, I2c1SdaPin};
use serde::{Deserialize, Serialize};

cfg_if::cfg_if! {
    if #[cfg(feature = "_std")] {
        use std::fmt;
    }
    else {
        use core::fmt;
    }
}

#[cfg_attr(any(feature = "_ffi-blocking", feature = "_ffi-async"), derive(uniffi::Enum))]
#[cfg_attr(feature = "_defmt", derive(defmt::Format))]
#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum I2cConfig {
    I2c0 {
        scl: I2c0SclPin,
        sda: I2c0SdaPin,
        requested_frequency_hz: Option<u32>,
    },
    I2c1 {
        scl: I2c1SclPin,
        sda: I2c1SdaPin,
        requested_frequency_hz: Option<u32>,
    },
}

impl fmt::Display for I2cConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
