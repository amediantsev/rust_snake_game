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

#[derive(Debug, PartialEq)]
struct Position {
    x: f64,
    y: f64,
}

impl Direction {
    fn is_opposite(self, other: Self) -> bool {
        matches!(
            (self, other),
            (Direction::Up, Direction::Down)
            | (Direction::Down, Direction::Up)
            | (Direction::Left, Direction::Right)
            | (Direction::Right, Direction::Left)
        )
    }
}

#[derive(Debug, PartialEq)]
struct SnakePiece {
    position: Position,
    direction_from: Direction,
    direction_to: Direction,
}

impl SnakePiece {
    fn new(x: f64, y: f64, direction: Direction) -> Self {
        SnakePiece {
            position: Position { x, y },
            direction_from: direction,
            direction_to: direction,
        }
    }
}

impl PartialEq<&mut Food> for SnakePiece {
    fn eq(&self, other: &&mut Food) -> bool {
        self.position.x == other.x && self.position.y == other.y
    }
}

pub struct Snake {
    pieces: Vec<SnakePiece>,
    pub dead: bool,
    window_size: f64,
}

impl Snake {
    pub fn new(window_size: f64) -> Self {
        Snake {
            pieces: vec![SnakePiece::new(0.0, 0.0, Direction::Right)],
            dead: false,
            window_size,
        }
    }

    fn head(&self) -> &SnakePiece {
        self.pieces.last().unwrap()
    }

    pub fn reset(&mut self) {
        *self = Self::new(self.window_size);
    }

    pub fn turn(&mut self, direction: Direction) {
        let mut head = self.pieces.last_mut().unwrap();
        if !(head.direction_to.is_opposite(direction)) {
            head.direction_to = direction
        }
    }

    fn generate_new_piece(&mut self) -> SnakePiece {
        let current_head = self.head();
        let (new_x, new_y) = match current_head.direction_to {
            Direction::Up => (current_head.position.x, self.get_new_coordinate(current_head.position.y, false)),
            Direction::Down => (current_head.position.x, self.get_new_coordinate(current_head.position.y, true)),
            Direction::Left => (self.get_new_coordinate(current_head.position.x, false), current_head.position.y),
            Direction::Right => (self.get_new_coordinate(current_head.position.x, true), current_head.position.y),
        };
        SnakePiece::new(new_x, new_y, current_head.direction_to)
    }

    fn get_new_coordinate(&self, coordinate: f64, is_increment: bool) -> f64 {
        let new_coordinate = if is_increment { coordinate + GRID_SIZE } else { coordinate - GRID_SIZE };
        if new_coordinate < 0.0 { self.window_size - GRID_SIZE } else if new_coordinate >= WINDOW_SIZE { 0.0 } else { new_coordinate }
    }

    pub fn move_ahead(&mut self, food: &mut Food) {
        let new_head = self.generate_new_piece();
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
            self.pieces.push(new_head);
        }
    }

    pub fn draw(&mut self, context: Context, graphics: &mut G2d, head_texture: &G2dTexture, body_piece_texture: &G2dTexture, snake_angle_piece_texture: &G2dTexture) {
        let head = self.head();
        for body_part in &self.pieces {
            let (texture, transform) = if body_part == head {
                (head_texture, context.transform.trans(body_part.position.x, body_part.position.y))
            } else {
                match (body_part.direction_from, body_part.direction_to) {
                    (Direction::Right | Direction::Left, Direction::Left | Direction::Right) => {
                        // Horizontal straight piece
                        (body_piece_texture, context.transform.trans(body_part.position.x, body_part.position.y))
                    }
                    (Direction::Up | Direction::Down, Direction::Down | Direction::Up) => {
                        // Vertical straight piece
                        (
                            body_piece_texture,
                            context.transform.trans(body_part.position.x + GRID_SIZE / 2.0, body_part.position.y + GRID_SIZE / 2.0).rot_rad(std::f64::consts::PI / 2.0).trans(-GRID_SIZE / 2.0, -GRID_SIZE / 2.0)
                        )
                    }
                    (from, to) => {
                        // Angle of the snake's body.
                        let rotation_rad = match (from, to) {
                            (Direction::Right, Direction::Up) | (Direction::Down, Direction::Left) => {
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
                        (
                            snake_angle_piece_texture,
                            context.transform.trans(body_part.position.x + GRID_SIZE / 2.0, body_part.position.y + GRID_SIZE / 2.0).rot_rad(rotation_rad).trans(-GRID_SIZE / 2.0, -GRID_SIZE / 2.0)
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