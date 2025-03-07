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



#[macro_export]

macro_rules! decorate_object {
    () => {
        #[derive(Debug)]
        #[cfg_attr(feature = "_ffi-uniffi", derive(uniffi::Object))]
        #[cfg_attr(feature = "_ffi-wasm-bindgen", wasm_bindgen::prelude::wasm_bindgen)]
    };
}


pub(crate) enum FFI {
    Invariant,
    On,
    Off,
}

pub(crate) enum Functions {
    Generic,
    Blocking,
    Async,
}

macro_rules! decorate_impl {
    ($impl_type:ident, $ffi:ident) => {
        match ($impl_type, $ffi) {
            (Functions::Generic, FFI::Invariant) => {
                #[cfg_attr(feature = "_ffi-uniffi", uniffi::export)]
                #[cfg_attr(feature = "_ffi-wasm-bindgen", wasm_bindgen::prelude::wasm_bindgen)]
            }
            (Functions::Generic, FFI::On) => {
                #[cfg(any(feature = "_ffi-blocking", feature = "_ffi-async"))]

                #[cfg_attr(feature = "_ffi-uniffi", uniffi::export)]
                #[cfg_attr(feature = "_ffi-wasm-bindgen", wasm_bindgen::prelude::wasm_bindgen)]
            }
            (Functions::Generic, FFI::Off) => {
                #[cfg(not(any(feature = "_ffi-blocking", feature = "_ffi-async")))]
            }
            (Functions::Blocking, FFI::Invariant) => {
                #[cfg(not(target_family = "wasm"))]

                #[cfg_attr(all(feature = "_ffi-blocking", feature = "_ffi-uniffi"), uniffi::export)]
                #[cfg_attr(all(feature = "_ffi-blocking", feature = "_ffi-wasm-bindgen"), wasm_bindgen::prelude::wasm_bindgen)]
            }
            (Functions::Blocking, FFI::On) => {
                #[cfg(all(not(target_family = "wasm"), feature = "_ffi-blocking"))]

                #[cfg_attr(all(feature = "_ffi-blocking", feature = "_ffi-uniffi"), uniffi::export)]
                #[cfg_attr(all(feature = "_ffi-blocking", feature = "_ffi-wasm-bindgen"), wasm_bindgen::prelude::wasm_bindgen)]
            }
            (Functions::Blocking, FFI::Off) => {
                #[cfg(all(not(target_family = "wasm"), not(feature = "_ffi-blocking")))]
            }
            (Functions::Async, FFI::Invariant) => {
                #[cfg_attr(all(feature = "_ffi-async", feature = "_ffi-uniffi"), uniffi::export)]
                #[cfg_attr(all(feature = "_ffi-async", feature = "_ffi-wasm-bindgen"), wasm_bindgen::prelude::wasm_bindgen)]
            }
            (Functions::Async, FFI::On) => {
                #[cfg(feature = "_ffi-async")]
                #[cfg_attr(all(feature = "_ffi-async", feature = "_ffi-uniffi"), uniffi::export)]
                #[cfg_attr(all(feature = "_ffi-async", feature = "_ffi-wasm-bindgen"), wasm_bindgen::prelude::wasm_bindgen)]
            }
            (Functions::Async, FFI::Off) => {
                #[cfg(not(feature = "_ffi-async"))]
            }
        }
    };
}
