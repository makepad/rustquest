use crate::{Geometry, Program, SwapChain, EGL};
use jni::sys::{jobject, JavaVM};
use libGLESv3_sys::{
    GLboolean, GLsizei, GL_COLOR_BUFFER_BIT, GL_CULL_FACE, GL_DEPTH_ATTACHMENT,
    GL_DEPTH_BUFFER_BIT, GL_DEPTH_TEST, GL_DRAW_FRAMEBUFFER, GL_FALSE, GL_SCISSOR_TEST,
    GL_TRIANGLES, GL_UNSIGNED_SHORT,
};
use libandroid_sys::ANativeWindow;
use libvrapi_sys::{
    ovrControllerType__ovrControllerType_TrackedRemote,
    ovrFrameLayerFlags__VRAPI_FRAME_LAYER_FLAG_CHROMATIC_ABERRATION_CORRECTION,
    ovrInitializeStatus__VRAPI_INITIALIZE_SUCCESS, ovrInputCapabilityHeader, ovrJava, ovrMobile,
    ovrModeFlags__VRAPI_MODE_FLAG_NATIVE_WINDOW,
    ovrModeFlags__VRAPI_MODE_FLAG_RESET_WINDOW_FULLSCREEN, ovrSubmitFrameDescription2,
    ovrSystemProperty__VRAPI_SYS_PROP_SUGGESTED_EYE_TEXTURE_HEIGHT,
    ovrSystemProperty__VRAPI_SYS_PROP_SUGGESTED_EYE_TEXTURE_WIDTH, ovrTracking, ovrVector3f,
    ovrQuatf,
    ovrInputTrackedRemoteCapabilities,
    ovrControllerCapabilities__ovrControllerCaps_RightHand,
};
use std::mem;
use std::ptr;

pub struct App {
    vm: *mut JavaVM,
    java: ovrJava,
    egl: EGL,
    swap_chains: [SwapChain; 2],
    program: Program,
    geometry: Geometry,
    resumed: bool,
    window: *mut ANativeWindow,
    vr: *mut ovrMobile,
    frame_index: u64,
    position: ovrVector3f,
    orientation: ovrQuatf,
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
        let width = unsafe {
            libvrapi_sys::vrapi_GetSystemPropertyInt(
                &java,
                ovrSystemProperty__VRAPI_SYS_PROP_SUGGESTED_EYE_TEXTURE_WIDTH,
            )
        };
        let height = unsafe {
            libvrapi_sys::vrapi_GetSystemPropertyInt(
                &java,
                ovrSystemProperty__VRAPI_SYS_PROP_SUGGESTED_EYE_TEXTURE_HEIGHT,
            )
        };
        let egl = EGL::new();
        App {
            vm,
            java,
            egl,
            swap_chains: [SwapChain::new(width, height), SwapChain::new(width, height)],
            program: Program::new(),
            geometry: Geometry::new(),
            resumed: false,
            window: ptr::null_mut(),
            vr: ptr::null_mut(),
            frame_index: 0,
            position: ovrVector3f {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            orientation: ovrQuatf {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                w: 1.0,
            },
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

    pub fn handle_input(&mut self) {
        unsafe {
            for index in 0.. {
                logv!("enumerate input device {}", index);
                let mut header = mem::zeroed::<ovrInputCapabilityHeader>();
                if libvrapi_sys::vrapi_EnumerateInputDevices(self.vr, index, &mut header) < 0 {
                    break;
                }

                if header.Type == ovrControllerType__ovrControllerType_TrackedRemote {
                    logv!("found tracked remote");

                    logv!("get input device capabilities");
                    let mut capabilities = mem::zeroed::<ovrInputTrackedRemoteCapabilities>();
                    capabilities.Header = header;
                    libvrapi_sys::vrapi_GetInputDeviceCapabilities(self.vr, &mut capabilities.Header);

                    if capabilities.ControllerCapabilities & ovrControllerCapabilities__ovrControllerCaps_RightHand != 0 {
                        logv!("found right hand tracked remote");

                        logv!("get input tracking state");
                        let mut tracking = mem::zeroed::<ovrTracking>();
                        if libvrapi_sys::vrapi_GetInputTrackingState(
                            self.vr,
                            header.DeviceID,
                            0.0,
                            &mut tracking,
                        ) < 0
                        {
                            panic!("can't get input tracking state");
                        }

                        self.position = tracking.HeadPose.Pose.__bindgen_anon_1.Position;
                        self.orientation = tracking.HeadPose.Pose.Orientation;
                    }
                }
            }
        }
    }

    pub fn render_frame(&mut self) {
        unsafe {
            if self.vr.is_null() {
                return;
            }

            self.frame_index += 1;

            let translation = libvrapi_sys::ovrMatrix4f_CreateTranslation(
                self.position.x,
                self.position.y,
                self.position.z,
            );
            let rotation = libvrapi_sys::ovrMatrix4f_CreateFromQuaternion(&self.orientation);
            let mut model_matrix = rotation;
            model_matrix = libvrapi_sys::ovrMatrix4f_Multiply(&translation, &model_matrix);
            model_matrix = libvrapi_sys::ovrMatrix4f_Transpose(&model_matrix);

            logv!("get predicted display time");
            let display_time =
                libvrapi_sys::vrapi_GetPredictedDisplayTime(self.vr, self.frame_index as i64);

            logv!("get predicted tracking");
            let tracking = libvrapi_sys::vrapi_GetPredictedTracking2(self.vr, display_time);

            let mut layer = libvrapi_sys::vrapi_DefaultLayerProjection2();
            layer.Header.Flags =
                ovrFrameLayerFlags__VRAPI_FRAME_LAYER_FLAG_CHROMATIC_ABERRATION_CORRECTION;
            layer.HeadPose = tracking.HeadPose;
            for (index, swap_chain) in self.swap_chains.iter_mut().enumerate() {
                layer.Textures[index].ColorSwapChain = swap_chain.color_swap_chain();
                layer.Textures[index].SwapChainIndex = swap_chain.index();
                layer.Textures[index].TexCoordsFromTanAngles =
                    libvrapi_sys::ovrMatrix4f_TanAngleMatrixFromProjection(
                        &tracking.Eye[index].ProjectionMatrix,
                    );

                libGLESv3_sys::glBindFramebuffer(
                    GL_DRAW_FRAMEBUFFER,
                    swap_chain.frame_buffer(swap_chain.index() as usize),
                );

                libGLESv3_sys::glEnable(GL_CULL_FACE);
                libGLESv3_sys::glEnable(GL_DEPTH_TEST);
                libGLESv3_sys::glEnable(GL_SCISSOR_TEST);
                libGLESv3_sys::glClearColor(0.1, 0.1, 0.1, 0.0);
                libGLESv3_sys::glScissor(0, 0, swap_chain.width(), swap_chain.height());
                libGLESv3_sys::glViewport(0, 0, swap_chain.width(), swap_chain.height());
                libGLESv3_sys::glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
                libGLESv3_sys::glUseProgram(self.program.program());
                libGLESv3_sys::glUniformMatrix4fv(
                    self.program.uniform_location("uModelMatrix"),
                    1,
                    GL_FALSE as GLboolean,
                    model_matrix.M.as_ptr() as *const _,
                );
                let view_matrix =
                    libvrapi_sys::ovrMatrix4f_Transpose(&tracking.Eye[index].ViewMatrix);
                libGLESv3_sys::glUniformMatrix4fv(
                    self.program.uniform_location("uViewMatrix"),
                    1,
                    GL_FALSE as GLboolean,
                    view_matrix.M.as_ptr() as *const _,
                );
                let projection_matrix =
                    libvrapi_sys::ovrMatrix4f_Transpose(&tracking.Eye[index].ProjectionMatrix);
                libGLESv3_sys::glUniformMatrix4fv(
                    self.program.uniform_location("uProjectionMatrix"),
                    1,
                    GL_FALSE as GLboolean,
                    projection_matrix.M.as_ptr() as *const _,
                );
                libGLESv3_sys::glBindVertexArray(self.geometry.vertex_array());
                libGLESv3_sys::glDrawElements(
                    GL_TRIANGLES,
                    self.geometry.count(),
                    GL_UNSIGNED_SHORT,
                    ptr::null_mut(),
                );
                libGLESv3_sys::glBindVertexArray(0);
                libGLESv3_sys::glUseProgram(0);

                libGLESv3_sys::glClearColor(0.0, 0.0, 0.0, 1.0);
                libGLESv3_sys::glScissor(0, 0, 1, swap_chain.height());
                libGLESv3_sys::glClear(GL_COLOR_BUFFER_BIT);
                libGLESv3_sys::glScissor(swap_chain.width() - 1, 0, 1, swap_chain.height());
                libGLESv3_sys::glClear(GL_COLOR_BUFFER_BIT);
                libGLESv3_sys::glScissor(0, 0, swap_chain.width(), 1);
                libGLESv3_sys::glClear(GL_COLOR_BUFFER_BIT);
                libGLESv3_sys::glScissor(0, swap_chain.height() - 1, swap_chain.width(), 1);
                libGLESv3_sys::glClear(GL_COLOR_BUFFER_BIT);

                let attachments = [GL_DEPTH_ATTACHMENT];
                libGLESv3_sys::glInvalidateFramebuffer(
                    GL_DRAW_FRAMEBUFFER,
                    attachments.len() as GLsizei,
                    attachments.as_ptr(),
                );
                libGLESv3_sys::glFlush();
                libGLESv3_sys::glBindFramebuffer(GL_DRAW_FRAMEBUFFER, 0);
                swap_chain.advance();
            }

            logv!("submit frame");
            let layers = [&layer.Header as *const _];
            let mut frame = mem::zeroed::<ovrSubmitFrameDescription2>();
            frame.Flags = 0;
            frame.SwapInterval = 1;
            frame.FrameIndex = self.frame_index;
            frame.DisplayTime = display_time;
            frame.LayerCount = 1;
            frame.Layers = layers.as_ptr();
            libvrapi_sys::vrapi_SubmitFrame2(self.vr, &frame);
        }
    }

    fn update_vr_mode(&mut self) {
        if self.resumed && !self.window.is_null() {
            if self.vr.is_null() {
                unsafe {
                    logi!("enter vr mode");
                    let mut parms = libvrapi_sys::vrapi_DefaultModeParms(&self.java);
                    parms.Flags &= !ovrModeFlags__VRAPI_MODE_FLAG_RESET_WINDOW_FULLSCREEN;
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
