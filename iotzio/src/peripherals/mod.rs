#![forbid(unsafe_code)]

pub mod gpio;
pub mod i2c;

cfg_if::cfg_if! {
    if #[cfg(feature = "_std")] {
        pub(crate) const BUS_BUFFER_SIZE: usize = 512;

        pub(crate) type BusBuffer = heapless::Vec<u8, BUS_BUFFER_SIZE>;
    }
    else
    {
        pub const BUS_BUFFER_SIZE: usize = 512;

        pub type BusBuffer = heapless::Vec<u8, BUS_BUFFER_SIZE>;
    }
}
