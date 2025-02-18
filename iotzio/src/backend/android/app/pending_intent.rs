use crate::backend::android::content::{Context, Intent};
use crate::backend::android::utils;
use crate::backend::DeviceError;
use jni::objects::{GlobalRef, JObject, JValue, JValueGen};
use jni::JNIEnv;
use std::ops::Deref;

#[derive(Debug, Clone)]
pub struct PendingIntent {
    instance: GlobalRef,
}

impl Deref for PendingIntent {
    type Target = JObject<'static>;

    fn deref(&self) -> &Self::Target {
        self.instance.as_obj()
    }
}

impl PendingIntent {
    pub const FLAG_ONE_SHOT: i32 = 1 << 30;

    pub const FLAG_IMMUTABLE: i32 = 1 << 26;

    pub fn get_broadcast(
        env: &mut JNIEnv,
        context: &Context,
        request_code: i32,
        intent: &Intent,
        flags: i32,
    ) -> Result<Self, DeviceError> {
        let class = env.find_class("android/app/PendingIntent")?;

        let instance = utils::call_static_method(
            env,
            class,
            "getBroadcast",
            "(Landroid/content/Context;ILandroid/content/Intent;I)Landroid/app/PendingIntent;",
            &[
                JValue::Object(context.deref()),
                JValueGen::Int(request_code),
                JValue::Object(intent.deref()),
                JValueGen::Int(flags),
            ],
        )?
        .l()?;

        let instance = env.new_global_ref(instance)?;

        match !instance.as_obj().is_null() {
            true => Ok(Self { instance }),
            false => Err(DeviceError::JavaWrapperError {
                error_message: "null check for android/app/PendingIntent failed ",
            }),
        }
    }
}
