mod app;
mod backend;
mod content;
mod context;
mod hardware;
mod os;
mod permissions;
mod utils;

use jni::objects::{JClass, JObject};
use jni::JNIEnv;

pub use self::backend::*;
use self::context::*;
use self::permissions::*;

#[no_mangle]
pub extern "system" fn Java_com_iotzio_api_AndroidHelper_onActivityCreateNative(
    mut env: JNIEnv,
    _: JClass,
    context: JObject,
) {
    _ = initialize_android_context(&mut env, context).map_err(|x| {
        _ = env
            .find_class("java/lang/RuntimeException")
            .and_then(|class| env.throw_new(class, format!("{0}", x)));
    });
}
