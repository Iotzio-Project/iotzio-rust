use derive_more::Display;
use serde::{Deserialize, Serialize};

/// Slew rate of an output.
#[cfg_attr(any(feature = "_uniffi-blocking", feature = "_uniffi-async"), derive(uniffi::Enum))]
#[cfg_attr(feature = "_defmt", derive(defmt::Format))]
#[derive(Serialize, Deserialize, Display, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SlewRate {
    /// Fast slew rate.
    Fast,
    /// Slow slew rate.
    Slow,
}
