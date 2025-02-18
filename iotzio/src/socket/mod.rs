#![forbid(unsafe_code)]

cfg_if::cfg_if! {
    if #[cfg(feature = "_std")] {
        mod socket;
        mod socket_service;
        mod runtime_identifier;

        pub(crate) use self::socket::*;
        pub(crate) use self::runtime_identifier::*;
    }
}
