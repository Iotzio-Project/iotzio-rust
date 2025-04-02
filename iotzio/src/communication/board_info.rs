use crate::communication::Version;
use serde::{Deserialize, Serialize};

cfg_if::cfg_if! {
    if #[cfg(feature = "_host")] {
        use std::fmt;

        pub type SerialNumberString = String;
    }
    else {
        use core::fmt;

        pub type SerialNumberString = heapless::String<16>;
    }
}

#[cfg_attr(feature = "_defmt", derive(defmt::Format))]
#[derive(Serialize, Deserialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BoardInfo {
    pub version: Version,
    pub protocol_version: u16,
    pub serial_number: SerialNumberString,
}

impl fmt::Display for BoardInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
