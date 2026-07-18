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

            let screen_width = window.get_screen_width() as f32;
            let screen_height = window.get_screen_height() as f32;

            let mut render = window.begin_drawing(raylib_thread);

            let source_rec = Rectangle::new(0.0, 0.0, self.width as f32, self.height as f32);
            let dest_rec = Rectangle::new(0.0, 0.0, screen_width, screen_height);

            render.draw_texture_pro(
                &texture,
                source_rec,
                dest_rec,
                Vector2::new(0.0, 0.0),
                0.0,
                Color::WHITE,
            );
        }

    }

    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }

    pub fn set_current_color(&mut self, color: Color) {
        self.current_color = color;
    }

    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.color_buffer = Image::gen_image_color(self.width, self.height, self.background_color);
    }

    pub fn set_pixel(&mut self, x: i32, y: i32) {

        let y_flipped = self.height - 1 - y;

        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            self.color_buffer.draw_pixel(x, y_flipped, self.current_color);
        }
    }

    #[allow(dead_code)]
    pub fn get_color(&mut self, x: i32, y: i32) -> Color {
        let y_flipped = self.height - 1 - y;
        self.color_buffer.get_color(x, y_flipped)
    }
    /*
    pub fn render_to_file(&self, file_path: &str) {
        self.color_buffer.export_image(file_path);
    }
    */

}