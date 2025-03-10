mod output_pin_module_error;

pub use self::output_pin_module_error::*;

cfg_if::cfg_if! {
    if #[cfg(feature = "_host")] {
        mod service;
        mod output_pin;

        pub use self::output_pin::*;
    }
}
