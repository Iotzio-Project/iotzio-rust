mod i2c0_scl_pin;
mod i2c0_sda_pin;
mod i2c1_scl_pin;
mod i2c1_sda_pin;
mod i2c_bus_module_error;
mod i2c_config;

pub use self::i2c0_scl_pin::*;
pub use self::i2c0_sda_pin::*;
pub use self::i2c1_scl_pin::*;
pub use self::i2c1_sda_pin::*;
pub use self::i2c_bus_module_error::*;
pub use self::i2c_config::*;

cfg_if::cfg_if! {
    if #[cfg(feature = "_host")] {
        mod service;
        mod i2c_bus;

        pub use self::i2c_bus::*;
    }
}
