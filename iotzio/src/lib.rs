#![forbid(future_incompatible)]
#![deny(missing_debug_implementations)]
#![cfg_attr(not(feature = "_std"), no_std)]

#[cfg(any(feature = "_ffi-blocking", feature = "_ffi-async"))]
uniffi::setup_scaffolding!();

mod backend;
pub mod communication;
mod iotzio;
pub mod modules;
pub mod peripherals;
pub mod socket;

#[allow(unused_imports)]
pub use self::iotzio::*;
