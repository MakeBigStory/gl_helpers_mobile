#![allow(unused_imports)]

use std::ops::{Deref, DerefMut};

use gles;
use gles::es20::data_struct::*;
use gles::es20::wrapper::*;
use gles::es30::ffi::*;

use super::{gl_major, GLAttribute, GLBuffer, GLVertex};

#[derive(Debug, Hash)]
pub struct GLVertexArray {
    id: GLuint,
    array: Vec<GLVertex>,
}

impl GLVertexArray {
    #[inline(always)]
    pub fn new() -> Self {
        GLVertexArray {
            id: {
                let mut id = 0;
                if gl_major() > 2 {
                    unsafe {
                        glGenVertexArrays(1, &mut id);
                    }
                }
                id
            },
            array: Vec::new(),
        }
    }

    #[inline]
    pub fn bind(&self) -> &Self {
        let gl_major = gl_major();

        if gl_major > 2 {
            unsafe {
                glBindVertexArray(self.id());
            }
        }

        self.enable_attributes();

        if gl_major < 3 {
            for gl_vertex in &self.array {
                gl_vertex.bind();
            }
        }

        self
    }
    #[inline]
    pub fn unbind(&self) -> &Self {
        if gl_major() > 2 {
            unsafe {
                glBindVertexArray(0);
            }
        }
        self.disable_attributes();
        self
    }

    #[inline]
    pub fn enable_attributes(&self) -> &Self {
        for gl_vertex in &self.array {
            gl_vertex.enable();
        }
        self
    }
    #[inline]
    pub fn disable_attributes(&self) -> &Self {
        for gl_vertex in &self.array {
            gl_vertex.disable();
        }
        self
    }

    #[inline]
    pub fn add_attribute(
        &mut self,
        gl_buffer: &GLBuffer,
        gl_attribute: &GLAttribute,
        offset: usize,
    ) -> &mut Self {
        let location = gl_attribute.location();
        let item_count = gl_attribute.item_count();
        let item_kind = gl_attribute.item_kind();
        let offset = offset * gl_buffer.kind_size();

        let gl_vertex = GLVertex::new(location, item_count, item_kind, gl_buffer.stride(), offset);
        gl_vertex.enable();
        gl_vertex.bind();
        self.array.push(gl_vertex);

        self
    }

    #[inline(always)]
    pub fn id(&self) -> GLuint {
        self.id
    }

    #[inline(always)]
    pub fn array(&self) -> &[GLVertex] {
        &*self.array
    }
    #[inline(always)]
    pub fn array_mut(&mut self) -> &mut [GLVertex] {
        &mut *self.array
    }
}

impl Deref for GLVertexArray {
    type Target = [GLVertex];

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        self.array()
    }
}

impl DerefMut for GLVertexArray {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.array_mut()
    }
}

impl Drop for GLVertexArray {
    #[inline]
    fn drop(&mut self) {
        if self.id != 0 {
            unsafe {
                glDeleteVertexArrays(1, &self.id);
            }
        }
    }
}
