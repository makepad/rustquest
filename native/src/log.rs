macro_rules! loge {
    ($($arg:tt)*) => ($crate::log::loge(&format!($($arg)*)));
}

macro_rules! logi {
    ($($arg:tt)*) => ($crate::log::logi(&format!($($arg)*)));
}

macro_rules! logv {
    ($($arg:tt)*) => ($crate::log::logv(&format!($($arg)*)));
}

use liblog_sys::{
    android_LogPriority_ANDROID_LOG_ERROR, android_LogPriority_ANDROID_LOG_INFO,
    android_LogPriority_ANDROID_LOG_VERBOSE,
};
use std::ffi::CString;

const TAG: &'static [u8] = b"rustquest\0";

pub fn loge(str: &str) {
    unsafe {
        liblog_sys::__android_log_print(
            android_LogPriority_ANDROID_LOG_ERROR as i32,
            TAG.as_ptr(),
            CString::new(str).unwrap().as_ptr(),
        );
    }
}

pub fn logi(str: &str) {
    unsafe {
        liblog_sys::__android_log_print(
            android_LogPriority_ANDROID_LOG_INFO as i32,
            TAG.as_ptr(),
            CString::new(str).unwrap().as_ptr(),
        );
    }
}

pub fn logv(str: &str) {
    unsafe {
        liblog_sys::__android_log_print(
            android_LogPriority_ANDROID_LOG_VERBOSE as i32,
            TAG.as_ptr(),
            CString::new(str).unwrap().as_ptr(),
        );
    }
}
