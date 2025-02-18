#![forbid(unsafe_code)]

use crate::backend::android::hardware::usb::UsbEndpoint;
use crate::backend::android::utils;
use crate::backend::DeviceError;
use jni::objects::{GlobalRef, JObject};
use jni::sys::jint;
use jni::JNIEnv;
use std::ops::Deref;

#[derive(Debug, Clone)]
pub struct UsbInterface {
    instance: GlobalRef,
}

impl Deref for UsbInterface {
    type Target = JObject<'static>;

    fn deref(&self) -> &Self::Target {
        self.instance.as_obj()
    }
}

impl UsbInterface {
    pub fn new(instance: &JObject, env: &mut JNIEnv) -> Result<Self, DeviceError> {
        let instance = env.new_global_ref(instance)?;

        let class = env.find_class("android/hardware/usb/UsbInterface")?;

        match !instance.as_obj().is_null() && env.is_instance_of(instance.as_obj(), class)? {
            true => Ok(Self { instance }),
            false => Err(DeviceError::JavaWrapperError {
                error_message: "instanceof check for android/hardware/usb/UsbInterface failed ",
            }),
        }
    }

    pub fn get_endpoint_count(&self, env: &mut JNIEnv) -> Result<u8, DeviceError> {
        let endpoint_count = utils::call_method(env, self.instance.as_obj(), "getEndpointCount", "()I", &[])?.i()?;

        Ok(endpoint_count as u8)
    }

    pub fn get_endpoint(&self, env: &mut JNIEnv, index: u8) -> Result<UsbEndpoint, DeviceError> {
        let usb_endpoint = utils::call_method(
            env,
            self.instance.as_obj(),
            "getEndpoint",
            "(I)Landroid/hardware/usb/UsbEndpoint;",
            &[(index as jint).into()],
        )?
        .l()?;

        Ok(UsbEndpoint::new(&usb_endpoint, env)?)
    }
}
