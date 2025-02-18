#![forbid(unsafe_code)]

use crate::backend::android::app::PendingIntent;
use crate::backend::android::hardware::usb::{UsbDevice, UsbDeviceConnection};
use crate::backend::android::utils;
use crate::backend::DeviceError;
use jni::objects::{GlobalRef, JMap, JObject, JString, JValue};
use jni::JNIEnv;
use std::collections::HashMap;
use std::ops::Deref;

#[derive(Debug, Clone)]
pub struct UsbManager {
    instance: GlobalRef,
}

impl Deref for UsbManager {
    type Target = JObject<'static>;

    fn deref(&self) -> &Self::Target {
        self.instance.as_obj()
    }
}

impl UsbManager {
    pub fn new(instance: &JObject, env: &mut JNIEnv) -> Result<Self, DeviceError> {
        let instance = env.new_global_ref(instance)?;

        let class = env.find_class("android/hardware/usb/UsbManager")?;

        match !instance.as_obj().is_null() && env.is_instance_of(instance.as_obj(), class)? {
            true => Ok(Self { instance }),
            false => Err(DeviceError::JavaWrapperError {
                error_message: "instanceof check for android/hardware/usb/UsbManager failed ",
            }),
        }
    }

    pub fn open_device(
        &self,
        env: &mut JNIEnv,
        usb_device: &UsbDevice,
    ) -> Result<Option<UsbDeviceConnection>, DeviceError> {
        let usb_device_connection = utils::call_method(
            env,
            self.instance.as_obj(),
            "openDevice",
            "(Landroid/hardware/usb/UsbDevice;)Landroid/hardware/usb/UsbDeviceConnection;",
            &[usb_device.deref().into()],
        )?
        .l()?;

        if usb_device_connection.is_null() {
            Ok(None)
        } else {
            Ok(Some(UsbDeviceConnection::new(&usb_device_connection, env)?))
        }
    }

    pub fn has_permission(&self, env: &mut JNIEnv, usb_device: &UsbDevice) -> Result<bool, DeviceError> {
        let has_permission = utils::call_method(
            env,
            self.instance.as_obj(),
            "hasPermission",
            "(Landroid/hardware/usb/UsbDevice;)Z",
            &[usb_device.deref().into()],
        )?
        .z()?;

        Ok(has_permission)
    }

    pub fn get_device_list(&self, env: &mut JNIEnv) -> Result<HashMap<String, UsbDevice>, DeviceError> {
        let j_object: JObject = utils::call_method(
            env,
            self.instance.as_obj(),
            "getDeviceList",
            "()Ljava/util/HashMap;",
            &[],
        )?
        .l()?;
        let j_map = JMap::from_env(env, &j_object)?;
        let mut j_iterator = j_map.iter(env)?;

        let mut hash_map = HashMap::new();

        loop {
            match j_iterator.next(env)? {
                None => break,
                Some((key, value)) => {
                    let key: String = env.get_string(&JString::from(key))?.into();
                    let usb_device = UsbDevice::new(&value, env)?;

                    hash_map.insert(key, usb_device);
                }
            }
        }

        Ok(hash_map)
    }

    pub fn request_permission(
        &self,
        env: &mut JNIEnv,
        usb_device: &UsbDevice,
        pending_intent: &PendingIntent,
    ) -> Result<(), DeviceError> {
        utils::call_method(
            env,
            self.instance.as_obj(),
            "requestPermission",
            "(Landroid/hardware/usb/UsbDevice;Landroid/app/PendingIntent;)V",
            &[
                JValue::Object(usb_device.deref()),
                JValue::Object(pending_intent.deref()),
            ],
        )?;
        Ok(())
    }
}
