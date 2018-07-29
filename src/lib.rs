//#![feature(const_atomic_usize_new)]

extern crate fnv;
extern crate opengles_rs as gles;
extern crate regex;
extern crate rand;

use rand::prelude::*;

mod gl_attribute;
mod gl_buffer;
mod gl_enums;
mod gl_framebuffer;
mod gl_helpers;
mod gl_info;
mod gl_program;
mod gl_renderbuffer;
mod gl_texture;
mod gl_uniform;
mod gl_vertex;
mod gl_vertex_array;

pub use self::gl_attribute::GLAttribute;
pub use self::gl_buffer::GLBuffer;
pub use self::gl_enums::*;
pub use self::gl_framebuffer::GLFramebuffer;
pub use self::gl_helpers::*;
pub use self::gl_info::*;
pub use self::gl_program::*;
pub use self::gl_renderbuffer::GLRenderbuffer;
pub use self::gl_texture::GLTexture;
pub use self::gl_uniform::GLUniform;
pub use self::gl_vertex::GLVertex;
pub use self::gl_vertex_array::GLVertexArray;

use gles::es20::data_struct::GLfloat;
use gles::es20::ffi::glFlush;


static SIMPLE_VERTEX_DATA: [GLfloat; 16] = [
    //   position     uv
    1f32,  1f32,   1f32, 1f32,
    -1f32,  1f32,   0f32, 1f32,
    1f32, -1f32,   1f32, 0f32,
    -1f32, -1f32,   0f32, 0f32
];

static SIMPLE_VERTEX: &'static str = "
#version 100
    uniform vec2 size;

    attribute vec2 position;
    attribute vec2 uv;

    varying vec2 v_uv;

    void main() {
        gl_Position = vec4(position * size * 0.5, 0, 1.0);
        v_uv = uv;
    }
";

#[cfg(any(target_os = "emscripten", target_os = "android"))]
static SIMPLE_FRAGMENT: &'static str = "
    #version 100
    precision mediump float;

    uniform float alpha;

    varying vec2 v_uv;

    void main() {
        gl_FragColor = vec4(v_uv, 1.0, alpha);
    }
";
#[cfg(not(any(target_os = "emscripten", target_os = "android")))]
static SIMPLE_FRAGMENT: &'static str = "
    #version 100
        precision mediump float;
    uniform float alpha;

    varying vec2 v_uv;

    void main() {
        gl_FragColor = vec4(v_uv, 1.0, alpha);
    }
";

#[repr(C)]
pub struct GLProgramWrapper {
    program: GLProgram,
    timestamp: f32,
}

#[no_mangle]
pub extern fn rust_opengl_backend_init() -> Box<GLProgramWrapper> {
    let gl_info = GLInfo::new();
    gl_set_defaults();


    let program = GLProgram::new(SIMPLE_VERTEX, SIMPLE_FRAGMENT);
    let program_wrapper = GLProgramWrapper{program: program, timestamp:0.1f32};

    println!("\n -------- rust_opengl_backend_init -----------------");

    Box::new(program_wrapper)
}


/*
uint8_t whatlang_detect(char* text, struct whatlang_info* info);

pub extern fn whatlang_detect(ptr: *const c_char, info: &mut Info) -> u8 {
    let cstr = unsafe { CStr::from_ptr(ptr) };

    match cstr.to_str() {
        Ok(s) => {
          // Here `s` is regular `&str` and we can work with it
        }
        Err(_) => {
          // handle the error
        }
    }
}
*/
#[no_mangle]
pub extern fn rust_opengl_backend_draw(wrapper: &mut GLProgramWrapper) {
    let program = &wrapper.program;
    program.bind();

    let buffer = GLBuffer::new(
        BufferTarget::Array,
        4,
        Usage::StaticDraw,
        &SIMPLE_VERTEX_DATA,
    );

    let mut vertex_array = GLVertexArray::new();

    vertex_array.bind();
    vertex_array.add_attribute(&buffer, program.get_attribute("position"), 0);
    vertex_array.add_attribute(&buffer, program.get_attribute("uv"), 2);

    vertex_array.enable_attributes();

    program.get_uniform("alpha").set_1f(0.5_f32);

    gl_set_viewport(0, 0, 750, 1334);

//    gl_set_clear_color(&[1.0, 0.0, 0.0, 1.0]);
    gl_clear(true, true, true);

    let mut size = [1f32; 2];
    size[0] = 1_f32 + ((wrapper.timestamp * 0.005_f32).cos() * 0.5_f32);
    size[1] = 1_f32 + ((wrapper.timestamp * 0.005_f32).sin() * 0.5_f32);
    program.get_uniform("size").set_2f(&size);

    program.get_uniform("alpha").set_1f(wrapper.timestamp);

    gl_draw_arrays(DrawMode::TriangleStrip, 0, 4);

    unsafe {
        glFlush();
    }

    wrapper.timestamp += 0.1;
    if wrapper.timestamp > 1.0 {
        wrapper.timestamp = 0.0;
    }
    println!("\n -------- over -----------------");
}

#[no_mangle]
pub extern fn init_env_rs() {
//    let mut screen_width = 1024_usize;
//    let mut screen_height = 768_usize;
//
//    let mut events_loop = glutin::EventsLoop::new();
//    let window = glutin::WindowBuilder::new()
//        .with_title("Simple")
//        .with_maximized(true);
//    let context = {
//        let builder = glutin::ContextBuilder::new().with_vsync(true);
//
//        if cfg!(target_os = "android") {
//            builder.with_gl(GlRequest::Specific(Api::OpenGlEs, (2, 0)))
//        } else {
//            builder
//        }
//    };
//    let gl_window = glutin::GlWindow::new(window, context, &events_loop).unwrap();
//
//    unsafe {
//        gl_window.make_current().unwrap();
//        gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);
//    }

    let gl_info = GLInfo::new();
//    println!("{}", gl_info.version());
//    println!(
//        "OpenGL version: {:?}.{:?}, GLSL version {:?}.{:?}, \nextensions = {:?}",
//        gl_info.major(),
//        gl_info.minor(),
//        gl_info.glsl_major(),
//        gl_info.glsl_minor(),
//        gl_info.extenstions(),
//    );

    gl_set_defaults();

//    if let Some((w, h)) = gl_window.get_inner_size() {
//        screen_width = w as usize;
//        screen_height = h as usize;
//        gl_window.resize(w, h);
//        gl_set_viewport(0, 0, screen_width, screen_height);
//    }

    let program = GLProgram::new(SIMPLE_VERTEX, SIMPLE_FRAGMENT);
    program.bind();

    let buffer = GLBuffer::new(
        BufferTarget::Array,
        4,
        Usage::StaticDraw,
        &SIMPLE_VERTEX_DATA,
    );

    let mut vertex_array = GLVertexArray::new();

    vertex_array.bind();
    vertex_array.add_attribute(&buffer, program.get_attribute("position"), 0);
    vertex_array.add_attribute(&buffer, program.get_attribute("uv"), 2);

    vertex_array.enable_attributes();

    program.get_uniform("alpha").set_1f(0.5_f32);

    let mut size = [1f32; 2];
    let ms = random::<f32>();

//    main_loop::glutin::run(&mut events_loop, move |events, ms_f64| {
//        let ms = ms_f64 as f32;
//
//        for event in events {
//            match event {
//                glutin::Event::WindowEvent { event, .. } => match event {
//                    glutin::WindowEvent::CloseRequested => return main_loop::ControlFlow::Break,
//                    glutin::WindowEvent::Resized(w, h) => {
//                        screen_width = w as usize;
//                        screen_height = h as usize;
//
//                        gl_window.resize(w, h);
                        gl_set_viewport(0, 0, 750, 1334);
//                    }
//                    _ => (),
//                },
//                _ => (),
//            }
//        }
//
        gl_set_clear_color(&[1.0, 0.0, 0.0, 1.0]);
        gl_clear(true, true, true);

        size[0] = 1_f32 + ((ms * 0.005_f32).cos() * 0.5_f32);
        size[1] = 1_f32 + ((ms * 0.005_f32).sin() * 0.5_f32);
        program.get_uniform("size").set_2f(&size);

        gl_draw_arrays(DrawMode::TriangleStrip, 0, 4);

    unsafe {
        glFlush();
    }
    println!("\n -------- over -----------------");

//        gl_window.swap_buffers().unwrap();
//
//        main_loop::ControlFlow::Continue
//    });
}