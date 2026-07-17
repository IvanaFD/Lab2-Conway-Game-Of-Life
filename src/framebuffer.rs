use raylib::prelude::*;

pub struct Framebuffer {
    pub width: i32,
    pub height: i32,
    background_color: Color,
    current_color: Color,
    pub color_buffer: Image,
}

impl Framebuffer {
    pub fn new(width: i32, height: i32) -> Self {
        let background_color = Color::GREEN;
        let color_buffer = Image::gen_image_color(width, height, background_color);

        Framebuffer {
            width,
            height,
            background_color,
            current_color: Color::WHITE,
            color_buffer,
        }
    }

    pub fn swap_buffers(
        &self,
        window: &mut RaylibHandle,
        raylib_thread: &mut RaylibThread,
    ){
        if let Ok(texture) = window.load_texture_from_image(raylib_thread, &self.color_buffer) {
            
            let mut render = window.begin_drawing(raylib_thread);

            render.draw_texture(&texture, 0, 0, Color::WHITE);
        }

    }

    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }

    pub fn set_current_color(&mut self, color: Color) {
        self.current_color = color;
    }

    pub fn clear(&mut self) {
        self.color_buffer = Image::gen_image_color(self.width, self.height, self.background_color);
    }

    pub fn set_pixel(&mut self, x: i32, y: i32) {

        let y_flipped = self.height - 1 - y;

        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            self.color_buffer.draw_pixel(x, y_flipped, self.current_color);
        }
    }
    /*
    pub fn render_to_file(&self, file_path: &str) {
        self.color_buffer.export_image(file_path);
    }
    */

}