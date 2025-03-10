use serde::{Deserialize, Serialize};

cfg_if::cfg_if! {
    if #[cfg(feature = "_host")] {
        use std::fmt;
    }
    else {
        use core::fmt;
    }
}

/// Semantic versioning "MAJOR.MINOR.PATCH". Applies to iotzio library and the board.
#[cfg_attr(any(feature = "_ffi-blocking", feature = "_ffi-async"), derive(uniffi::Record))]
#[cfg_attr(feature = "_defmt", derive(defmt::Format))]
#[derive(Serialize, Deserialize, Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Version {
    /// Major version change when incompatible API changes are done.
    pub major: u16,

    /// Minor version change when backwards compatible API changes are done.
    pub minor: u16,

    /// Patch version change when backwards compatible bug fixes are done.
    pub patch: u16,
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{0}.{1}.{2}", self.major, self.minor, self.patch)
    }
}
