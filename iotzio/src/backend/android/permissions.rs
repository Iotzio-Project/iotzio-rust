#![forbid(unsafe_code)]

use crate::backend::android::app::PendingIntent;
use crate::backend::android::content::{Context, Intent, IntentFilter};
use crate::backend::android::hardware::usb::{UsbDevice, UsbManager};
use crate::backend::android::os::BuildVersion;
use crate::backend::DeviceError;
use jni::objects::JObject;
use jni::{JNIEnv, JavaVM};
use jni_min_helper::BroadcastReceiver;
use std::sync::{Arc, Mutex};

static PACKAGE: &'static str = "com.iotzio.api";

static PENDING_INTENTS: Mutex<
    Vec<(
        String,
        UsbDevice,
        UsbManager,
        Arc<BroadcastReceiver>,
        async_oneshot::Sender<Result<(), DeviceError>>,
    )>,
> = Mutex::new(Vec::new());

pub fn request_permissions(
    java_vm: &Arc<JavaVM>,
    context: &Context,
    usb_manager: &UsbManager,
    device_info: &UsbDevice,
    mut tx: async_oneshot::Sender<Result<(), DeviceError>>,
) -> Result<(), DeviceError> {
    let mut env = java_vm.attach_current_thread()?;

    if usb_manager.has_permission(&mut env, device_info)? {
        _ = tx.send(Ok(()));
        return Ok(());
    }

    let device_id = device_info.get_device_id(&mut env)?;

    let action = format!("{0}.{1}", PACKAGE, device_id);

    let permission_request_filter = Intent::new_action(&mut env, &action)?;
    let permission_request = PendingIntent::get_broadcast(
        &mut env,
        &context,
        0,
        &permission_request_filter,
        PendingIntent::FLAG_ONE_SHOT | PendingIntent::FLAG_IMMUTABLE,
    )?;

    let context_listener_filter = IntentFilter::new_action(&mut env, &action)?;
    let context_listener = Arc::new(BroadcastReceiver::build(on_receive)?);

    if BuildVersion::current(&mut env)? >= BuildVersion::TIRAMISU {
        context.register_receiver_with_flags(
            &mut env,
            &context_listener,
            &context_listener_filter,
            Context::RECEIVER_NOT_EXPORTED,
        )?;
    } else {
        context.register_receiver(&mut env, &context_listener, &context_listener_filter)?;
    }

    PENDING_INTENTS.lock().unwrap().push((
        action,
        device_info.clone(),
        usb_manager.clone(),
        context_listener.clone(),
        tx,
    ));

    usb_manager.request_permission(&mut env, &device_info, &permission_request)
}

fn on_receive(env: &mut JNIEnv, context: &JObject, intent: &JObject) -> Result<(), jni::errors::Error> {
    let context = Context::new(context, env).unwrap();

    let intent = Intent::new(intent, env).unwrap();

    let current_action = intent.get_action(env).unwrap().unwrap_or(String::with_capacity(0));

    let mut pending_requests = PENDING_INTENTS.lock().unwrap();

    let mut target_index: Option<usize> = None;

    for (index, (action, device_info, usb_manager, context_listener, tx)) in pending_requests.iter_mut().enumerate() {
        if action.as_str().eq(current_action.as_str()) {
            fn inner(
                env: &mut JNIEnv,
                device_info: &UsbDevice,
                usb_manager: &UsbManager,
                context: &Context,
                context_listener: &Arc<BroadcastReceiver>,
            ) -> Result<(), DeviceError> {
                let mut result = match usb_manager.has_permission(env, device_info) {
                    Ok(true) => Ok(()),
                    Ok(false) => Err(DeviceError::NoPermissionGranted),
                    Err(x) => Err(x),
                };

                match context.unregister_receiver(env, context_listener) {
                    Ok(_) => {}
                    Err(x) => {
                        if result.is_ok() {
                            result = Err(x)
                        }
                    }
                }

                result
            }

            _ = tx.send(inner(env, &device_info, usb_manager, &context, context_listener));

            target_index = Some(index);
            break;
        }
    }

    match target_index {
        None => {}
        Some(x) => _ = pending_requests.remove(x),
    }

    Ok(())
}
