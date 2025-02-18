#![forbid(unsafe_code)]

use crate::backend::android::hardware::usb::UsbInterface;
use crate::backend::android::utils;
use crate::backend::DeviceError;
use jni::objects::{GlobalRef, JObject, JString};
use jni::sys::jint;
use jni::JNIEnv;
use std::ops::Deref;

#[derive(Debug, Clone)]
pub struct UsbDevice {
    instance: GlobalRef,
}

impl Deref for UsbDevice {
    type Target = JObject<'static>;

    fn deref(&self) -> &Self::Target {
        self.instance.as_obj()
    }
}

impl UsbDevice {
    pub fn new(instance: &JObject, env: &mut JNIEnv) -> Result<Self, DeviceError> {
        let instance = env.new_global_ref(instance)?;

        let class = env.find_class("android/hardware/usb/UsbDevice")?;

        match !instance.as_obj().is_null() && env.is_instance_of(instance.as_obj(), class)? {
            true => Ok(Self { instance }),
            false => Err(DeviceError::JavaWrapperError {
                error_message: "instanceof check for android/hardware/usb/UsbDevice failed ",
            }),
        }
    }

    pub fn get_device_id(&self, env: &mut JNIEnv) -> Result<i32, DeviceError> {
        let device_id = utils::call_method(env, self.instance.as_obj(), "getDeviceId", "()I", &[])?.i()?;

        Ok(device_id)
    }

    pub fn get_vendor_id(&self, env: &mut JNIEnv) -> Result<u16, DeviceError> {
        let vendor_id = utils::call_method(env, self.instance.as_obj(), "getVendorId", "()I", &[])?.i()?;

        Ok(vendor_id as u16)
    }

    pub fn get_product_id(&self, env: &mut JNIEnv) -> Result<u16, DeviceError> {
        let product_id = utils::call_method(env, self.instance.as_obj(), "getProductId", "()I", &[])?.i()?;

        Ok(product_id as u16)
    }

    pub fn get_manufacturer_name(&self, env: &mut JNIEnv) -> Result<Option<String>, DeviceError> {
        let java_str: JString = utils::call_method(
            env,
            self.instance.as_obj(),
            "getManufacturerName",
            "()Ljava/lang/String;",
            &[],
        )?
        .l()?
        .into();

        if java_str.is_null() {
            return Ok(None);
        }

        let product_name: String = env.get_string(&java_str)?.into();

        Ok(Some(product_name))
    }

    pub fn get_product_name(&self, env: &mut JNIEnv) -> Result<Option<String>, DeviceError> {
        let java_str: JString = utils::call_method(
            env,
            self.instance.as_obj(),
            "getProductName",
            "()Ljava/lang/String;",
            &[],
        )?
        .l()?
        .into();

        if java_str.is_null() {
            return Ok(None);
        }

        let product_name: String = env.get_string(&java_str)?.into();

        Ok(Some(product_name))
    }

    pub fn get_serial_number(&self, env: &mut JNIEnv) -> Result<Option<String>, DeviceError> {
        let java_object = utils::call_method(
            env,
            self.instance.as_obj(),
            "getSerialNumber",
            "()Ljava/lang/String;",
            &[],
        )?;

        let java_str: JString = java_object.l()?.into();

        if java_str.is_null() {
            return Ok(None);
        }

        let serial_number: String = env.get_string(&java_str)?.into();

        Ok(Some(serial_number))
    }

    pub fn get_interface(&self, env: &mut JNIEnv, index: u8) -> Result<UsbInterface, DeviceError> {
        let usb_interface = utils::call_method(
            env,
            self.instance.as_obj(),
            "getInterface",
            "(I)Landroid/hardware/usb/UsbInterface;",
            &[(index as jint).into()],
        )?
        .l()?;

        Ok(UsbInterface::new(&usb_interface, env)?)
    }
}
