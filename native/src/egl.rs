use libEGL_sys::{
    EGLContext, EGLDisplay, EGLSurface, EGLint, EGL_ALPHA_SIZE, EGL_BAD_ACCESS, EGL_BAD_ALLOC,
    EGL_BAD_ATTRIBUTE, EGL_BAD_CONFIG, EGL_BAD_CONTEXT, EGL_BAD_CURRENT_SURFACE, EGL_BAD_DISPLAY,
    EGL_BAD_MATCH, EGL_BAD_NATIVE_PIXMAP, EGL_BAD_NATIVE_WINDOW, EGL_BAD_PARAMETER,
    EGL_BAD_SURFACE, EGL_BLUE_SIZE, EGL_CONTEXT_CLIENT_VERSION, EGL_CONTEXT_LOST, EGL_DEPTH_SIZE,
    EGL_FALSE, EGL_GREEN_SIZE, EGL_HEIGHT, EGL_NONE, EGL_NOT_INITIALIZED, EGL_OPENGL_ES3_BIT_KHR,
    EGL_PBUFFER_BIT, EGL_RED_SIZE, EGL_RENDERABLE_TYPE, EGL_SAMPLES, EGL_STENCIL_SIZE,
    EGL_SURFACE_TYPE, EGL_WIDTH, EGL_WINDOW_BIT,
};
use std::ptr;

pub struct EGL {
    display: EGLDisplay,
    context: EGLContext,
    surface: EGLSurface,
}

impl EGL {
    pub fn new() -> EGL {
        unsafe {
            logi!("get EGL display connection");
            let display = libEGL_sys::eglGetDisplay(ptr::null_mut());
            if display.is_null() {
                panic!(
                    "can't get EGL display connection: {}",
                    get_error_string(libEGL_sys::eglGetError())
                );
            }

            logi!("initialize EGL display connection");
            if libEGL_sys::eglInitialize(display, ptr::null_mut(), ptr::null_mut()) == EGL_FALSE {
                panic!(
                    "can't initialize EGL display connection: {}",
                    get_error_string(libEGL_sys::eglGetError())
                );
            }

            logi!("choose EGL config");

            logv!("get number of EGL configs");
            let mut config_count = 0;
            if libEGL_sys::eglGetConfigs(display, ptr::null_mut(), 0, &mut config_count)
                == EGL_FALSE
            {
                panic!(
                    "can't choose EGL config: {}",
                    get_error_string(libEGL_sys::eglGetError())
                );
            };

            logv!("get EGL configs");
            let mut configs = Vec::with_capacity(config_count as usize);
            if libEGL_sys::eglGetConfigs(
                display,
                configs.as_mut_ptr(),
                config_count,
                &mut config_count,
            ) == EGL_FALSE
            {
                panic!(
                    "can't choose EGL config: {}",
                    get_error_string(libEGL_sys::eglGetError())
                );
            }
            configs.set_len(config_count as usize);

            let config = configs
                .iter()
                .cloned()
                .enumerate()
                .find_map(|(index, config)| {
                    logv!("check renderable type of config {}", index);
                    let mut renderable_type = 0;
                    if libEGL_sys::eglGetConfigAttrib(
                        display,
                        config,
                        EGL_RENDERABLE_TYPE as i32,
                        &mut renderable_type,
                    ) == EGL_FALSE
                    {
                        panic!(
                            "can't choose EGL config: {}",
                            get_error_string(libEGL_sys::eglGetError())
                        );
                    }
                    if renderable_type & EGL_OPENGL_ES3_BIT_KHR as EGLint == 0 {
                        return None;
                    }

                    logv!("check surface type of config {}", index);
                    let mut surface_type = 0;
                    if libEGL_sys::eglGetConfigAttrib(
                        display,
                        config,
                        EGL_SURFACE_TYPE as EGLint,
                        &mut surface_type,
                    ) == EGL_FALSE
                    {
                        panic!(
                            "can't choose EGL config: {}",
                            get_error_string(libEGL_sys::eglGetError())
                        );
                    }
                    if surface_type & EGL_WINDOW_BIT as EGLint == 0
                        || surface_type & EGL_PBUFFER_BIT as EGLint == 0
                    {
                        return None;
                    }

                    logv!("check remaining attributes of config {}", index);
                    let attribs = [
                        EGL_RED_SIZE as EGLint,
                        8,
                        EGL_GREEN_SIZE as EGLint,
                        8,
                        EGL_BLUE_SIZE as EGLint,
                        8,
                        EGL_ALPHA_SIZE as EGLint,
                        8,
                        EGL_DEPTH_SIZE as EGLint,
                        0,
                        EGL_STENCIL_SIZE as EGLint,
                        0,
                        EGL_SAMPLES as EGLint,
                        0,
                    ];
                    if !attribs
                    .chunks(2)
                    .all(|attrib| {
                        let mut value = 0;
                        if libEGL_sys::eglGetConfigAttrib(display, config, attrib[0], &mut value)
                            == EGL_FALSE
                        {
                            panic!(
                                "can't choose EGL config: {}",
                                get_error_string(libEGL_sys::eglGetError())
                            );
                        }

                        value == attrib[1]
                    }) {
                        return None;
                    }

                    logv!("chose config {}", index);

                    Some(config)
                })
                .expect("can't choose EGL config");

            logi!("create EGL context");
            let attribs = [EGL_CONTEXT_CLIENT_VERSION as EGLint, 3, EGL_NONE as EGLint];
            let context =
                libEGL_sys::eglCreateContext(display, config, ptr::null_mut(), attribs.as_ptr());
            if context.is_null() {
                panic!(
                    "can't create EGL context: {}",
                    get_error_string(libEGL_sys::eglGetError())
                );
            }

            logi!("create EGL pbuffer surface");
            let attribs = [
                EGL_WIDTH as EGLint,
                16,
                EGL_HEIGHT as EGLint,
                16,
                EGL_NONE as EGLint,
            ];
            let surface = libEGL_sys::eglCreatePbufferSurface(display, config, attribs.as_ptr());
            if surface.is_null() {
                panic!(
                    "can't create EGL pbuffer surface: {}",
                    get_error_string(libEGL_sys::eglGetError())
                );
            }

            logi!("make EGL context current");
            if libEGL_sys::eglMakeCurrent(display, surface, surface, context) == EGL_FALSE {
                panic!(
                    "can't make EGL context current: {}",
                    get_error_string(libEGL_sys::eglGetError())
                );
            }

            EGL {
                display,
                context,
                surface,
            }
        }
    }

    pub fn display(&self) -> EGLDisplay {
        self.display
    }

    pub fn context(&self) -> EGLContext {
        self.context
    }
}

impl Drop for EGL {
    fn drop(&mut self) {
        unsafe {
            logi!("make EGL context uncurrent");
            libEGL_sys::eglMakeCurrent(self.display, self.surface, self.surface, self.context);

            logi!("destroy EGL surface");
            libEGL_sys::eglDestroySurface(self.display, self.surface);

            logi!("destroy EGL context");
            libEGL_sys::eglDestroyContext(self.display, self.context);

            logi!("terminate EGL display connection");
            libEGL_sys::eglTerminate(self.display);
        }
    }
}

fn get_error_string(error: EGLint) -> &'static str {
    match error as u32 {
        EGL_NOT_INITIALIZED => "EGL_NOT_INITIALIZED",
        EGL_BAD_ACCESS => "EGL_BAD_ACCESS",
        EGL_BAD_ALLOC => "EGL_BAD_ALLOC",
        EGL_BAD_ATTRIBUTE => "EGL_BAD_ATTRIBUTE",
        EGL_BAD_CONTEXT => "EGL_BAD_CONTEXT",
        EGL_BAD_CONFIG => "EGL_BAD_CONFIG",
        EGL_BAD_CURRENT_SURFACE => "EGL_BAD_CURRENT_SURFACE",
        EGL_BAD_DISPLAY => "EGL_BAD_DISPLAY",
        EGL_BAD_SURFACE => "EGL_BAD_SURFACE",
        EGL_BAD_MATCH => "EGL_BAD_MATCH",
        EGL_BAD_PARAMETER => "EGL_BAD_PARAMETER",
        EGL_BAD_NATIVE_PIXMAP => "EGL_NATIVE_PIXMAP",
        EGL_BAD_NATIVE_WINDOW => "EGL_BAD_NATIVE_WINDOW",
        EGL_CONTEXT_LOST => "EGL_CONTEXT_LOST",
        _ => panic!(),
    }
}
