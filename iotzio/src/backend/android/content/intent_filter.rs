use crate::backend::android::utils;
use crate::backend::DeviceError;
use jni::objects::{GlobalRef, JObject, JValue};
use jni::JNIEnv;
use std::ops::Deref;

#[derive(Debug, Clone)]
pub struct IntentFilter {
    instance: GlobalRef,
}

impl Deref for IntentFilter {
    type Target = JObject<'static>;

    fn deref(&self) -> &Self::Target {
        self.instance.as_obj()
    }
}

impl IntentFilter {
    pub fn new_action(env: &mut JNIEnv, action: &str) -> Result<Self, DeviceError> {
        let class = env.find_class("android/content/IntentFilter")?;

        let java_str = env.new_string(action)?;

        let instance = utils::call_constructor(env, class, "(Ljava/lang/String;)V", &[JValue::Object(&java_str)])?;

        let instance = env.new_global_ref(instance)?;

        match !instance.as_obj().is_null() {
            true => Ok(Self { instance }),
            false => Err(DeviceError::JavaWrapperError {
                error_message: "null check for android/content/IntentFilter failed ",
            }),
        }
    }
}
