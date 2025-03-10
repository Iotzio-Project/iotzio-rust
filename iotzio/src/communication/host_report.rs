use crate::communication::Command;
use serde::{Deserialize, Serialize};

cfg_if::cfg_if! {
    if #[cfg(feature = "_host")] {
        use std::fmt;
    }
    else {
        use core::fmt;
    }
}

impl fmt::Display for HostReport {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg_attr(feature = "_defmt", derive(defmt::Format))]
#[derive(Serialize, Deserialize, Debug, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct HostReport {
    pub identifier: u32,
    pub command: Command,
}

pub const HOST_REPORT_HEADER_SIZE: usize = 1 + 4 + 2;
