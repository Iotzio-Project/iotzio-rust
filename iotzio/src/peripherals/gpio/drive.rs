use derive_more::Display;
use serde::{Deserialize, Serialize};

/// Drive strength of an output
#[cfg_attr(any(feature = "_uniffi-blocking", feature = "_uniffi-async"), derive(uniffi::Enum))]
#[cfg_attr(feature = "_defmt", derive(defmt::Format))]
#[derive(Serialize, Deserialize, Display, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Drive {
    /// 2 mA drive.
    TwoMilliAmpere,
    /// 4 mA drive.
    FourMilliAmpere,
    /// 8 mA drive.
    EightMilliAmpere,
    /// 12 mA drive.
    TwelveMilliAmpere,
}
