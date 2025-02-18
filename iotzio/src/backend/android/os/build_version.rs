use crate::backend::DeviceError;
use jni::JNIEnv;

#[derive(Debug, Clone)]
pub struct BuildVersion;

impl BuildVersion {
    pub fn current(env: &mut JNIEnv) -> Result<i32, DeviceError> {
        let class = env.find_class("android/os/Build$VERSION")?;

        let sdk_int = env.get_static_field(class, "SDK_INT", "I")?.i()?;

        Ok(sdk_int)
    }

    pub const Q: i32 = 29;

    pub const TIRAMISU: i32 = 33;
}
