use serde::{Deserialize, Serialize};

cfg_if::cfg_if! {
    if #[cfg(feature = "_std")] {
        use std::time::Duration;
    }
    else {
        use core::time::Duration;
    }
}

#[cfg_attr(feature = "_defmt", derive(defmt::Format))]
#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SignalTypeRequest {
    Low,
    High,
    FallingEdge,
    RisingEdge,
    AnyEdge,
    LowPulse,
    HighPulse,
    AnyPulse,
}

#[cfg_attr(feature = "_defmt", derive(defmt::Format))]
#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SignalTypeResponse {
    Low,
    High,
    FallingEdge,
    RisingEdge,
    AnyEdge,
    LowPulse(Duration),
    HighPulse(Duration),
    AnyPulse(Duration),
}
