#[macro_use]
mod log;
mod app;
mod app_thread;
mod egl;
mod geometry;
mod program;
mod swap_chain;

use crate::app::App;
use crate::app_thread::AppThread;
use crate::egl::EGL;
use crate::geometry::Geometry;
use crate::program::Program;
use crate::swap_chain::SwapChain;
use jni::sys::{jlong, jobject, JNIEnv};
use std::panic;

#[no_mangle]
pub unsafe extern "C" fn Java_com_makepad_rustquest_JNI_onCreate(
    env: *mut JNIEnv,
    _: jobject,
    activity: jobject,
) -> jlong {
    logv!("JNI::onCreate");
    set_panic_hook();
    let app_thread = Box::new(AppThread::new(env, activity));
    Box::into_raw(app_thread) as jlong
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_makepad_rustquest_JNI_onStart(
    _: *mut JNIEnv,
    _: jobject,
    app_thread: jlong,
) {
    logv!("JNI::onStart");
    let app_thread = (app_thread as *mut AppThread).as_mut().unwrap();
    app_thread.on_start();
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_makepad_rustquest_JNI_onResume(
    _: *mut JNIEnv,
    _: jobject,
    app_thread: jlong,
) {
    logv!("JNI::onResume");
    let app_thread = (app_thread as *mut AppThread).as_mut().unwrap();
    app_thread.on_resume();
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_makepad_rustquest_JNI_onPause(
    _: *mut JNIEnv,
    _: jobject,
    app_thread: jlong,
) {
    logv!("JNI::onPause");
    let app_thread = (app_thread as *mut AppThread).as_mut().unwrap();
    app_thread.on_pause();
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_makepad_rustquest_JNI_onStop(
    _: *mut JNIEnv,
    _: jobject,
    app_thread: jlong,
) {
    logv!("JNI::onStop");
    let app_thread = (app_thread as *mut AppThread).as_mut().unwrap();
    app_thread.on_stop();
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_makepad_rustquest_JNI_onDestroy(
    _: *mut JNIEnv,
    _: jobject,
    app_thread: jlong,
) {
    logv!("JNI::onDestroy");
    let app_thread = (app_thread as *mut AppThread).as_mut().unwrap();
    app_thread.on_destroy();
    Box::from_raw(app_thread);
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_makepad_rustquest_JNI_surfaceCreated(
    env: *mut JNIEnv,
    _: jobject,
    app_thread: jlong,
    surface: jobject,
) {
    logv!("JNI::surfaceCreated");
    let app_thread = (app_thread as *mut AppThread).as_mut().unwrap();
    app_thread.surface_created(env, surface);
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_makepad_rustquest_JNI_surfaceChanged(
    env: *mut JNIEnv,
    _: jobject,
    app_thread: jlong,
    surface: jobject,
) {
    logv!("JNI::surfaceChanged");
    let app_thread = (app_thread as *mut AppThread).as_mut().unwrap();
    app_thread.surface_changed(env, surface);
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_makepad_rustquest_JNI_surfaceDestroyed(
    _: *mut JNIEnv,
    _: jobject,
    app_thread: jlong,
) {
    logv!("JNI::surfaceDestroyed");
    let app_thread = (app_thread as *mut AppThread).as_mut().unwrap();
    app_thread.surface_destroyed();
}

fn set_panic_hook() {
    panic::set_hook(Box::new(|panic_info| {
        loge!("{}", panic_info.to_string());
    }));
}
