use crate::communication::{FatalError, Response};
use crate::modules::ModuleError;
use serde::{Deserialize, Serialize};

cfg_if::cfg_if! {
    if #[cfg(feature = "_std")] {
        use std::fmt;
    }
    else {
        use core::fmt;
    }
}

impl fmt::Display for DeviceReport {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg_attr(feature = "_defmt", derive(defmt::Format))]
#[derive(Serialize, Deserialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DeviceReport {
    Response {
        identifier: u32,
        result: Result<Response, ModuleError>,
    },
    FatalError {
        error: FatalError,
    },
}

pub const DEVICE_REPORT_HEADER_SIZE: usize = 1;
