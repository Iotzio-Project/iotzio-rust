#![forbid(unsafe_code)]

use crate::backend::DeviceError;
use jni::descriptors::Desc;
use jni::errors::Error;
use jni::objects::{JClass, JObject, JString, JValue, JValueOwned};
use jni::strings::JNIString;
use jni::JNIEnv;
use jni_min_helper::JObjectGet;

pub fn call_constructor<'local, T, U>(
    env: &mut JNIEnv<'local>,
    class: T,
    ctor_sig: U,
    ctor_args: &[JValue],
) -> Result<JObject<'local>, DeviceError>
where
    T: Desc<'local, JClass<'local>>,
    U: Into<JNIString> + AsRef<str>,
{
    env.new_object(class, ctor_sig, ctor_args)
        .map_err(|x| handle_exception(env, x))
}

pub fn call_method<'local, 'other_local, O, S, T>(
    env: &mut JNIEnv<'local>,
    obj: O,
    name: S,
    sig: T,
    args: &[JValue],
) -> Result<JValueOwned<'local>, DeviceError>
where
    O: AsRef<JObject<'other_local>>,
    S: Into<JNIString>,
    T: Into<JNIString> + AsRef<str>,
{
    env.call_method(obj, name, sig, args)
        .map_err(|x| handle_exception(env, x))
}

pub fn call_static_method<'local, T, U, V>(
    env: &mut JNIEnv<'local>,
    class: T,
    name: U,
    sig: V,
    args: &[JValue],
) -> Result<JValueOwned<'local>, DeviceError>
where
    T: Desc<'local, JClass<'local>>,
    U: Into<JNIString>,
    V: Into<JNIString> + AsRef<str>,
{
    env.call_static_method(class, name, sig, args)
        .map_err(|x| handle_exception(env, x))
}

fn handle_exception(env: &mut JNIEnv, error: jni::errors::Error) -> DeviceError {
    match error {
        Error::JavaException => {
            let throwable = env.exception_occurred().unwrap();

            let class = env.get_object_class(&throwable).unwrap().get_class_name(env).unwrap();

            let message: JString = env
                .call_method(throwable, "getMessage", "()Ljava/lang/String;", &[])
                .unwrap()
                .l()
                .unwrap()
                .into();

            let message: String = env.get_string(&message).unwrap().into();

            env.exception_clear().unwrap();

            DeviceError::JavaException { class, message }
        }
        x => DeviceError::from(x),
    }
}
