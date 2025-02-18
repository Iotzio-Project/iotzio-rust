use crate::backend::android::utils;
use crate::backend::DeviceError;
use jni::objects::{GlobalRef, JObject, JString, JValue};
use jni::JNIEnv;
use std::ops::Deref;

#[derive(Debug, Clone)]
pub struct Intent {
    instance: GlobalRef,
}

impl Deref for Intent {
    type Target = JObject<'static>;

    fn deref(&self) -> &Self::Target {
        self.instance.as_obj()
    }
}

impl Intent {
    pub fn new(instance: &JObject, env: &mut JNIEnv) -> Result<Self, DeviceError> {
        let instance = env.new_global_ref(instance)?;

        let class = env.find_class("android/content/Intent")?;

        match !instance.as_obj().is_null() && env.is_instance_of(instance.as_obj(), class)? {
            true => Ok(Self { instance }),
            false => Err(DeviceError::JavaWrapperError {
                error_message: "instanceof check for android/content/Intent failed ",
            }),
        }
    }

    pub fn new_action(env: &mut JNIEnv, action: &str) -> Result<Self, DeviceError> {
        let class = env.find_class("android/content/Intent")?;

        let java_str = env.new_string(action)?;

        let instance = utils::call_constructor(env, class, "(Ljava/lang/String;)V", &[JValue::Object(&java_str)])?;

        let instance = env.new_global_ref(instance)?;

        match !instance.as_obj().is_null() {
            true => Ok(Self { instance }),
            false => Err(DeviceError::JavaWrapperError {
                error_message: "null check for android/content/Intent failed ",
            }),
        }
    }

    pub fn get_action(&self, env: &mut JNIEnv) -> Result<Option<String>, DeviceError> {
        let java_str: JString =
            utils::call_method(env, self.instance.as_obj(), "getAction", "()Ljava/lang/String;", &[])?
                .l()?
                .into();

        if java_str.is_null() {
            return Ok(None);
        }

        let product_name: String = env.get_string(&java_str)?.into();

        Ok(Some(product_name))
    }
}
