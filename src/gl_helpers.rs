#![allow(unused_imports)]

use std::mem;

use gles;
use gles::es20::data_struct::*;
use gles::es20::wrapper::*;
use gles::es20::ffi::*;

use super::{Blending, CullFace, Depth, DrawMode, Error, IndexKind};

#[inline]
pub fn gl_set_defaults() {
    unsafe {
        glFrontFace(GL_CCW as GLenum);
//        front_face(GL_CCW);

//        if gl::PixelStorei::is_loaded() {
            glPixelStorei(GL_UNPACK_ALIGNMENT, 1);
//        }

//            pixel_storei(GL_UNPACK_ALIGNMENT, 1);

    }

    gl_set_viewport(0, 0, 1, 1);

    gl_set_clear_depth(1.0);
    gl_set_clear_stencil(0);

    gl_set_depth_write(true);
    gl_set_depth_range(0.0, 1.0);
    gl_set_line_width(1.0);

    gl_set_blending(Blending::Default);
    gl_set_cull_face(CullFace::Back);
    gl_set_depth_func(Depth::LessThanOrEqual);

    gl_set_clear_color(&[0.0, 0.0, 0.0, 1.0]);
    gl_clear(true, true, true);
}

#[inline(always)]
pub fn gl_get_error() -> Error {
    unsafe {
//        gl::GetError().into()
        gl_get_error().into()
    }
}

#[inline(always)]
pub fn gl_set_viewport(x: usize, y: usize, width: usize, height: usize) {
    unsafe {
        glViewport(x as GLint, y as GLint, width as GLsizei, height as GLsizei);
//        viewport(x, y, width, height);
    }
}

#[inline(always)]
pub fn gl_set_clear_depth(clear_depth: f64) {
    unsafe {
//        if gl::ClearDepthf::is_loaded() {
//            gl::ClearDepthf(clear_depth as f32);
//        } else if gl::ClearDepth::is_loaded() {
            glClearDepthf(clear_depth as GLclampf);
//        }
//        clear_depthf(clear_depth);
    }
}

#[inline(always)]
pub fn gl_set_clear_stencil(clear_stencil_i: isize) {
    unsafe {
        glClearStencil(clear_stencil as GLint);
//        clear_stencil(clear_stencil_i);
    }
}

#[inline(always)]
pub fn gl_set_depth_write(depth_write: bool) {
    unsafe {
        glDepthMask(if depth_write { GL_TRUE } else { GL_FALSE });
//        depth_mask(
//            if depth_write {
//                GL_TRUE
//            } else {
//                GL_FALSE
//            }
//        )
    }
}

#[inline(always)]
pub fn gl_set_depth_range(near: f64, far: f64) {
    unsafe {
//        if gl::DepthRangef::is_loaded() {
//            gl::DepthRangef(near as f32, far as f32);
//        } else if gl::DepthRange::is_loaded() {
            glDepthRangef(near as GLclampf, far as GLclampf);
//        }
//        depth_rangef(near, far);
    }
}

#[inline(always)]
pub fn gl_set_line_width(line_width_f: f32) {
    unsafe {
//        gl::LineWidth(line_width as GLfloat);
        line_width(line_width_f)
    }
}

#[inline]
pub fn gl_set_blending(blending: Blending) {
    match &blending {
        &Blending::Additive => unsafe {
            enable(GL_BLEND);
            blend_equation(GL_FUNC_ADD);
            blend_func(GL_SRC_ALPHA, GL_ONE);
        },
        &Blending::Subtractive => unsafe {
            enable(GL_BLEND);
            blend_equation(GL_FUNC_ADD);
            blend_func(GL_ZERO, GL_ONE_MINUS_SRC_COLOR);
        },
        &Blending::Multiply => unsafe {
            enable(GL_BLEND);
            blend_equation(GL_FUNC_ADD);
            blend_func(GL_ZERO, GL_SRC_COLOR);
        },
        &Blending::Default => unsafe {
            enable(GL_BLEND);
            blend_equation_separate(GL_FUNC_ADD, GL_FUNC_ADD);
            blend_func_separate(
                GL_SRC_ALPHA,
                GL_ONE_MINUS_SRC_ALPHA,
                GL_ONE,
                GL_ONE_MINUS_SRC_ALPHA,
            );
        },
        &Blending::None => unsafe {
            disable(GL_BLEND);
        },
    }
}

#[inline]
pub fn gl_set_cull_face(cull_face: CullFace) {
    match &cull_face {
        &CullFace::Back => unsafe {
            glEnable(GL_CULL_FACE);
            glCullFace(GL_BACK);
        },
        &CullFace::Front => unsafe {
            glEnable(GL_CULL_FACE);
            glCullFace(GL_FRONT);
        },
        &CullFace::FrontAndBack => unsafe {
            glEnable(GL_CULL_FACE);
            glCullFace(GL_FRONT_AND_BACK);
        },
        &CullFace::None => unsafe {
            glDisable(GL_CULL_FACE);
        },
    }
}

#[inline]
pub fn gl_set_depth_func(depth_func: Depth) {
    match &depth_func {
        &Depth::Never => unsafe {
            glEnable(GL_DEPTH_TEST);
            glDepthFunc(GL_NEVER);
        },
        &Depth::LessThan => unsafe {
            glEnable(GL_DEPTH_TEST);
            glDepthFunc(GL_LESS);
        },
        &Depth::Equal => unsafe {
            glEnable(GL_DEPTH_TEST);
            glDepthFunc(GL_EQUAL);
        },
        &Depth::LessThanOrEqual => unsafe {
            glEnable(GL_DEPTH_TEST);
            glDepthFunc(GL_LEQUAL);
        },
        &Depth::GreaterThan => unsafe {
            glEnable(GL_DEPTH_TEST);
            glDepthFunc(GL_GREATER);
        },
        &Depth::NotEqual => unsafe {
            glEnable(GL_DEPTH_TEST);
            glDepthFunc(GL_NOTEQUAL);
        },
        &Depth::GreaterThanOrEqual => unsafe {
            glEnable(GL_DEPTH_TEST);
            glDepthFunc(GL_GEQUAL);
        },
        &Depth::Always => unsafe {
            glEnable(GL_DEPTH_TEST);
            glDepthFunc(GL_ALWAYS);
        },
        &Depth::None => unsafe {
            glDisable(GL_DEPTH_TEST);
        },
    }
}

#[inline(always)]
pub fn gl_set_clear_color(color: &[f32; 4]) {
    unsafe {
        clear_color(color[0], color[1], color[2], color[3]);
    }
}

#[inline]
pub fn gl_clear(color: bool, depth: bool, stencil: bool) {
    let mut bits: GLbitfield = 0;

    if color {
        bits = bits | GL_COLOR_BUFFER_BIT;
    }
    if depth {
        bits = bits | GL_DEPTH_BUFFER_BIT;
    }
    if stencil {
        bits = bits | GL_STENCIL_BUFFER_BIT;
    }

    gl_clear_bits(bits);
}

#[inline(always)]
pub fn gl_clear_bits(bits: GLbitfield) {
    unsafe {
        clear(bits);
    }
}

#[inline]
pub fn gl_draw_arrays(mode: DrawMode, first: usize, count: usize) {
    unsafe {
        glDrawArrays(mode.into(), first as GLint, count as GLsizei);
    }
}
#[inline]
pub fn gl_draw_elements(mode: DrawMode, count: usize, kind: IndexKind, offset: GLint) {
    unsafe {
        glDrawElements(
            mode.into(),
            count as GLint,
            kind.into(),
            mem::transmute(offset as usize),
        );
    }
}
