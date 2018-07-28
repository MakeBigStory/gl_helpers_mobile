////#![feature(link_args)]
//
//extern crate gl;
//extern crate gl_helpers;
//extern crate glutin;
//extern crate main_loop;
//
//use gl::types::*;
//use glutin::{Api, GlContext, GlRequest};
//
//use gl_helpers::*;
//
//#[cfg(target_os = "emscripten")]
//#[allow(unused_attributes)]
//#[link_args = "-s OFFSCREENCANVAS_SUPPORT=1"]
//extern "C" {}
//
//static SIMPLE_VERTEX_DATA: [GLfloat; 16] = [
//    //   position     uv
//    1f32,
//    1f32,
//    1f32,
//    1f32,
//    -1f32,
//    1f32,
//    0f32,
//    1f32,
//    1f32,
//    -1f32,
//    1f32,
//    0f32,
//    -1f32,
//    -1f32,
//    0f32,
//    0f32,
//];
//
//static SIMPLE_VERTEX: &'static str = "
//#version 100
//    uniform vec2 size;
//
//    attribute vec2 position;
//    attribute vec2 uv;
//
//    varying vec2 v_uv;
//
//    void main() {
//        gl_Position = vec4(position * size * 0.5, 0, 1.0);
//        v_uv = uv;
//    }
//";
//
//#[cfg(any(target_os = "emscripten", target_os = "android"))]
//static SIMPLE_FRAGMENT: &'static str = "
//    #version 100
//    precision mediump float;
//
//    uniform float alpha;
//
//    varying vec2 v_uv;
//
//    void main() {
//        gl_FragColor = vec4(v_uv, 1.0, alpha);
//    }
//";
//#[cfg(not(any(target_os = "emscripten", target_os = "android")))]
//static SIMPLE_FRAGMENT: &'static str = "
//    #version 100
//        precision mediump float;
//    uniform float alpha;
//
//    varying vec2 v_uv;
//
//    void main() {
//        gl_FragColor = vec4(v_uv, 1.0, alpha);
//    }
//";
//
//fn main() {
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
//
//    let gl_info = GLInfo::new();
//    println!("{}", gl_info.version());
//    println!(
//        "OpenGL version: {:?}.{:?}, GLSL version {:?}.{:?}0",
//        gl_info.major(),
//        gl_info.minor(),
//        gl_info.glsl_major(),
//        gl_info.glsl_minor()
//    );
//
//    gl_set_defaults();
//
//    if let Some((w, h)) = gl_window.get_inner_size() {
//        screen_width = w as usize;
//        screen_height = h as usize;
//        gl_window.resize(w, h);
//        gl_set_viewport(0, 0, screen_width, screen_height);
//    }
//
//    let program = GLProgram::new(SIMPLE_VERTEX, SIMPLE_FRAGMENT);
//
//    let buffer = GLBuffer::new(
//        BufferTarget::Array,
//        4,
//        Usage::StaticDraw,
//        &SIMPLE_VERTEX_DATA,
//    );
//
//    let mut vertex_array = GLVertexArray::new();
//
//    vertex_array.bind();
//    vertex_array.add_attribute(&buffer, program.get_attribute("position"), 0);
//    vertex_array.add_attribute(&buffer, program.get_attribute("uv"), 2);
//
//    vertex_array.enable_attributes();
//
//    program.get_uniform("alpha").set_1f(0.5_f32);
//
//    let mut size = [1f32; 2];
//
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
//                        gl_set_viewport(0, 0, screen_width, screen_height);
//                    }
//                    _ => (),
//                },
//                _ => (),
//            }
//        }
//
//        gl_set_clear_color(&[0.3, 0.3, 0.3, 1.0]);
//        gl_clear(true, true, true);
//
//        size[0] = 1_f32 + ((ms * 0.005_f32).cos() * 0.5_f32);
//        size[1] = 1_f32 + ((ms * 0.005_f32).sin() * 0.5_f32);
//        program.get_uniform("size").set_2f(&size);
//
//        gl_draw_arrays(DrawMode::TriangleStrip, 0, 4);
//        gl_window.swap_buffers().unwrap();
//
//        main_loop::ControlFlow::Continue
//    });
//}
