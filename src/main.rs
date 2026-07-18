mod framebuffer;
mod render;
mod life;
mod patterns;


use raylib::prelude::*;
use framebuffer::Framebuffer;
use crate::render::render;
use crate::life::GameOfLife;
use std::thread;
use std::time::Duration;



fn main() {
    let window_width = 800;
    let window_height = 600;

    let framebuffer_width = 100;
    let framebuffer_height = 75;

    let (mut window,  mut raylib_thread) = raylib::init()
        .size(window_width, window_height)
        .title("Conway’s Game Of Life")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();



    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);

    framebuffer.set_background_color(Color::new(50, 50, 50, 255));

    let mut grid = GameOfLife::new(framebuffer_width, framebuffer_height);
  

    patterns::block(&mut grid, 50, 40);
    patterns::block(&mut grid, 53, 40);
    patterns::block(&mut grid, 47, 40);
    patterns::block(&mut grid, 50, 43);
    patterns::block(&mut grid, 50, 37);

    patterns::blinker(&mut grid, 46, 44);
    patterns::blinker(&mut grid, 54, 36);
    patterns::blinker(&mut grid, 54, 44);
    patterns::blinker(&mut grid, 46, 36);

    patterns::beacon(&mut grid, 50, 15);

    patterns::beacon(&mut grid, 25, 40);
    patterns::beacon(&mut grid, 75, 40);
   
    patterns::block(&mut grid, 43, 30);
    patterns::block(&mut grid, 37, 30);
    patterns::block(&mut grid, 40, 33);
    patterns::block(&mut grid, 40, 27);
    
    patterns::beehive(&mut grid, 5, 5);
    patterns::loaf(&mut grid, 85, 5);
    patterns::boat(&mut grid, 5, 65);
    patterns::tub(&mut grid, 85, 65);
    patterns::toad(&mut grid, 10, 35);
    patterns::glider(&mut grid, 90, 20);
    patterns::lwss(&mut grid, 10, 55);

    patterns::beehive(&mut grid, 8, 5);
    patterns::loaf(&mut grid, 80, 5);
    patterns::boat(&mut grid, 50, 6);
    patterns::tub(&mut grid, 8, 65);
    patterns::toad(&mut grid, 10, 35);

    patterns::beehive(&mut grid, 78, 9);
    patterns::loaf(&mut grid, 80, 5);
    patterns::lwss(&mut grid, 90, 4);
    patterns::tub(&mut grid, 80, 65);
    patterns::blinker(&mut grid, 76, 60);
    patterns::toad(&mut grid, 90, 35);
    patterns::beacon(&mut grid, 90, 65);
    patterns::glider(&mut grid, 60, 25);
    patterns::glider(&mut grid, 90, 8);

    patterns::tub(&mut grid, 30, 30);
    patterns::blinker(&mut grid, 20, 20);
    patterns::beacon(&mut grid, 62, 28);

    patterns::beacon(&mut grid, 20, 60);
    patterns::beacon(&mut grid, 70, 60);

    patterns::glider(&mut grid, 15, 55);
    patterns::glider(&mut grid, 65, 8);
    patterns::glider(&mut grid, 35, 65);
    patterns::glider(&mut grid, 88, 45);

    patterns::lwss(&mut grid, 5, 30);
    patterns::lwss(&mut grid, 55, 55);
    patterns::lwss(&mut grid, 25, 12);
    patterns::lwss(&mut grid, 70, 30);
    patterns::lwss(&mut grid, 40, 68);

    patterns::glider(&mut grid, 45, 20);
    patterns::glider(&mut grid, 10, 45);
    patterns::glider(&mut grid, 75, 60);

    patterns::lwss(&mut grid, 15, 65);
    patterns::lwss(&mut grid, 60, 10);
    patterns::lwss(&mut grid, 85, 55);

    while !window.window_should_close() {

        render(&mut framebuffer, &mut grid);

        framebuffer.swap_buffers(&mut window, &mut raylib_thread);

        thread::sleep(Duration::from_millis(120));
    }
}
