use crate::constants::{mold_constants::FADE_FACTOR, GridType};

pub struct Map {
    width: usize,
    height: usize,
    grid: GridType,
}

impl Map {
    pub fn new(width: usize, height: usize) -> Self {
        Map {
            width,
            height,
            grid: Map::create_grid(width, height),
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    fn create_grid(width: usize, height: usize) -> GridType {
        let mut grid: GridType = Vec::new();
        for y in 0..height {
            grid.push(Vec::new());
            for _ in 0..width {
                grid[y as usize].push(0.0);
            }
        }
        grid
    }

    pub fn get_field(&self, x: usize, y: usize) -> f32 {
        if !self.check_coords_in_bounds(x, y) {
            return 0.0;
        }
        self.grid[y][x]
    }

    pub fn set_field(&mut self, x: usize, y: usize, val: f32) {
        if !self.check_coords_in_bounds(x, y) {
            return;
        }
        self.grid[y][x] = val;
    }

    fn check_coords_in_bounds(&self, x: usize, y: usize) -> bool {
        if (x < self.width) && (y < self.height) {
            return true;
        }
        false
    }

    pub fn blue(&mut self) {}

    pub fn fade(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.grid[y][x] -= FADE_FACTOR;
            }
        }
    }

    pub fn get_vals_to_update(&self) -> Vec<((i32, i32), f32)> {
        let mut output = Vec::new();

        for y in 0..self.height {
            for x in 0..self.width {
                if self.get_field(x, y) > 0.0 {
                    output.push(((x as i32, y as i32), self.get_field(x, y)));
                }
            }
        }

        output
    }
}
