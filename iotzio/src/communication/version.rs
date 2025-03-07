use serde::{Deserialize, Serialize};

cfg_if::cfg_if! {
    if #[cfg(feature = "_std")] {
        use std::fmt;
    }
    else {
        use core::fmt;
    }
}

#[cfg_attr(any(feature = "_ffi-blocking", feature = "_ffi-async"), derive(uniffi::Record))]
#[cfg_attr(feature = "_defmt", derive(defmt::Format))]
#[derive(Serialize, Deserialize, Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Version {
    pub major: u16,
    pub minor: u16,
    pub patch: u16,
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{0}.{1}.{2}", self.major, self.minor, self.patch)
    }
}
