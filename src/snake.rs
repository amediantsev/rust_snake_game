use piston_window::{Context, G2d, G2dTexture, Image, Transformed};

use super::Food;

const GRID_SIZE: f64 = 30.0;
const WINDOW_SIZE: f64 = 600.0;


#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct SnakePiece {
    x: f64,
    y: f64,
    direction_from: Direction,
    direction_to: Direction,
}

impl PartialEq<&mut Food> for SnakePiece {
    fn eq(&self, other: &&mut Food) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl PartialEq for SnakePiece {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

pub struct Snake {
    pieces: Vec<SnakePiece>,
    direction: Direction,
    pub dead: bool,
}

impl Default for Snake {
    fn default() -> Self {
        Snake {
            pieces: vec![SnakePiece {
                x: 0.0,
                y: 0.0,
                direction_from: Direction::Right,
                direction_to: Direction::Right,
            }],
            direction: Direction::Right,
            dead: false,
        }
    }
}

impl Snake {
    fn head(&self) -> &SnakePiece {
        self.pieces.last().unwrap()
    }

    pub fn turn(&mut self, direction: Direction) {
        println!("TURN TO {:?}", direction);
        self.direction = direction;
        self.pieces.last_mut().unwrap().direction_to = self.direction;
        // let len = self.pieces.len();
        // if len >= 2 {
        //     self.pieces.get_mut(len - 2).unwrap().direction_to = self.direction;
        //     println!("Changed neck - {:?}", self.pieces.get(len - 2).unwrap());
        // }
    }

    fn generate_new_piece(&mut self) -> SnakePiece {
        let current_head = self.head();
        let (new_x, new_y) = match self.direction {
            Direction::Up => {
                let new_y = current_head.y - GRID_SIZE;
                (current_head.x, if new_y < 0.0 { WINDOW_SIZE } else { new_y })
            }
            Direction::Down => {
                let new_y = current_head.y + GRID_SIZE;
                (current_head.x, if new_y > WINDOW_SIZE { 0.0 } else { new_y })
            }
            Direction::Left => {
                let new_x = current_head.x - GRID_SIZE;
                (if new_x < 0.0 { WINDOW_SIZE } else { new_x }, current_head.y)
            }
            Direction::Right => {
                let new_x = current_head.x + GRID_SIZE;
                (if new_x > WINDOW_SIZE { 0.0 } else { new_x }, current_head.y)
            }
        };
        SnakePiece {
            x: new_x,
            y: new_y,
            direction_from: current_head.direction_to,
            direction_to: self.direction,
        }
    }

    pub fn move_ahead(&mut self, food: &mut Food) {
        let new_head = self.generate_new_piece();
        println!("New head: {:?}", new_head);
        // Check if the snake's new head position encounters its own body
        if self.pieces.contains(&new_head) {
            self.dead = true;
        } else {
            if new_head == food {
                // Increase the snake's length and generate a new food position
                food.regenerate();
            } else {
                // Remove the tail
                self.pieces.remove(0);
            }
            // let len = self.pieces.len();
            // if len >= 2 {
            //     self.pieces.get_mut(len - 2).unwrap().direction_to = self.direction;
            //     println!("Changed neck - {:?}", self.pieces.get(len - 2).unwrap());
            // }
            self.pieces.push(new_head);
        }
    }

    pub fn draw(&self, context: Context, graphics: &mut G2d, head_texture: &G2dTexture, body_piece_texture: &G2dTexture, snake_angle_piece_texture: &G2dTexture) {
        let head = self.head();
        for body_part in &self.pieces {
            let (texture, transform) = if body_part == head {
                println!("drawing head {:?}", body_part);
                (head_texture, context.transform.trans(body_part.x, body_part.y))
            } else {
                match (body_part.direction_from, body_part.direction_to) {
                    (Direction::Right | Direction::Left, Direction::Left | Direction::Right) => {
                        // Horizontal straight piece
                        println!("drawing horizontal poop for {:?}", body_part);
                        (body_piece_texture, context.transform.trans(body_part.x, body_part.y))
                    }
                    (Direction::Up | Direction::Down, Direction::Down | Direction::Up) => {
                        // Vertical straight piece
                        println!("drawing vertical poop for {:?}", body_part);
                        (
                            body_piece_texture,
                            context.transform
                                .trans(body_part.x + GRID_SIZE / 2.0, body_part.y + GRID_SIZE / 2.0)
                                .rot_rad(std::f64::consts::PI / 2.0)
                                .trans(-GRID_SIZE / 2.0, -GRID_SIZE / 2.0)
                        )
                    }
                    (from, to) => {
                        // Angle of the snake's body.
                        let rotation_rad = match (from, to) {
                            (Direction::Right, Direction::Up) | (Direction::Up, Direction::Left) => {
                                std::f64::consts::PI / 2.0
                            }
                            (Direction::Down, Direction::Right) | (Direction::Left, Direction::Up) => {
                                std::f64::consts::PI
                            }
                            (Direction::Up, Direction::Right) | (Direction::Left, Direction::Down) => {
                                std::f64::consts::PI * 1.5
                            }
                            _ => 0.0
                        };
                        println!("drawing angle poop ({rotation_rad}) for {:?}", body_part);
                        (
                            snake_angle_piece_texture,
                            context.transform
                                .trans(body_part.x + GRID_SIZE / 2.0, body_part.y + GRID_SIZE / 2.0)
                                .rot_rad(rotation_rad)
                                .trans(-GRID_SIZE / 2.0, -GRID_SIZE / 2.0)
                        )
                    }
                }
            };
            Image::new().draw(
                texture,
                &context.draw_state,
                transform,
                graphics,
            );
        }
    }
}