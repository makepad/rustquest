use crate::{EGL, Program};
use jni::sys::{jobject, JavaVM};
use libandroid_sys::ANativeWindow;
use libvrapi_sys::{
    ovrInitializeStatus__VRAPI_INITIALIZE_SUCCESS, ovrJava, ovrMobile,
    ovrModeFlags__VRAPI_MODE_FLAG_NATIVE_WINDOW,
    ovrModeFlags__VRAPI_MODE_FLAG_RESET_WINDOW_FULLSCREEN,
};
use std::mem;
use std::ptr;

pub struct App {
    vm: *mut JavaVM,
    java: ovrJava,
    egl: EGL,
    program: Program,
    resumed: bool,
    window: *mut ANativeWindow,
    vr: *mut ovrMobile,
}

impl App {
    pub fn new(vm: *mut JavaVM, activity: jobject) -> App {
        let java = unsafe {
            logi!("attach current thread");
            let mut java = mem::zeroed::<ovrJava>();
            java.Vm = vm as _;
            ((**vm).AttachCurrentThread.unwrap())(
                vm,
                &mut java.Env as *mut _ as *mut _,
                ptr::null_mut(),
            );
            java.ActivityObject = activity as _;
            java
        };
        unsafe {
            logi!("initialize vrapi");
            let parms = libvrapi_sys::vrapi_DefaultInitParms(&java);
            if libvrapi_sys::vrapi_Initialize(&parms)
                != ovrInitializeStatus__VRAPI_INITIALIZE_SUCCESS
            {
                panic!("can't initialize vrapi");
            }
        };
        App {
            vm,
            java,
            egl: EGL::new(),
            program: Program::new(),
            resumed: false,
            window: ptr::null_mut(),
            vr: ptr::null_mut(),
        }
    }

    pub fn set_resumed(&mut self, resumed: bool) {
        self.resumed = resumed;
        self.update_vr_mode();
    }

    pub fn set_window(&mut self, window: *mut ANativeWindow) {
        self.window = window;
        self.update_vr_mode();
    }

    pub fn render_frame(&mut self) {
        if self.vr.is_null() {
            return;
        }
    }

    fn update_vr_mode(&mut self) {
        if self.resumed && !self.window.is_null() {
            if self.vr.is_null() {
                unsafe {
                    logi!("enter vr mode");
                    let mut parms = libvrapi_sys::vrapi_DefaultModeParms(&self.java);
                    parms.Flags |= ovrModeFlags__VRAPI_MODE_FLAG_RESET_WINDOW_FULLSCREEN;
                    parms.Flags |= ovrModeFlags__VRAPI_MODE_FLAG_NATIVE_WINDOW;
                    parms.Display = self.egl.display() as u64;
                    parms.WindowSurface = self.window as u64;
                    parms.ShareContext = self.egl.context() as u64;
                    self.vr = libvrapi_sys::vrapi_EnterVrMode(&parms);
                    if self.vr.is_null() {
                        panic!("can't enter vr mode");
                    }
                }
            }
        } else {
            if !self.vr.is_null() {
                unsafe {
                    logi!("leave vr mode");
                    libvrapi_sys::vrapi_LeaveVrMode(self.vr);
                    self.vr = ptr::null_mut();
                }
            }
        }
    }
}

impl Drop for App {
    fn drop(&mut self) {
        unsafe {
            logi!("shutdown vrapi");
            libvrapi_sys::vrapi_Shutdown();

            logi!("detach current thread");
            ((**self.vm).DetachCurrentThread.unwrap())(self.vm);
        }
    }
}
