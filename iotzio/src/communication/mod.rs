#![forbid(unsafe_code)]

mod board_info;
mod command;
mod device_report;
mod fatal_error;
mod host_report;
mod protocol_error;
mod response;
mod version;

cfg_if::cfg_if! {
    if #[cfg(feature = "_std")] {
        pub(crate) use self::command::*;
        pub(crate) use self::device_report::*;
        pub(crate) use self::host_report::*;
        pub(crate) use self::response::*;
        pub(crate) use self::board_info::*;

        pub(crate) const IOTZIO_PROTOCOL_VERSION: u16 = 1;
        pub(crate) const PROTOCOL_INFO_REPORT_ID: u8 = 0xFF;
        pub(crate) const PROTOCOL_INFO_BUFFER_SIZE: usize = 1025;
    }
    else {
        pub use self::command::*;
        pub use self::device_report::*;
        pub use self::host_report::*;
        pub use self::response::*;
        pub use self::board_info::*;

        pub const IOTZIO_PROTOCOL_VERSION: u16 = 1;
        pub const PROTOCOL_INFO_REPORT_ID: u8 = 0xFF;
        pub const PROTOCOL_INFO_BUFFER_SIZE: usize = 1025;
    }
}

pub use self::fatal_error::*;
pub use self::protocol_error::*;
pub use self::version::*;
