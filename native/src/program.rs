use libGLESv3_sys::{
    GLenum, GLint, GLuint, GL_COMPILE_STATUS, GL_FALSE, GL_FRAGMENT_SHADER, GL_INFO_LOG_LENGTH,
    GL_LINK_STATUS, GL_VERTEX_SHADER,
};
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::ptr;

const VERTEX_SHADER: &'static str = r#"
    #version 300 es

    uniform mat4 uModelMatrix;
    uniform mat4 uViewMatrix;
    uniform mat4 uProjectionMatrix;

    in vec3 aPosition;
    in vec3 aColor;

    out vec3 vColor;

    void main() {
        gl_Position = uProjectionMatrix * (uViewMatrix * (uModelMatrix * vec4(aPosition, 1.0)));
        vColor = aColor;
    }
"#;

const FRAGMENT_SHADER: &'static str = r#"
    #version 300 es

    in lowp vec3 vColor;

    out lowp vec4 fragColor;

    void main() {
        fragColor = vec4(vColor, 1.0);
    }
"#;

pub struct Program {
    program: GLuint,
    vertex_shader: GLuint,
    fragment_shader: GLuint,
    uniform_locations: HashMap<String, GLint>,
}

impl Program {
    pub fn new() -> Program {
        unsafe {
            let vertex_shader = compile_shader(GL_VERTEX_SHADER, VERTEX_SHADER);
            let fragment_shader = compile_shader(GL_FRAGMENT_SHADER, FRAGMENT_SHADER);

            logi!("link program");
            let program = libGLESv3_sys::glCreateProgram();
            libGLESv3_sys::glAttachShader(program, fragment_shader);
            libGLESv3_sys::glAttachShader(program, vertex_shader);
            let attrib_names = ["aPosition", "aColor"];
            for (index, name) in attrib_names.iter().cloned().enumerate() {
                libGLESv3_sys::glBindAttribLocation(
                    program,
                    index as GLuint,
                    CString::new(name).unwrap().as_ptr(),
                );
            }
            libGLESv3_sys::glLinkProgram(program);
            let mut status = 0;
            libGLESv3_sys::glGetProgramiv(program, GL_LINK_STATUS, &mut status);
            if status == GL_FALSE as GLint {
                let mut length = 0;
                libGLESv3_sys::glGetProgramiv(program, GL_INFO_LOG_LENGTH, &mut length);
                let mut log = Vec::with_capacity(length as usize);
                libGLESv3_sys::glGetProgramInfoLog(
                    program,
                    length,
                    ptr::null_mut(),
                    log.as_mut_ptr(),
                );
                log.set_len(length as usize);
                panic!(
                    "can't link program: {}",
                    CStr::from_ptr(log.as_ptr()).to_str().unwrap()
                );
            }

            logi!("get uniform locations");
            let mut uniform_locations = HashMap::new();
            let uniform_names = ["uModelMatrix", "uViewMatrix", "uProjectionMatrix"];
            for name in uniform_names.iter().cloned() {
                uniform_locations.insert(
                    String::from(name),
                    libGLESv3_sys::glGetUniformLocation(
                        program,
                        CString::new(name).unwrap().as_ptr(),
                    ),
                );
            }

            Program {
                program,
                vertex_shader,
                fragment_shader,
                uniform_locations,
            }
        }
    }

    pub fn program(&self) -> GLuint {
        self.program
    }

    pub fn uniform_location(&self, name: &str) -> GLint {
        self.uniform_locations.get(name).cloned().unwrap_or(-1)
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            logi!("delete program");
            libGLESv3_sys::glDeleteProgram(self.program);

            logi!("delete fragment shader");
            libGLESv3_sys::glDeleteProgram(self.fragment_shader);

            logi!("delete vertex shader");
            libGLESv3_sys::glDeleteProgram(self.vertex_shader);
        }
    }
}

unsafe fn compile_shader(type_: GLenum, string: &str) -> GLuint {
    logi!("compile shader");
    let shader = libGLESv3_sys::glCreateShader(type_);
    let string = CString::new(string).unwrap();
    let strings = [string.as_ptr()];
    libGLESv3_sys::glShaderSource(
        shader,
        1,
        strings.as_ptr(),
        ptr::null_mut(),
    );
    libGLESv3_sys::glCompileShader(shader);
    let mut status = 0;
    libGLESv3_sys::glGetShaderiv(shader, GL_COMPILE_STATUS, &mut status);
    if status == GL_FALSE as GLint {
        let mut length = 0;
        libGLESv3_sys::glGetShaderiv(shader, GL_INFO_LOG_LENGTH, &mut length);
        let mut log = Vec::with_capacity(length as usize);
        libGLESv3_sys::glGetShaderInfoLog(shader, length, ptr::null_mut(), log.as_mut_ptr());
        log.set_len(length as usize);
        panic!(
            "can't compile shader: {}",
            CStr::from_ptr(log.as_ptr()).to_str().unwrap()
        );
    }
    shader
}
