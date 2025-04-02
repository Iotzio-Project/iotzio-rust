#![forbid(unsafe_code)]

#[allow(unused)]
pub const USB_VENDOR_ID: u16 = 0x2E8A;

#[allow(unused)]
pub const USB_PRODUCT_ID: u16 = 0x000F;

#[allow(unused)]
pub const USB_USAGE_PAGE: u16 = 0xFF00;

#[allow(unused)]
pub const USB_USAGE_ID: u16 = 0x0001;

#[allow(unused)]
pub const USB_MANUFACTURER_NAME: &str = "Iotzio Project";

#[allow(unused)]
pub const USB_PRODUCT_NAME_PREFIX: &str = "Iotzio ";

cfg_if::cfg_if! {
    if #[cfg(feature = "_host")] {
        pub(crate) mod iotzio_service;

        mod initialization_error;
        pub use self::initialization_error::*;

        mod iotzio_manager;
        pub use self::iotzio_manager::*;

        mod iotzio;
        pub use self::iotzio::*;

        mod iotzio_info;
        pub use self::iotzio_info::*;
    }
}

cfg_if::cfg_if! {
    if #[cfg(all(feature = "_host", feature = "embedded-hal"))] {
        mod delay;
        pub use self::delay::*;
    }
}
