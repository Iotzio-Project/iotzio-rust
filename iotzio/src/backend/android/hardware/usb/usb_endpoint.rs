#![forbid(unsafe_code)]

use crate::backend::android::utils;
use crate::backend::DeviceError;
use jni::objects::{GlobalRef, JObject};
use jni::JNIEnv;
use std::ops::Deref;

#[derive(Debug, Clone)]
pub struct UsbEndpoint {
    instance: GlobalRef,
}

impl Deref for UsbEndpoint {
    type Target = JObject<'static>;

    fn deref(&self) -> &Self::Target {
        self.instance.as_obj()
    }
}

impl UsbEndpoint {
    pub fn new(instance: &JObject, env: &mut JNIEnv) -> Result<Self, DeviceError> {
        let instance = env.new_global_ref(instance)?;

        let class = env.find_class("android/hardware/usb/UsbEndpoint")?;

        match !instance.as_obj().is_null() && env.is_instance_of(instance.as_obj(), class)? {
            true => Ok(Self { instance }),
            false => Err(DeviceError::JavaWrapperError {
                error_message: "instanceof check for android/hardware/usb/UsbEndpoint failed ",
            }),
        }
    }

    pub fn get_type(&self, env: &mut JNIEnv) -> Result<i32, DeviceError> {
        let type_value = utils::call_method(env, self.instance.as_obj(), "getType", "()I", &[])?.i()?;

        Ok(type_value)
    }

    pub fn get_direction(&self, env: &mut JNIEnv) -> Result<i32, DeviceError> {
        let direction_value = utils::call_method(env, self.instance.as_obj(), "getDirection", "()I", &[])?.i()?;

        Ok(direction_value)
    }
}
