use crate::framebuffer::Framebuffer;
use crate::life::GameOfLife;

pub fn render(framebuffer: &mut Framebuffer, grid: &mut GameOfLife) {
    grid.draw(framebuffer);
    *grid = grid.step();
}