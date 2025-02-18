use crate::backend::android::content::Context;
use crate::backend::DeviceError;
use jni::objects::JObject;
use jni::{JNIEnv, JavaVM};
use std::panic;
use std::sync::{Arc, Mutex};

static SDK_CONTEXT: Mutex<Option<Context>> = Mutex::new(None);

pub fn get_android_context() -> Result<(Arc<JavaVM>, JObject<'static>), DeviceError> {
    let android_context =
        panic::catch_unwind(|| ndk_context::android_context()).map_err(|_| DeviceError::NdkContextError {
            error_message:
                "ndk_context is not initialized. See class com.iotzio.api.AndroidHelper for more information.",
        })?;

    let java_vm = unsafe { JavaVM::from_raw(android_context.vm().cast()) }?;

    let context = unsafe { JObject::from_raw(android_context.context().cast()) };

    Ok((java_vm.into(), context))
}

pub fn initialize_android_context(env: &mut JNIEnv, context: JObject) -> Result<(), DeviceError> {
    let mut current_context = SDK_CONTEXT.lock().unwrap();

    if current_context.is_some() {
        return Ok(());
    }

    //android_logger::init_once(android_logger::Config::default().with_max_level(log::LevelFilter::Trace).with_tag("Iotzio Native"));

    let java_vm_ptr = env.get_java_vm()?.get_java_vm_pointer().cast();

    let context = Context::new(&context, env)?;

    let context_ptr = context.jni_ptr();

    *current_context = Some(context);

    match panic::catch_unwind(|| unsafe { ndk_context::initialize_android_context(java_vm_ptr, context_ptr) }) {
        Ok(_) => Ok(()),
        Err(_) => Err(DeviceError::NdkContextError {
            error_message: "ndk_context was already initialized.",
        }),
    }
}
