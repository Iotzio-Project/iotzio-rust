use crate::backend::android::content::intent_filter::IntentFilter;
use crate::backend::android::content::Intent;
use crate::backend::android::hardware::usb::UsbManager;
use crate::backend::android::utils;
use crate::backend::DeviceError;
use jni::objects::{GlobalRef, JObject, JValue};
use jni::JNIEnv;
use jni_min_helper::BroadcastReceiver;
use std::ffi::c_void;
use std::ops::Deref;

#[derive(Debug, Clone)]
pub struct Context {
    instance: GlobalRef,
}

impl Deref for Context {
    type Target = JObject<'static>;

    fn deref(&self) -> &Self::Target {
        self.instance.as_obj()
    }
}

impl Context {
    pub const RECEIVER_NOT_EXPORTED: i32 = 0x4;

    pub fn new(instance: &JObject, env: &mut JNIEnv) -> Result<Self, DeviceError> {
        let instance = env.new_global_ref(instance)?;

        let class = env.find_class("android/content/Context")?;

        match !instance.as_obj().is_null() && env.is_instance_of(instance.as_obj(), class)? {
            true => Ok(Self { instance }),
            false => Err(DeviceError::JavaWrapperError {
                error_message: "instanceof check for android/content/Context failed ",
            }),
        }
    }

    pub fn get_usb_manager(&self, env: &mut JNIEnv) -> Result<UsbManager, DeviceError> {
        let class = env.find_class("android/content/Context")?;

        let usb_service_name = env.get_static_field(class, "USB_SERVICE", "Ljava/lang/String;")?.l()?;

        let usb_service = utils::call_method(
            env,
            self.instance.as_obj(),
            "getSystemService",
            "(Ljava/lang/String;)Ljava/lang/Object;",
            &[JValue::Object(&usb_service_name)],
        )?;

        UsbManager::new(&usb_service.l()?, env)
    }

    pub fn register_receiver(
        &self,
        env: &mut JNIEnv,
        receiver: &BroadcastReceiver,
        filter: &IntentFilter,
    ) -> Result<Option<Intent>, DeviceError> {
        let intent = utils::call_method(
            env,
            self.instance.as_obj(),
            "registerReceiver",
            "(Landroid/content/BroadcastReceiver;Landroid/content/IntentFilter;)Landroid/content/Intent;",
            &[JValue::Object(receiver.deref()), JValue::Object(filter.deref())],
        )?
        .l()?;

        Ok(match intent.is_null() {
            true => None,
            false => Some(Intent::new(&intent, env)?),
        })
    }

    pub fn register_receiver_with_flags(
        &self,
        env: &mut JNIEnv,
        receiver: &BroadcastReceiver,
        filter: &IntentFilter,
        flags: i32,
    ) -> Result<Option<Intent>, DeviceError> {
        let intent = utils::call_method(
            env,
            self.instance.as_obj(),
            "registerReceiver",
            "(Landroid/content/BroadcastReceiver;Landroid/content/IntentFilter;I)Landroid/content/Intent;",
            &[
                JValue::Object(receiver.deref()),
                JValue::Object(filter.deref()),
                JValue::Int(flags),
            ],
        )?
        .l()?;

        Ok(match intent.is_null() {
            true => None,
            false => Some(Intent::new(&intent, env)?),
        })
    }

    pub fn unregister_receiver(&self, env: &mut JNIEnv, receiver: &BroadcastReceiver) -> Result<(), DeviceError> {
        utils::call_method(
            env,
            self.instance.as_obj(),
            "unregisterReceiver",
            "(Landroid/content/BroadcastReceiver;)V",
            &[JValue::Object(receiver.deref())],
        )?;
        Ok(())
    }

    pub fn jni_ptr(&self) -> *mut c_void {
        self.instance.as_obj().as_raw() as _
    }
}
