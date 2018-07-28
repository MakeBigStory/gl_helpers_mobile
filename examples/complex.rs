////#![feature(link_args)]
//
//extern crate gl;
//extern crate gl_helpers;
//extern crate glutin;
//extern crate main_loop;
//extern crate mat4;
//extern crate rand;
//
//use std::f32::consts::PI;
//
//use gl::types::*;
//use glutin::{Api, GlContext, GlRequest};
//
//use gl_helpers::*;
//use rand::{weak_rng, Rng};
//
//#[cfg(target_os = "emscripten")]
//#[allow(unused_attributes)]
//#[link_args = "-s OFFSCREENCANVAS_SUPPORT=1"]
//extern "C" {}
//
//static TO_RADS: f32 = PI / 180f32;
//
//const TEX_WIDTH: usize = 512;
//const TEX_HEIGHT: usize = TEX_WIDTH;
//const TEX_SIZE: usize = TEX_WIDTH * TEX_HEIGHT;
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
//    uniform mat4 projection;
//    uniform mat4 model_view;
//
//    uniform vec2 uv_offset;
//
//    attribute vec2 position;
//    attribute vec2 uv;
//
//    varying vec2 v_uv;
//
//    void main() {
//        gl_Position = projection * model_view * vec4(position, 0.0, 1.0);
//        v_uv = uv_offset + uv;
//    }
//";
//#[cfg(any(target_os = "emscripten", target_os = "android"))]
//static SIMPLE_FRAGMENT: &'static str = "
//    precision mediump float;
//
//    uniform sampler2D diffuse;
//
//    varying vec2 v_uv;
//
//    void main() {
//        gl_FragColor = texture2D(diffuse, v_uv);
//    }
//";
//#[cfg(not(any(target_os = "emscripten", target_os = "android")))]
//static SIMPLE_FRAGMENT: &'static str = "
//    uniform sampler2D diffuse;
//
//    varying vec2 v_uv;
//
//    void main() {
//        gl_FragColor = texture2D(diffuse, v_uv);
//    }
//";
//
//fn main() {
//    let mut screen_width = 1024_usize;
//    let mut screen_height = 768_usize;
//
//    let mut events_loop = glutin::EventsLoop::new();
//    let window = glutin::WindowBuilder::new()
//        .with_title("Complex")
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
//    let mut random = weak_rng();
//    let mut data = [0xffffffffu32; TEX_SIZE];
//
//    for i in 0..TEX_SIZE {
//        let r = (random.next_f32() * 256f32) as u32;
//        let c = (0xff000000) | (r << 16) | (r << 8) | r;
//        data[i] = c;
//    }
//
//    let texture = GLTexture::new_2d(
//        TEX_WIDTH,
//        TEX_HEIGHT,
//        InternalFormat::RGBA,
//        DataFormat::RGBA,
//        DataKind::UnsignedByte,
//        FilterMode::Linear,
//        Wrap::Repeat,
//        true,
//        &data,
//    );
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
//    let mut projection = mat4::new_identity::<f32>();
//    let mut model_view = mat4::new_identity::<f32>();
//    let mut camera = [0f32, 0f32, -5f32];
//    let mut uv_offset = [0f32, 0f32];
//    let mut color = [0f32, 0f32, 0f32, 1f32];
//
//    main_loop::glutin::run(&mut events_loop, move |events, ms_f64| {
//        let ms = ms_f64 as f32;
//
//        for event in events {
//            match event {
//                glutin::Event::WindowEvent {
//                    event: glutin::WindowEvent::CloseRequested,
//                    ..
//                } => {
//                    return main_loop::ControlFlow::Break;
//                }
//                glutin::Event::WindowEvent {
//                    event: glutin::WindowEvent::Resized(w, h),
//                    ..
//                } => {
//                    screen_width = w as usize;
//                    screen_height = h as usize;
//
//                    gl_window.resize(w, h);
//                    gl_set_viewport(0, 0, screen_width, screen_height);
//                    mat4::perspective(
//                        &mut projection,
//                        45f32 * TO_RADS,
//                        w as f32 / h as f32,
//                        0.01f32,
//                        1024f32,
//                    );
//                }
//                glutin::Event::WindowEvent {
//                    event: glutin::WindowEvent::CursorMoved { position, .. },
//                    ..
//                } => {
//                    let (x, y) = position;
//                    let x = x as f32;
//                    let y = y as f32;
//                    let w = screen_width as f32;
//                    let h = screen_height as f32;
//                    camera[0] = ((x - (w / 2f32)) / w) * 2f32;
//                    camera[1] = (((h / 2f32) - y) / w) * 2f32;
//                }
//                _ => (),
//            }
//        }
//
//        color[0] = (ms * 0.000001f32).cos();
//        color[1] = (ms * 0.0001f32).sin();
//        color[2] = (ms * 0.001f32).cos();
//
//        uv_offset[0] = (ms * 0.0001f32).sin() * 0.5f32;
//        uv_offset[1] = (ms * 0.0001f32).cos() * 0.5f32;
//
//        mat4::set_position(&mut model_view, &camera);
//
//        gl_set_clear_color(&color);
//        gl_clear(true, true, true);
//
//        program.bind();
//        vertex_array.bind();
//        vertex_array.enable_attributes();
//
//        program.get_uniform("diffuse").set_sampler_2d(&texture, 0);
//        program.get_uniform("uv_offset").set_2f(&uv_offset);
//        program.get_uniform("projection").set_mat4f(&projection);
//        program.get_uniform("model_view").set_mat4f(&model_view);
//
//        gl_draw_arrays(DrawMode::TriangleStrip, 0, 4);
//
//        gl_window.swap_buffers().unwrap();
//
//        main_loop::ControlFlow::Continue
//    });
//}
