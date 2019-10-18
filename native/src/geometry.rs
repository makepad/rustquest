use libGLESv3_sys::{
    GLboolean, GLenum, GLint, GLsizei, GLsizeiptr, GLushort, GLuint, GLvoid, GL_ARRAY_BUFFER,
    GL_ELEMENT_ARRAY_BUFFER, GL_FALSE, GL_FLOAT, GL_STATIC_DRAW,
};
use std::mem;

const VERTICES: &'static [Vertex] = &[
    Vertex {
        position: [-1.0, 1.0, -1.0],
        color: [1.0, 0.0, 1.0],
    },
    Vertex {
        position: [1.0, 1.0, -1.0],
        color: [0.0, 1.0, 0.0],
    },
    Vertex {
        position: [1.0, 1.0, 1.0],
        color: [0.0, 0.0, 1.0],
    },
    Vertex {
        position: [-1.0, 1.0, 1.0],
        color: [1.0, 0.0, 0.0],
    },
    Vertex {
        position: [-1.0, -1.0, -1.0],
        color: [0.0, 0.0, 1.0],
    },
    Vertex {
        position: [-1.0, -1.0, 1.0],
        color: [0.0, 1.0, 0.0],
    },
    Vertex {
        position: [1.0, -1.0, 1.0],
        color: [1.0, 0.0, 1.0],
    },
    Vertex {
        position: [1.0, -1.0, -1.0],
        color: [1.0, 0.0, 0.0],
    },
];

const INDICES: &'static [GLushort] = &[
    0, 2, 1, 2, 0, 3, 4, 6, 5, 6, 4, 7, 2, 6, 7, 7, 1, 2, 0, 4, 5, 5, 3, 0, 3, 5, 6, 6, 2, 3, 0, 1,
    7, 7, 4, 0,
];

pub struct Geometry {
    vertex_array: GLuint,
    vertex_buffer: GLuint,
    index_buffer: GLuint,
}

impl Geometry {
    pub fn new() -> Geometry {
        unsafe {
            logi!("generate vertex buffer");
            let mut vertex_buffer = 0;
            libGLESv3_sys::glGenBuffers(1, &mut vertex_buffer);
            libGLESv3_sys::glBindBuffer(GL_ARRAY_BUFFER, vertex_buffer);
            libGLESv3_sys::glBufferData(
                GL_ARRAY_BUFFER,
                (VERTICES.len() * mem::size_of::<Vertex>()) as GLsizeiptr,
                VERTICES.as_ptr() as *const _,
                GL_STATIC_DRAW,
            );
            libGLESv3_sys::glBindBuffer(GL_ARRAY_BUFFER, 0);

            logi!("generate index buffer");
            let mut index_buffer = 0;
            libGLESv3_sys::glGenBuffers(1, &mut index_buffer);
            libGLESv3_sys::glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, index_buffer);
            libGLESv3_sys::glBufferData(
                GL_ELEMENT_ARRAY_BUFFER,
                (INDICES.len() * mem::size_of::<GLushort>()) as GLsizeiptr,
                INDICES.as_ptr() as *const _,
                GL_STATIC_DRAW,
            );
            libGLESv3_sys::glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, 0);

            logi!("generate vertex array");
            let mut vertex_array = 0;
            libGLESv3_sys::glGenVertexArrays(1, &mut vertex_array);

            logi!("record vertex array");
            libGLESv3_sys::glBindVertexArray(vertex_array);
            libGLESv3_sys::glBindBuffer(GL_ARRAY_BUFFER, vertex_buffer);
            let attrib_pointers = [
                AttribPointer {
                    size: 3,
                    type_: GL_FLOAT,
                    normalized: GL_FALSE as GLboolean,
                    stride: 24,
                    pointer: 0 as *mut GLvoid,
                },
                AttribPointer {
                    size: 3,
                    type_: GL_FLOAT,
                    normalized: GL_FALSE as GLboolean,
                    stride: 24,
                    pointer: 12 as *mut GLvoid,
                },
            ];
            for (index, attrib_pointer) in attrib_pointers.iter().enumerate() {
                libGLESv3_sys::glVertexAttribPointer(
                    index as GLuint,
                    attrib_pointer.size,
                    attrib_pointer.type_,
                    attrib_pointer.normalized,
                    attrib_pointer.stride,
                    attrib_pointer.pointer,
                );
                libGLESv3_sys::glEnableVertexAttribArray(index as GLuint);
            }
            libGLESv3_sys::glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, index_buffer);
            libGLESv3_sys::glBindVertexArray(0);

            Geometry {
                vertex_buffer,
                index_buffer,
                vertex_array,
            }
        }
    }

    pub fn count(&self) -> GLsizei {
        INDICES.len() as GLsizei
    }

    pub fn vertex_array(&self) -> GLuint {
        self.vertex_array
    }
}

impl Drop for Geometry {
    fn drop(&mut self) {
        unsafe {
            logi!("delete vertex array");
            libGLESv3_sys::glDeleteVertexArrays(1, &self.vertex_array);

            logi!("delete index buffer");
            libGLESv3_sys::glDeleteBuffers(1, &self.index_buffer);

            logi!("delete vertex_buffer");
            libGLESv3_sys::glDeleteBuffers(1, &self.vertex_buffer);
        }
    }
}

struct AttribPointer {
    size: GLint,
    type_: GLenum,
    normalized: GLboolean,
    stride: GLsizei,
    pointer: *const GLvoid,
}

#[repr(C)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}
