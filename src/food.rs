use piston_window::{G2d, rectangle, math};
use rand::Rng;

const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

pub struct Food {
    pub x: f64,
    pub y: f64,
    window_size: f64,
    grid_size: f64,
}

impl Food {
    fn get_random_coordinate(&self) -> f64 {
        let mut rng = rand::thread_rng();
        (rng.gen_range(0..(self.window_size as u32 / self.grid_size as u32)) * self.grid_size as u32) as f64
    }
    pub fn new(window_size: f64, grid_size: f64) -> Food {
        let mut food = Food {
            x: 0.0,
            y: 0.0,
            window_size,
            grid_size
        };
        food.regenerate();
        food
    }
    pub fn regenerate(&mut self) {
        self.x = self.get_random_coordinate();
        self.y = self.get_random_coordinate();
    }
    pub fn draw(&self, transform: math::Matrix2d, graphics: &mut G2d) {
        rectangle(
            RED,
            [self.x, self.y, self.grid_size, self.grid_size],
            transform,
            graphics,
        );
    }
}