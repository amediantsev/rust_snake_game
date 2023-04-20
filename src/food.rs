use piston_window::{G2d, G2dTexture, Image, Context, Transformed};
use rand::Rng;


pub struct Food {
    pub x: f64,
    pub y: f64,
    window_size: f64,
    grid_size: f64,
    texture: G2dTexture,
}

impl Food {
    fn get_random_coordinate(&self) -> f64 {
        let mut rng = rand::thread_rng();
        (rng.gen_range(0..(self.window_size as u32 / self.grid_size as u32)) * self.grid_size as u32) as f64
    }
    pub fn new(window_size: f64, grid_size: f64, texture: G2dTexture) -> Food {
        let mut food = Food {
            x: 0.0,
            y: 0.0,
            window_size,
            grid_size,
            texture,
        };
        food.regenerate();
        return food;
    }
    pub fn regenerate(&mut self) {
        self.x = self.get_random_coordinate();
        self.y = self.get_random_coordinate();
    }
    pub fn draw(&self, context: Context, graphics: &mut G2d) {
        Image::new().draw(
            &self.texture,
            &context.draw_state,
            context.transform.trans(self.x, self.y),
            graphics,
        );
    }
}