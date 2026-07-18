use raylib::prelude::*;
use crate::framebuffer::Framebuffer;

pub struct GameOfLife {
    pub width: i32,
    pub height: i32,
    cells: Vec<bool>,
}

impl GameOfLife {
    pub fn new(width: i32, height: i32) -> Self {
        GameOfLife {
            width,
            height,
            cells: vec![false; (width * height) as usize],
        }
    }

    fn index(&self, x: i32, y: i32) -> usize {
        (y * self.width + x) as usize
    }

    pub fn set_alive(&mut self, x: i32, y: i32) {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            let idx = self.index(x, y);
            self.cells[idx] = true;
        }
    }

    pub fn is_alive(&self, x: i32, y: i32) -> bool {
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            return false;
        }
        self.cells[self.index(x, y)]
    }

    fn count_alive_neighbors(&self, x: i32, y: i32) -> u8 {
        let mut count = 0;
        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                if self.is_alive(x + dx, y + dy) {
                    count += 1;
                }
            }
        }
        count
    }

    pub fn step(&self) -> GameOfLife {
        let mut next = GameOfLife::new(self.width, self.height);

        for y in 0..self.height {
            for x in 0..self.width {
                let alive = self.is_alive(x, y);
                let neighbors = self.count_alive_neighbors(x, y);

                let next_alive = match (alive, neighbors) {
                    (true, 2) | (true, 3) => true,
                    (false, 3) => true,
                    _ => false,
                };

                if next_alive {
                    next.set_alive(x, y);
                }
            }
        }

        next
    }

    pub fn draw(&self, framebuffer: &mut Framebuffer) {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.is_alive(x, y) {
                    framebuffer.set_current_color(Color::YELLOW);
                } else {
                    framebuffer.set_current_color(Color::BLUE);
                }
                framebuffer.set_pixel(x, y);
            }
        }
    }
}
