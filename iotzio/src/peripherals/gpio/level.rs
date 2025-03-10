use derive_more::Display;
use serde::{Deserialize, Serialize};

cfg_if::cfg_if! {
    if #[cfg(feature = "_host")] {
        use std::ops::Not;
    }
    else {
        use core::ops::Not;
    }
}

/// Represents a digital input or output level.
#[cfg_attr(any(feature = "_ffi-blocking", feature = "_ffi-async"), derive(uniffi::Enum))]
#[cfg_attr(feature = "_defmt", derive(defmt::Format))]
#[derive(Serialize, Deserialize, Display, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Level {
    /// Logical low.
    Low,
    /// Logical high.
    High,
}

impl Not for Level {
    type Output = Level;

    fn not(self) -> Self::Output {
        match self {
            Level::Low => Level::High,
            Level::High => Level::Low,
        }
    }
}

impl From<bool> for Level {
    fn from(val: bool) -> Self {
        match val {
            true => Self::High,
            false => Self::Low,
        }
    }
}

impl From<Level> for bool {
    fn from(level: Level) -> bool {
        match level {
            Level::Low => false,
            Level::High => true,
        }
    }
}

#[cfg(feature = "embedded-hal")]
impl From<embedded_hal::digital::PinState> for Level {
    fn from(value: embedded_hal::digital::PinState) -> Self {
        match value {
            embedded_hal::digital::PinState::Low => Level::Low,
            embedded_hal::digital::PinState::High => Level::High,
        }
    }
}

#[cfg(feature = "embedded-hal")]
impl Into<embedded_hal::digital::PinState> for Level {
    fn into(self) -> embedded_hal::digital::PinState {
        match self {
            Level::Low => embedded_hal::digital::PinState::Low,
            Level::High => embedded_hal::digital::PinState::High,
        }
    }
}
