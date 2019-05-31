use std::os::raw::{c_int, c_void, c_uchar};
use std::cell::RefCell;
use std::ptr::null_mut;
use std::mem;
use stb_image::image;
use stb_image::image::LoadResult::{ImageU8, ImageF32, Error};

pub mod gl {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}
use self::gl::types::*;

#[allow(non_camel_case_types)]
type em_callback_func = unsafe extern "C" fn();

extern "C" {
    // This extern is built in by Emscripten.
    pub fn emscripten_run_script_int(x: *const c_uchar) -> c_int;
    pub fn emscripten_cancel_main_loop();
    pub fn emscripten_set_main_loop(func: em_callback_func,
                                    fps: c_int,
                                    simulate_infinite_loop: c_int);
}

thread_local!(static MAIN_LOOP_CALLBACK: RefCell<*mut c_void> = RefCell::new(null_mut()));

pub fn set_main_loop_callback<F>(callback : F) where F : FnMut() {
    MAIN_LOOP_CALLBACK.with(|log| {
            *log.borrow_mut() = &callback as *const _ as *mut c_void;
            });

    unsafe { emscripten_set_main_loop(wrapper::<F>, 0, 1); }

    unsafe extern "C" fn wrapper<F>() where F : FnMut() {
        MAIN_LOOP_CALLBACK.with(|z| {
            let closure = *z.borrow_mut() as *mut F;
            (*closure)();
        });
    }
}

fn texture_from_image_u8(image: image::Image<u8>) -> GLuint {
    unsafe {
        let mut tex_id: GLuint = 0;
        gl::GenTextures(1, &mut tex_id);
        gl::BindTexture(gl::TEXTURE_2D, tex_id);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
        gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as i32, image.width as i32, image.height as i32, 0, gl::RGBA, gl::UNSIGNED_BYTE, mem::transmute(&image.data[0]));
        return tex_id;
    }
}

fn load_texture_from_memory(data: &[u8]) -> GLuint {
    let stbimg = image::load_from_memory(data);
    match stbimg {
        ImageU8(img) => {
            return texture_from_image_u8(img);
        },
        ImageF32(img) => {
            return 0;
        },
        Error(error) => {
            let e: &str = &error[..];
            let er: &str = &format!("Error loading texture: {}", e)[..];
            sdl2::log::log(er);
            return 0;
        },
    }
}

pub struct Context {
    window: sdl2::video::Window,
    gl_context: sdl2::video::GLContext,
    dummy: &'static str,
}

pub struct Engine {
    context: Option<Context>,
}

impl Engine {
    pub fn new() -> Self {
        Engine {
            context: None,
        }
    }

    pub fn setup(&mut self) {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        video_subsystem.gl_attr().set_context_profile(sdl2::video::GLProfile::GLES);
        video_subsystem.gl_attr().set_context_major_version(2);
        video_subsystem.gl_attr().set_context_minor_version(0);

        let window = video_subsystem
            .window("rust-sdl2-wasm", 800, 600)
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        let gl_context = window.gl_create_context().unwrap();
        window.gl_make_current(&gl_context).unwrap();

        sdl2::log::log("Loading GL extensions");
        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const _);

        let mut event_pump = sdl_context.event_pump().unwrap();

        // TODO: create the shaders

        //let img_data = include_bytes!("../assets/wabbit_alpha.png");
        //let tex_id = load_texture_from_memory(img_data);

        self.context = Some(Context {
            window: window,
            gl_context: gl_context,
            dummy: "test 123",
        });

    }

}

fn main_loop(mut engine: &mut Engine) {
    //sdl2::log::log("Main loop");
    let context = &mut engine.context;
    match context {
        Some(context) => {
            unsafe {
                gl::ClearColor(191.0/255.0, 255.0/255.0, 255.0/255.0, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }
            
            sdl2::log::log(context.dummy);
            context.window.gl_swap_window();
        },
        None => {
            sdl2::log::log("Missing context");
        }
    }
}

/*
fn main() {
    println!("Startup");
    let mut engine = Engine::new();
    engine.setup();
    set_main_loop_callback(|| {
        main_loop(&mut engine);
    });
    stdweb::event_loop();
}
*/

fn main() {
    println!("Startup");
    /*
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    video_subsystem.gl_attr().set_context_profile(sdl2::video::GLProfile::GLES);
    video_subsystem.gl_attr().set_context_major_version(2);
    video_subsystem.gl_attr().set_context_minor_version(0);

    let window = video_subsystem
        .window("rust-sdl2-wasm", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let gl_context = window.gl_create_context().unwrap();
    window.gl_make_current(&gl_context).unwrap();

    sdl2::log::log("Loading GL extensions");
    gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const _);

    let mut event_pump = sdl_context.event_pump().unwrap();

    // TODO: create the shaders

    //let img_data = include_bytes!("../assets/wabbit_alpha.png");
    //let tex_id = load_texture_from_memory(img_data);
    */
    let mut dummy = "test 123";

    set_main_loop_callback(|| {
        unsafe {
            gl::ClearColor(191.0/255.0, 255.0/255.0, 255.0/255.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        println!("{}", dummy);
        let mut something = vec![1; 1_000_000];
        //sdl2::log::log(dummy);
        //window.gl_swap_window();
    });

}