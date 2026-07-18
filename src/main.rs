mod framebuffer;
mod line;
mod render;


use raylib::prelude::*;
use framebuffer::Framebuffer;
use crate::render::render;
use std::thread;
use std::time::Duration;



fn main() {
    let window_width = 800;
    let window_height = 600;

    let (mut window,  mut raylib_thread) = raylib::init()
        .size(window_width, window_height)
        .title("Window Example")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();



    let mut framebuffer = Framebuffer::new(window_width, window_height);

    framebuffer.set_background_color(Color::new(50, 50, 50, 255));
    
    let mut traslate_x = 0.0;
    let mut traslate_y = 0.0;

    while !window.window_should_close() {

        traslate_x += 1.0;
        traslate_y += 1.0;

        framebuffer.clear();

        render(&mut framebuffer, traslate_x, traslate_y);

        framebuffer.swap_buffers(&mut window, &mut raylib_thread);

        thread::sleep(Duration::from_millis(16));
    }
}
