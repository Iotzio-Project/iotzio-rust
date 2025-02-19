mod app;
mod backend;
mod content;
mod context;
mod hardware;
mod os;
mod permissions;
mod utils;

use jni::objects::{JClass, JObject};
use jni::sys::{jboolean, JNI_FALSE, JNI_TRUE};
use jni::JNIEnv;

pub use self::backend::*;
use self::context::*;
use self::permissions::*;

#[no_mangle]
pub extern "system" fn Java_com_iotzio_api_AndroidHelper_onActivityCreateNative(
    mut env: JNIEnv,
    _: JClass,
    context: JObject,
) -> jboolean {
    match initialize_android_context(&mut env, context) {
        Ok(_) => JNI_TRUE,
        Err(_) => JNI_FALSE,
    }
}
