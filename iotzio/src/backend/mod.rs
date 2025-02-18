cfg_if::cfg_if! {
    if #[cfg(all(feature = "_std", any(target_os = "windows", target_os = "linux", target_os = "macos")))] {
        mod desktop;
        pub use self::desktop::*;
    }
    else if #[cfg(all(feature = "_std", target_os = "android"))] {
        mod android;
        pub use self::android::*;
    }
    else if #[cfg(all(feature = "_std", target_family = "wasm"))] {
        mod browser;
        pub use self::browser::*;
    }
    else if #[cfg(feature = "_std")] {
        compile_error!("Unsupported target. No backend available.");
    }
}

#[cfg(all(feature = "_std", not(target_arch = "wasm32")))]
static_assertions::assert_impl_all!(DeviceError: std::fmt::Debug, std::fmt::Display, Send, Sync);

#[cfg(all(feature = "_std", target_arch = "wasm32"))]
static_assertions::assert_impl_all!(DeviceError: std::fmt::Debug, std::fmt::Display);

#[cfg(all(feature = "_std", not(target_arch = "wasm32")))]
static_assertions::assert_impl_all!(DeviceInfo: std::fmt::Debug, Send, Sync);

#[cfg(all(feature = "_std", target_arch = "wasm32"))]
static_assertions::assert_impl_all!(DeviceInfo: std::fmt::Debug);

#[cfg(all(feature = "_std", not(target_arch = "wasm32")))]
static_assertions::assert_impl_all!(DeviceReader: std::fmt::Debug, Send, Sync);

#[cfg(all(feature = "_std", target_arch = "wasm32"))]
static_assertions::assert_impl_all!(DeviceReader: std::fmt::Debug);

#[cfg(all(feature = "_std", not(target_arch = "wasm32")))]
static_assertions::assert_impl_all!(DeviceWriter: std::fmt::Debug, Send, Sync);

#[cfg(all(feature = "_std", target_arch = "wasm32"))]
static_assertions::assert_impl_all!(DeviceWriter: std::fmt::Debug);
