mod input_pin_module_error;
mod interrupt_trigger;
mod signal_type;

pub use self::input_pin_module_error::*;
pub use self::interrupt_trigger::*;

cfg_if::cfg_if! {
    if #[cfg(feature = "_host")] {
        mod service;
        mod input_pin;

        pub use self::input_pin::*;
        pub(crate) use self::signal_type::*;
    }
    else {
        pub use self::signal_type::*;
    }
}
