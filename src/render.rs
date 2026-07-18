use raylib::prelude::*;
use crate::framebuffer::Framebuffer;
use crate::line::line;


pub fn render(
    framebuffer: &mut Framebuffer,
    traslate_x: f32,
    traslate_y: f32,
){
    framebuffer.set_current_color(Color::GREEN);
    line(
        framebuffer,
        Vector2::new(50.0 + traslate_x, 50.0 + traslate_y),
        Vector2::new(350.0 + traslate_x, 350.0 + traslate_y),
    );

    framebuffer.set_current_color(Color::RED);
    line(
        framebuffer,
        Vector2::new(350.0 + traslate_x, 50.0 + traslate_y),
        Vector2::new(50.0 + traslate_x, 350.0 + traslate_y),
    );

}