////#![feature(link_args)]
//
//extern crate gl;
//extern crate gl_helpers;
//extern crate glutin;
//extern crate main_loop;
//extern crate mat4;
//extern crate vec3;
//
//use gl::types::*;
//use glutin::{Api, GlContext, GlRequest};
//use std::f32;
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
//static CUBE_DATA: [GLfloat; 288] = [
//    1.0, 1.0, -1.0, 0.577349, 0.577349, -0.577349, 0.0, 0.0, 1.0, -1.0, -1.0, 0.577349, -0.577349,
//    -0.577349, 1.0, 0.0, -1.0, -1.0, -1.0, -0.577349, -0.577349, -0.577349, 1.0, 1.0, 1.0, 1.0,
//    1.0, 0.577349, 0.577349, 0.577349, 0.0, 0.0, -1.0, 1.0, 1.0, -0.577349, 0.577349, 0.577349,
//    1.0, 0.0, -1.0, -1.0, 1.0, -0.577349, -0.577349, 0.577349, 1.0, 1.0, 1.0, 1.0, -1.0, 0.577349,
//    0.577349, -0.577349, 0.0, 0.0, 1.0, 1.0, 1.0, 0.577349, 0.577349, 0.577349, 1.0, 0.0, 1.0,
//    -1.0, 1.0, 0.577349, -0.577349, 0.577349, 1.0, 1.0, 1.0, -1.0, -1.0, 0.577349, -0.577349,
//    -0.577349, 0.0, 0.0, 1.0, -1.0, 1.0, 0.577349, -0.577349, 0.577349, 1.0, 0.0, -1.0, -1.0, 1.0,
//    -0.577349, -0.577349, 0.577349, 1.0, 1.0, -1.0, -1.0, -1.0, -0.577349, -0.577349, -0.577349,
//    0.0, 0.0, -1.0, -1.0, 1.0, -0.577349, -0.577349, 0.577349, 1.0, 0.0, -1.0, 1.0, 1.0, -0.577349,
//    0.577349, 0.577349, 1.0, 1.0, 1.0, 1.0, 1.0, 0.577349, 0.577349, 0.577349, 0.0, 0.0, 1.0, 1.0,
//    -1.0, 0.577349, 0.577349, -0.577349, 1.0, 0.0, -1.0, 1.0, -1.0, -0.577349, 0.577349, -0.577349,
//    1.0, 1.0, -1.0, 1.0, -1.0, -0.577349, 0.577349, -0.577349, 0.0, 1.0, 1.0, 1.0, -1.0, 0.577349,
//    0.577349, -0.577349, 0.0, 0.0, -1.0, -1.0, -1.0, -0.577349, -0.577349, -0.577349, 1.0, 1.0,
//    1.0, -1.0, 1.0, 0.577349, -0.577349, 0.577349, 0.0, 1.0, 1.0, 1.0, 1.0, 0.577349, 0.577349,
//    0.577349, 0.0, 0.0, -1.0, -1.0, 1.0, -0.577349, -0.577349, 0.577349, 1.0, 1.0, 1.0, -1.0, -1.0,
//    0.577349, -0.577349, -0.577349, 0.0, 1.0, 1.0, 1.0, -1.0, 0.577349, 0.577349, -0.577349, 0.0,
//    0.0, 1.0, -1.0, 1.0, 0.577349, -0.577349, 0.577349, 1.0, 1.0, -1.0, -1.0, -1.0, -0.577349,
//    -0.577349, -0.577349, 0.0, 1.0, 1.0, -1.0, -1.0, 0.577349, -0.577349, -0.577349, 0.0, 0.0,
//    -1.0, -1.0, 1.0, -0.577349, -0.577349, 0.577349, 1.0, 1.0, -1.0, 1.0, -1.0, -0.577349,
//    0.577349, -0.577349, 0.0, 1.0, -1.0, -1.0, -1.0, -0.577349, -0.577349, -0.577349, 0.0, 0.0,
//    -1.0, 1.0, 1.0, -0.577349, 0.577349, 0.577349, 1.0, 1.0, -1.0, 1.0, 1.0, -0.577349, 0.577349,
//    0.577349, 0.0, 1.0, 1.0, 1.0, 1.0, 0.577349, 0.577349, 0.577349, 0.0, 0.0, -1.0, 1.0, -1.0,
//    -0.577349, 0.577349, -0.577349, 1.0, 1.0,
//];
//static CUBE_INDEX_DATA: [u16; 36] = [
//    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
//    26, 27, 28, 29, 30, 31, 32, 33, 34, 35,
//];
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
//static CUBE_VERTEX: &'static str = "
//    uniform mat4 projection;
//    uniform mat4 model_view;
//    uniform vec3 light;
//
//    attribute vec3 position;
//    attribute vec3 normal;
//    attribute vec2 uv;
//
//    varying vec3 v_light;
//    varying vec3 v_normal;
//    varying vec2 v_uv;
//
//    void main() {
//        gl_Position = projection * model_view * vec4(position, 1.0);
//        v_light = normalize(light - position);
//        v_normal = normal;
//        v_uv = uv;
//    }
//";
//#[cfg(any(target_os = "emscripten", target_os = "android"))]
//static CUBE_FRAGMENT: &'static str = "
//    precision mediump float;
//
//    varying vec3 v_light;
//    varying vec3 v_normal;
//    varying vec2 v_uv;
//
//    void main() {
//        vec3 light_dir = v_light;
//        vec3 color = vec3(v_uv, 0.5);
//        vec3 intensity = vec3(0.75, 0.75, 0.75);
//        float dprod = dot(light_dir, v_normal);
//
//        if (dprod > 0.75) {
//            intensity = vec3(1.0, 1.0, 1.0);
//        } else if (dprod > 0.25) {
//            intensity = vec3(0.8, 0.8, 0.8);
//        }
//
//        gl_FragColor = vec4(color * intensity, 1.0);
//    }
//";
//#[cfg(not(any(target_os = "emscripten", target_os = "android")))]
//static CUBE_FRAGMENT: &'static str = "
//    varying vec3 v_light;
//    varying vec3 v_normal;
//    varying vec2 v_uv;
//
//    void main() {
//        vec3 light_dir = v_light;
//        vec3 color = vec3(v_uv, 0.5);
//        vec3 intensity = vec3(0.75, 0.75, 0.75);
//        float dprod = dot(light_dir, v_normal);
//
//        if (dprod > 0.75) {
//            intensity = vec3(1.0, 1.0, 1.0);
//        } else if (dprod > 0.25) {
//            intensity = vec3(0.8, 0.8, 0.8);
//        }
//
//        gl_FragColor = vec4(color * intensity, 1.0);
//    }
//";
//
//fn main() {
//    let mut screen_width = 1024_usize;
//    let mut screen_height = 768_usize;
//
//    let mut events_loop = glutin::EventsLoop::new();
//    let window = glutin::WindowBuilder::new()
//        .with_title("Cube")
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
//    let mut fb_vertex_array = GLVertexArray::new();
//
//    fb_vertex_array.bind();
//
//    let fb_buffer = GLBuffer::new(BufferTarget::Array, 4, Usage::StaticDraw, &FB_DATA);
//
//    fb_buffer.bind();
//    fb_vertex_array.add_attribute(&fb_buffer, fb_program.get_attribute("position"), 0);
//    fb_vertex_array.add_attribute(&fb_buffer, fb_program.get_attribute("uv"), 2);
//
//    let cube_program = GLProgram::new(CUBE_VERTEX, CUBE_FRAGMENT);
//
//    let mut cube_vertex_array = GLVertexArray::new();
//
//    cube_vertex_array.bind();
//
//    let cube_buffer = GLBuffer::new(BufferTarget::Array, 8, Usage::StaticDraw, &CUBE_DATA);
//    let cube_index_buffer = GLBuffer::new(
//        BufferTarget::ElementArray,
//        0,
//        Usage::StaticDraw,
//        &CUBE_INDEX_DATA,
//    );
//
//    cube_buffer.bind();
//    cube_vertex_array.add_attribute(&cube_buffer, cube_program.get_attribute("position"), 0);
//    cube_vertex_array.add_attribute(&cube_buffer, cube_program.get_attribute("normal"), 3);
//    cube_vertex_array.add_attribute(&cube_buffer, cube_program.get_attribute("uv"), 5);
//    cube_index_buffer.bind();
//
//    let fov = 45_f32 * (f32::consts::PI / 180_f32);
//    //let ortho_size: f32 = 2_f32;
//
//    let origin = vec3::new(0_f32, 0_f32, 0_f32);
//    let up = vec3::new(0_f32, 0_f32, 1_f32);
//
//    let mut camera = mat4::new_identity::<f32>();
//    let mut camera_position = vec3::new(6_f32, 6_f32, 4_f32);
//    let mut light_position = vec3::new(2_f32, 2_f32, 2_f32);
//
//    let mut projection = mat4::new_identity::<f32>();
//    mat4::perspective(
//        &mut projection,
//        &fov,
//        &(screen_width as f32 / screen_height as f32),
//        &0.0001_f32,
//        &1024_f32,
//    );
//
//    let mut model_view = mat4::new_identity::<f32>();
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
//                    /*
//                    let aspect = w as f32 / h as f32;
//                    let r = ortho_size * aspect;
//                    let l = -r;
//                    let t = ortho_size;
//                    let b = -t;
//                    mat4::orthographic(&mut projection, t, r, b, l, 0.0001_f32, 1024_f32);
//                    */
//                    mat4::perspective(
//                        &mut projection,
//                        &fov,
//                        &(screen_width as f32 / screen_height as f32),
//                        &0.0001_f32,
//                        &1024_f32,
//                    );
//                }
//                _ => (),
//            }
//        }
//
//        light_position[0] = (ms * 0.001f32).sin() * 6_f32;
//        light_position[1] = (ms * 0.001f32).cos() * 6_f32;
//        light_position[2] = (ms * 0.001f32).sin() * 6_f32;
//
//        camera_position[0] = (ms * 0.001f32).sin() * 6_f32;
//        camera_position[1] = (ms * 0.001f32).cos() * 6_f32;
//        camera_position[2] = 3.5_f32 + ((ms * 0.001f32).sin() * 3_f32);
//
//        mat4::look_at(&mut camera, &camera_position, &origin, &up);
//        mat4::set_position(&mut camera, &camera_position);
//        mat4::inv(&mut model_view, &camera);
//
//        fb_framebuffer.bind();
//        fb_renderbuffer.bind();
//
//        gl_set_viewport(0, 0, screen_width / SIZE, screen_height / SIZE);
//        gl_set_clear_color(&[0.3, 0.3, 0.3, 1.0]);
//        gl_clear(true, true, true);
//
//        cube_program.bind();
//
//        cube_vertex_array.bind();
//        cube_vertex_array.enable_attributes();
//
//        cube_program
//            .get_uniform("projection")
//            .set_mat4f(&projection);
//        cube_program
//            .get_uniform("model_view")
//            .set_mat4f(&model_view);
//        cube_program.get_uniform("light").set_3f(&light_position);
//
//        gl_draw_elements(DrawMode::Triangles, 36, IndexKind::UnsignedShort, 0);
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
