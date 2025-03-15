use derive_more::Display;
use serde::{Deserialize, Serialize};

/// Represents a pull setting for an input.
#[cfg_attr(any(feature = "_ffi-blocking", feature = "_ffi-async"), derive(uniffi::Enum))]
#[cfg_attr(feature = "_defmt", derive(defmt::Format))]
#[derive(Serialize, Deserialize, Display, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Pull {
    /// No pull.
    None,
    /// Internal pull-up resistor.
    Up,
    /// Internal pull-down resistor.
    ///
    /// Don't expect the internal pulldown to work when working with weak input signals. Use an
    /// external pulldown 8.2k or lower if you need a reliable pulldown, or make sure your input
    /// signal can sink enough current to override the leakage current.
    Down,
}
