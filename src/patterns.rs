use crate::life::GameOfLife;

fn place(grid: &mut GameOfLife, x0: i32, y0: i32, cells: &[(i32, i32)]) {
    for (dx, dy) in cells {
        grid.set_alive(x0 + dx, y0 + dy);
    }
}


pub fn block(grid: &mut GameOfLife, x0: i32, y0: i32) {
    place(grid, x0, y0, &[(0, 0), (1, 0), (0, 1), (1, 1)]);
}

pub fn beehive(grid: &mut GameOfLife, x0: i32, y0: i32) {
    place(
        grid,
        x0,
        y0,
        &[(1, 0), (2, 0), (0, 1), (3, 1), (1, 2), (2, 2)],
    );
}

pub fn loaf(grid: &mut GameOfLife, x0: i32, y0: i32) {
    place(
        grid,
        x0,
        y0,
        &[(1, 0), (2, 0), (0, 1), (3, 1), (1, 2), (3, 2), (2, 3)],
    );
}

pub fn boat(grid: &mut GameOfLife, x0: i32, y0: i32) {
    place(grid, x0, y0, &[(0, 0), (1, 0), (0, 1), (2, 1), (1, 2)]);
}

pub fn tub(grid: &mut GameOfLife, x0: i32, y0: i32) {
    place(grid, x0, y0, &[(1, 0), (0, 1), (2, 1), (1, 2)]);
}



pub fn blinker(grid: &mut GameOfLife, x0: i32, y0: i32) {
    place(grid, x0, y0, &[(0, 0), (1, 0), (2, 0)]);
}

pub fn toad(grid: &mut GameOfLife, x0: i32, y0: i32) {
    place(
        grid,
        x0,
        y0,
        &[(1, 0), (2, 0), (3, 0), (0, 1), (1, 1), (2, 1)],
    );
}

pub fn beacon(grid: &mut GameOfLife, x0: i32, y0: i32) {
    place(
        grid,
        x0,
        y0,
        &[(0, 0), (1, 0), (0, 1), (1, 1), (2, 2), (3, 2), (2, 3), (3, 3)],
    );
}



pub fn glider(grid: &mut GameOfLife, x0: i32, y0: i32) {
    place(grid, x0, y0, &[(1, 0), (2, 1), (0, 2), (1, 2), (2, 2)]);
}

pub fn lwss(grid: &mut GameOfLife, x0: i32, y0: i32) {
    place(
        grid,
        x0,
        y0,
        &[
            (1, 0), (4, 0),
            (0, 1),
            (0, 2), (4, 2),
            (0, 3), (1, 3), (2, 3), (3, 3),
        ],
    );
}
