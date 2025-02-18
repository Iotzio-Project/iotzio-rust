use derive_more::Display;
use serde::{Deserialize, Serialize};

/// Number of a pin suitable for GPIO and PWM.
#[cfg_attr(any(feature = "_uniffi-blocking", feature = "_uniffi-async"), derive(uniffi::Enum))]
#[cfg_attr(feature = "_defmt", derive(defmt::Format))]
#[derive(Serialize, Deserialize, Display, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum GpioPin {
    Pin0,
    Pin1,
    Pin2,
    Pin3,
    Pin4,
    Pin5,
    Pin6,
    Pin7,
    Pin8,
    Pin9,
    Pin10,
    Pin11,
    Pin12,
    Pin13,
    Pin14,
    Pin15,
    Pin16,
    Pin17,
    Pin18,
    Pin19,
    Pin20,
    Pin21,
    Pin22,
    Pin25,
    Pin26,
    Pin27,
    Pin28,
}

impl TryFrom<u8> for GpioPin {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(GpioPin::Pin0),
            1 => Ok(GpioPin::Pin1),
            2 => Ok(GpioPin::Pin2),
            3 => Ok(GpioPin::Pin3),
            4 => Ok(GpioPin::Pin4),
            5 => Ok(GpioPin::Pin5),
            6 => Ok(GpioPin::Pin6),
            7 => Ok(GpioPin::Pin7),
            8 => Ok(GpioPin::Pin8),
            9 => Ok(GpioPin::Pin9),
            10 => Ok(GpioPin::Pin10),
            11 => Ok(GpioPin::Pin11),
            12 => Ok(GpioPin::Pin12),
            13 => Ok(GpioPin::Pin13),
            14 => Ok(GpioPin::Pin14),
            15 => Ok(GpioPin::Pin15),
            16 => Ok(GpioPin::Pin16),
            17 => Ok(GpioPin::Pin17),
            18 => Ok(GpioPin::Pin18),
            19 => Ok(GpioPin::Pin19),
            20 => Ok(GpioPin::Pin20),
            21 => Ok(GpioPin::Pin21),
            22 => Ok(GpioPin::Pin22),
            25 => Ok(GpioPin::Pin25),
            26 => Ok(GpioPin::Pin26),
            27 => Ok(GpioPin::Pin27),
            28 => Ok(GpioPin::Pin28),
            _ => Err(()),
        }
    }
}

impl From<GpioPin> for u8 {
    fn from(pin: GpioPin) -> u8 {
        match pin {
            GpioPin::Pin0 => 0,
            GpioPin::Pin1 => 1,
            GpioPin::Pin2 => 2,
            GpioPin::Pin3 => 3,
            GpioPin::Pin4 => 4,
            GpioPin::Pin5 => 5,
            GpioPin::Pin6 => 6,
            GpioPin::Pin7 => 7,
            GpioPin::Pin8 => 8,
            GpioPin::Pin9 => 9,
            GpioPin::Pin10 => 10,
            GpioPin::Pin11 => 11,
            GpioPin::Pin12 => 12,
            GpioPin::Pin13 => 13,
            GpioPin::Pin14 => 14,
            GpioPin::Pin15 => 15,
            GpioPin::Pin16 => 16,
            GpioPin::Pin17 => 17,
            GpioPin::Pin18 => 18,
            GpioPin::Pin19 => 19,
            GpioPin::Pin20 => 20,
            GpioPin::Pin21 => 21,
            GpioPin::Pin22 => 22,
            GpioPin::Pin25 => 25,
            GpioPin::Pin26 => 26,
            GpioPin::Pin27 => 27,
            GpioPin::Pin28 => 28,
        }
    }
}
