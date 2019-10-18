use libGLESv3_sys::{
    GLenum, GLint, GLsizei, GLuint, GL_CLAMP_TO_EDGE, GL_COLOR_ATTACHMENT0, GL_DEPTH_ATTACHMENT,
    GL_DEPTH_COMPONENT24, GL_DRAW_FRAMEBUFFER, GL_FRAMEBUFFER_COMPLETE,
    GL_FRAMEBUFFER_INCOMPLETE_ATTACHMENT, GL_FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT,
    GL_FRAMEBUFFER_INCOMPLETE_MULTISAMPLE, GL_FRAMEBUFFER_UNDEFINED, GL_FRAMEBUFFER_UNSUPPORTED,
    GL_LINEAR, GL_RENDERBUFFER, GL_RGBA8, GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER,
    GL_TEXTURE_MIN_FILTER, GL_TEXTURE_WRAP_S, GL_TEXTURE_WRAP_T,
};
use libvrapi_sys::{ovrTextureSwapChain, ovrTextureType__VRAPI_TEXTURE_TYPE_2D};

pub struct SwapChain {
    length: GLsizei,
    width: GLsizei,
    height: GLsizei,
    color_swap_chain: *mut ovrTextureSwapChain,
    depth_buffers: Vec<GLuint>,
    frame_buffers: Vec<GLuint>,
    index: GLsizei,
}

impl SwapChain {
    pub fn new(width: GLsizei, height: GLsizei) -> SwapChain {
        unsafe {
            logi!("create color swap chain");
            let color_swap_chain = libvrapi_sys::vrapi_CreateTextureSwapChain3(
                ovrTextureType__VRAPI_TEXTURE_TYPE_2D,
                GL_RGBA8 as i64,
                width,
                height,
                1,
                3,
            );
            if color_swap_chain.is_null() {
                panic!("can't create color swap chain");
            }

            let length = libvrapi_sys::vrapi_GetTextureSwapChainLength(color_swap_chain) as GLsizei;

            for index in 0..length {
                logv!("initialize color texture {}", index);
                libGLESv3_sys::glBindTexture(
                    GL_TEXTURE_2D,
                    libvrapi_sys::vrapi_GetTextureSwapChainHandle(color_swap_chain, index),
                );
                libGLESv3_sys::glTexParameteri(
                    GL_TEXTURE_2D,
                    GL_TEXTURE_MIN_FILTER,
                    GL_LINEAR as GLint,
                );
                libGLESv3_sys::glTexParameteri(
                    GL_TEXTURE_2D,
                    GL_TEXTURE_MAG_FILTER,
                    GL_LINEAR as GLint,
                );
                libGLESv3_sys::glTexParameteri(
                    GL_TEXTURE_2D,
                    GL_TEXTURE_WRAP_S,
                    GL_CLAMP_TO_EDGE as GLint,
                );
                libGLESv3_sys::glTexParameteri(
                    GL_TEXTURE_2D,
                    GL_TEXTURE_WRAP_T,
                    GL_CLAMP_TO_EDGE as GLint,
                );
                libGLESv3_sys::glBindTexture(GL_TEXTURE_2D, 0);
            }

            logi!("generate depth buffers");
            let mut depth_buffers = Vec::with_capacity(length as usize);
            libGLESv3_sys::glGenRenderbuffers(length, depth_buffers.as_mut_ptr());
            depth_buffers.set_len(length as usize);
            for index in 0..length {
                logv!("initialize depth buffer {}", index);
                libGLESv3_sys::glBindRenderbuffer(GL_RENDERBUFFER, depth_buffers[index as usize]);
                libGLESv3_sys::glRenderbufferStorage(
                    GL_RENDERBUFFER,
                    GL_DEPTH_COMPONENT24,
                    width,
                    height,
                );
                libGLESv3_sys::glBindRenderbuffer(GL_RENDERBUFFER, 0);
            }

            logi!("generate frame buffers");
            let mut frame_buffers = Vec::with_capacity(length as usize);
            libGLESv3_sys::glGenFramebuffers(length, frame_buffers.as_mut_ptr());
            frame_buffers.set_len(length as usize);
            for index in 0..length {
                logv!("initialize frame buffer {}", index);
                libGLESv3_sys::glBindFramebuffer(
                    GL_DRAW_FRAMEBUFFER,
                    frame_buffers[index as usize],
                );
                libGLESv3_sys::glFramebufferTexture2D(
                    GL_DRAW_FRAMEBUFFER,
                    GL_COLOR_ATTACHMENT0,
                    GL_TEXTURE_2D,
                    libvrapi_sys::vrapi_GetTextureSwapChainHandle(color_swap_chain, index),
                    0,
                );
                libGLESv3_sys::glFramebufferRenderbuffer(
                    GL_DRAW_FRAMEBUFFER,
                    GL_DEPTH_ATTACHMENT,
                    GL_RENDERBUFFER,
                    depth_buffers[index as usize],
                );
                let status = libGLESv3_sys::glCheckFramebufferStatus(GL_DRAW_FRAMEBUFFER);
                if status != GL_FRAMEBUFFER_COMPLETE {
                    panic!(
                        "can't initialize framebuffer {}: {}",
                        index,
                        get_framebuffer_status_string(status)
                    );
                }
                libGLESv3_sys::glBindFramebuffer(GL_DRAW_FRAMEBUFFER, 0);
            }

            SwapChain {
                length,
                width,
                height,
                color_swap_chain,
                depth_buffers,
                frame_buffers,
                index: 0,
            }
        }
    }

    pub fn width(&self) -> GLsizei {
        self.width
    }

    pub fn height(&self) -> GLsizei {
        self.height
    }

    pub fn color_swap_chain(&self) -> *mut ovrTextureSwapChain {
        self.color_swap_chain
    }

    pub fn frame_buffer(&self, index: usize) -> GLuint {
        self.frame_buffers[index]
    }

    pub fn index(&self) -> GLsizei {
        self.index
    }

    pub fn advance(&mut self) {
        self.index = (self.index + 1) % self.length;
    }
}

impl Drop for SwapChain {
    fn drop(&mut self) {
        unsafe {
            logi!("delete frame buffers");
            libGLESv3_sys::glDeleteFramebuffers(self.length, self.frame_buffers.as_ptr());

            logi!("delete depth buffers");
            libGLESv3_sys::glDeleteRenderbuffers(self.length, self.depth_buffers.as_ptr());

            logi!("destroy color swap chain");
            libvrapi_sys::vrapi_DestroyTextureSwapChain(self.color_swap_chain);
        }
    }
}

fn get_framebuffer_status_string(status: GLenum) -> &'static str {
    match status as u32 {
        GL_FRAMEBUFFER_UNDEFINED => "GL_FRAMEBUFFER_UNDEFINED",
        GL_FRAMEBUFFER_INCOMPLETE_ATTACHMENT => "GL_FRAMEBUFFER_INCOMPLETE_ATTACHMENT",
        GL_FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT => {
            "GL_FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT"
        }
        GL_FRAMEBUFFER_UNSUPPORTED => "GL_FRAMEBUFFER_UNSUPPORTED",
        GL_FRAMEBUFFER_INCOMPLETE_MULTISAMPLE => "GL_FRAMEBUFFER_INCOMPLETE_MULTISAMPLE",
        _ => panic!(),
    }
}
