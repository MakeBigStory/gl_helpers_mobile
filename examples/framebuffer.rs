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
//static FB_DATA: [GLfloat; 16] = [
//    // position     uv
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
//static TR_DATA: [GLfloat; 6] = [0.0, 0.5, -0.5, -0.5, 0.5, -0.5];
//
//static FB_VERTEX: &'static str = "
//    attribute vec2 position;
//    attribute vec2 uv;
//
//    varying vec2 v_uv;
//
//    void main() {
//        gl_Position = vec4(position, 0.0, 1.0);
//        v_uv = uv;
//    }
//";
//#[cfg(any(target_os = "emscripten", target_os = "android"))]
//static FB_FRAGMENT: &'static str = "
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
//static FB_FRAGMENT: &'static str = "
//    uniform sampler2D diffuse;
//
//    varying vec2 v_uv;
//
//    void main() {
//        gl_FragColor = texture2D(diffuse, v_uv);
//    }
//";
//
//static TR_VERTEX: &'static str = "
//    uniform vec2 offset;
//
//    attribute vec2 position;
//
//    void main() {
//        gl_Position = vec4(position + offset, 0, 1.0);
//    }
//";
//#[cfg(any(target_os = "emscripten", target_os = "android"))]
//static TR_FRAGMENT: &'static str = "
//    precision mediump float;
//
//    void main() {
//        gl_FragColor = vec4(1.0, 1.0, 1.0, 1.0);
//    }
//";
//#[cfg(not(any(target_os = "emscripten", target_os = "android")))]
//static TR_FRAGMENT: &'static str = "
//    void main() {
//        gl_FragColor = vec4(1.0, 1.0, 1.0, 1.0);
//    }
//";
//
//fn main() {
//    let mut screen_width = 1024_usize;
//    let mut screen_height = 768_usize;
//
//    let mut events_loop = glutin::EventsLoop::new();
//    let window = glutin::WindowBuilder::new()
//        .with_title("Framebuffer")
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
//    static SIZE: usize = 8_usize;
//    let fb_program = GLProgram::new(FB_VERTEX, FB_FRAGMENT);
//
//    let mut fb_texture = GLTexture::new_null_2d(
//        screen_width / SIZE,
//        screen_height / SIZE,
//        InternalFormat::RGBA,
//        DataFormat::RGBA,
//        DataKind::UnsignedByte,
//        FilterMode::None,
//        Wrap::Clamp,
//        false,
//    );
//
//    let fb_framebuffer = GLFramebuffer::new(&fb_texture, &[Attachment::Color(0)], 0);
//    let fb_renderbuffer = GLRenderbuffer::new(
//        InternalFormat::DepthComponent16,
//        Attachment::Depth,
//        screen_width / SIZE,
//        screen_height / SIZE,
//    );
//
//    let fb_buffer = GLBuffer::new(BufferTarget::Array, 4, Usage::StaticDraw, &FB_DATA);
//
//    let mut fb_vertex_array = GLVertexArray::new();
//    fb_vertex_array.bind();
//    fb_buffer.bind();
//    fb_vertex_array.add_attribute(&fb_buffer, fb_program.get_attribute("position"), 0);
//    fb_vertex_array.add_attribute(&fb_buffer, fb_program.get_attribute("uv"), 2);
//
//    let tr_program = GLProgram::new(TR_VERTEX, TR_FRAGMENT);
//
//    let tr_buffer = GLBuffer::new(BufferTarget::Array, 0, Usage::StaticDraw, &TR_DATA);
//
//    let mut tr_vertex_array = GLVertexArray::new();
//    tr_vertex_array.bind();
//    tr_buffer.bind();
//    tr_vertex_array.add_attribute(&tr_buffer, tr_program.get_attribute("position"), 0);
//
//    let mut offset = [1f32; 2];
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
//                    gl_window.resize(w, h);
//
//                    screen_width = w as usize;
//                    screen_height = h as usize;
//
//                    fb_texture.resize_null_2d(screen_width / SIZE, screen_height / SIZE);
//                    fb_framebuffer.set(&fb_texture, &[Attachment::Color(0)], 0);
//                    fb_renderbuffer.set(
//                        InternalFormat::DepthComponent16,
//                        Attachment::Depth,
//                        screen_width / SIZE,
//                        screen_height / SIZE,
//                    );
//                }
//                _ => (),
//            }
//        }
//
//        fb_framebuffer.bind();
//        fb_renderbuffer.bind();
//
//        gl_set_viewport(0, 0, screen_width / SIZE, screen_height / SIZE);
//        gl_set_clear_color(&[0.3, 0.3, 0.3, 1.0]);
//        gl_clear(true, true, true);
//
//        tr_program.bind();
//
//        tr_vertex_array.bind();
//        tr_vertex_array.enable_attributes();
//
//        offset[0] = (ms * 0.001_f32).cos() * 0.5_f32;
//        offset[1] = (ms * 0.001_f32).sin() * 0.5_f32;
//
//        tr_program.get_uniform("offset").set_2f(&offset);
//
//        gl_draw_arrays(DrawMode::Triangles, 0, 3);
//
//        fb_framebuffer.unbind();
//        fb_renderbuffer.unbind();
//
//        gl_set_viewport(0, 0, screen_width, screen_height);
//        gl_set_clear_color(&[0.3, 0.3, 0.3, 1.0]);
//        gl_clear(true, true, true);
//
//        fb_program.bind();
//        fb_vertex_array.bind();
//        fb_vertex_array.enable_attributes();
//
//        fb_program
//            .get_uniform("diffuse")
//            .set_sampler_2d(&fb_texture, 0);
//
//        gl_draw_arrays(DrawMode::TriangleStrip, 0, 4);
//        gl_window.swap_buffers().unwrap();
//
//        main_loop::ControlFlow::Continue
//    });
//}
