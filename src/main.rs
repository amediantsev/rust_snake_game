use piston_window::{clear, rectangle, Button, Key, PistonWindow, PressEvent, RenderEvent, WindowSettings, CharacterCache, G2d};
use piston_window::{EventLoop, UpdateEvent};
use piston_window::{text, Glyphs, TextureSettings, Transformed};
use piston_window::{MouseCursorEvent, MouseButton};
use piston_window::math::{Matrix2d};
use piston_window::{Image, Texture, Flip, G2dTexture};
use rand::Rng;
use crate::Position::Angle;

const GRID_SIZE: f64 = 30.0;
const WINDOW_SIZE: f64 = 600.0;

const BUTTON_WIDTH: f64 = 150.0;
const BUTTON_HEIGHT: f64 = 50.0;
const BUTTON_X: f64 = (WINDOW_SIZE - BUTTON_WIDTH) / 2.0;
const BUTTON_Y: f64 = 350.0;
const BUTTON_TEXT_SIZE: u32 = 24;
const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];


#[derive(Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}


enum Position {
    Horizontal,
    Vertical,
    Angle { rotation_rad: f64 },
}

struct SnakePiece {
    x: f64,
    y: f64,
    position: Position,
}


impl PartialEq<&mut Food> for SnakePiece {
    fn eq(&self, other: &&mut Food) -> bool { self.x == other.x && self.y == other.y }
}

impl PartialEq for SnakePiece {
    fn eq(&self, other: &Self) -> bool { self.x == other.x && self.y == other.y }
}

struct Snake {
    coordinates: Vec<SnakePiece>,
    previous_direction: Direction,
    direction: Direction,
    dead: bool,
}

impl Default for Snake {
    fn default() -> Self {
        Snake {
            coordinates: vec![SnakePiece { x: 0.0, y: 0.0, position: Position::Horizontal }],
            previous_direction: Direction::Right,
            direction: Direction::Right,
            dead: false,
        }
    }
}

impl Snake {
    fn current_head(&self) -> &SnakePiece { self.coordinates.last().unwrap() }

    fn turn(&mut self, direction: Direction) {
        self.previous_direction = self.direction;
        self.direction = direction;
    }

    fn generate_new_piece(&mut self) -> SnakePiece {
        let position = self.current_head();
        let (new_x, new_y) = match self.direction {
            Direction::Up => {
                let new_y = position.y - GRID_SIZE;
                (position.x, if new_y < 0.0 { WINDOW_SIZE } else { new_y })
            }
            Direction::Down => {
                let new_y = position.y + GRID_SIZE;
                (position.x, if new_y > WINDOW_SIZE { 0.0 } else { new_y })
            }
            Direction::Left => {
                let new_x = position.x - GRID_SIZE;
                (if new_x < 0.0 { WINDOW_SIZE } else { new_x }, position.y)
            }
            Direction::Right => {
                let new_x = position.x + GRID_SIZE;
                (if new_x > WINDOW_SIZE { 0.0 } else { new_x }, position.y)
            }
        };
        SnakePiece {
            x: new_x,
            y: new_y,
            position: {
                if self.previous_direction == self.direction {
                    match self.direction {
                        Direction::Right | Direction::Left => Position::Horizontal,
                        Direction::Up | Direction::Down => Position::Vertical,
                    }
                } else {
                    let angle = Angle {
                        rotation_rad: match (self.previous_direction, self.direction) {
                            (Direction::Up, Direction::Left) | (Direction::Left, Direction::Up) => {
                                std::f64::consts::PI * 1.5
                            }
                            (Direction::Right, Direction::Up) | (Direction::Up, Direction::Right) => {
                                std::f64::consts::PI
                            }
                            (Direction::Down, Direction::Left) | (Direction::Left, Direction::Down) => {
                                std::f64::consts::PI / 2.0
                            }
                            _ => 0.0
                        }
                    };
                    self.previous_direction = self.direction;
                    angle
                }
            },
        }
    }

    fn move_ahead(&mut self, food: &mut Food) {
        let new_head = self.generate_new_piece();
        // Check if the snake's new head position encounters its own body
        if self.coordinates.contains(&new_head) {
            self.dead = true;
        } else {
            if new_head == food {
                // Increase the snake's length and generate a new food position
                food.regenerate();
            } else {
                // Remove the tail
                self.coordinates.remove(0);
            }

            // Add the new head position
            self.coordinates.push(new_head);
        }
    }

    fn draw(&self, context: piston_window::Context, graphics: &mut G2d, head_texture: &G2dTexture, body_piece_texture: &G2dTexture, snake_angle_piece_texture: &G2dTexture) {
        let head = self.current_head();
        for body_part in &self.coordinates {
            let (texture, transform) = if body_part == head {
                (head_texture, context.transform.trans(body_part.x, body_part.y))
            } else {
                match body_part.position {
                    Position::Vertical => {
                        (
                            body_piece_texture,
                            context.transform
                                .trans(body_part.x + GRID_SIZE / 2.0, body_part.y + GRID_SIZE / 2.0)
                                .rot_rad(std::f64::consts::PI / 2.0)
                                .trans(-GRID_SIZE / 2.0, -GRID_SIZE / 2.0)
                        )
                    }
                    Position::Horizontal => {
                        (body_piece_texture, context.transform.trans(body_part.x, body_part.y))
                    }
                    Angle { rotation_rad } => {
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

fn get_random_coordinate() -> f64 {
    let mut rng = rand::thread_rng();
    (rng.gen_range(0..(WINDOW_SIZE as u32 / GRID_SIZE as u32)) * GRID_SIZE as u32) as f64
}

struct Food {
    x: f64,
    y: f64,
}

impl Food {
    fn new() -> Food {
        Food {
            x: get_random_coordinate(),
            y: get_random_coordinate(),
        }
    }
    fn regenerate(&mut self) {
        self.x = get_random_coordinate();
        self.y = get_random_coordinate();
    }
    fn draw(&self, transform: Matrix2d, graphics: &mut G2d) {
        rectangle(
            RED,
            [self.x, self.y, GRID_SIZE, GRID_SIZE],
            transform,
            graphics,
        );
    }
}


fn main() {
    let mut window: PistonWindow = WindowSettings::new("Snake", [WINDOW_SIZE, WINDOW_SIZE])
        .exit_on_esc(true)
        .build()
        .unwrap();
    window.set_ups(2);

    let mut food = Food::new();
    let texture_context = window.create_texture_context();
    let mut glyphs = Glyphs::new("/Library/Fonts/Arial Unicode.ttf", texture_context, TextureSettings::new()).unwrap();
    let mut mouse_pos = [0.0, 0.0];
    let mut texture_context = window.create_texture_context();
    let head_texture = Texture::from_path(
        &mut texture_context,
        "images/huilo.png",
        Flip::None,
        &TextureSettings::new(),
    ).unwrap();
    let body_piece_texture = Texture::from_path(
        &mut texture_context,
        "images/poop_horizontal.png",
        Flip::None,
        &TextureSettings::new(),
    ).unwrap();
    let snake_angle_piece_texture = Texture::from_path(
        &mut texture_context,
        "images/poop_angle.png",
        Flip::None,
        &TextureSettings::new(),
    ).unwrap();
    let mut snake = Snake::default();

    while let Some(event) = window.next() {
        if let Some(_) = event.render_args() {
            window.draw_2d(&event, |context, graphics, device| {
                // Clear the window
                clear([0.0, 0.0, 0.0, 1.0], graphics);

                snake.draw(context, graphics, &head_texture, &body_piece_texture, &snake_angle_piece_texture);
                food.draw(context.transform, graphics);

                if snake.dead {
                    let game_over_text = "GAME OVER";
                    let game_over_text_style = text::Text::new_color([1.0, 1.0, 1.0, 1.0], 32);
                    let game_over_text_width = glyphs.width(BUTTON_TEXT_SIZE, game_over_text).unwrap();
                    let game_over_text_x = BUTTON_X - (BUTTON_WIDTH - game_over_text_width) / 2.0;
                    let glyphs = &mut glyphs;

                    game_over_text_style.draw(
                        game_over_text,
                        glyphs,
                        &context.draw_state,
                        context.transform.trans(game_over_text_x, 300.0),
                        graphics,
                    ).unwrap();

                    // Draw the button
                    rectangle(
                        [1.0, 1.0, 1.0, 1.0],
                        [BUTTON_X, BUTTON_Y, BUTTON_WIDTH, BUTTON_HEIGHT],
                        context.transform,
                        graphics,
                    );
                    let text = "RESTART";
                    let text_style = text::Text::new_color([0.0, 0.0, 0.0, 1.0], BUTTON_TEXT_SIZE);
                    let text_width = glyphs.width(BUTTON_TEXT_SIZE, text).unwrap();
                    let text_x = BUTTON_X + (BUTTON_WIDTH - text_width) / 2.0;
                    let text_y = BUTTON_Y + (BUTTON_HEIGHT - BUTTON_TEXT_SIZE as f64) / 2.0 + BUTTON_TEXT_SIZE as f64;

                    text_style.draw(
                        text,
                        glyphs,
                        &context.draw_state,
                        context.transform.trans(text_x, text_y),
                        graphics,
                    ).unwrap();

                    // Update glyphs before rendering.
                    glyphs.factory.encoder.flush(device);
                }
            });
        }

        if let Some(pos) = event.mouse_cursor_args() {
            mouse_pos = pos;
        }

        match event.press_args() {
            Some(Button::Keyboard(Key::Up)) => snake.turn(Direction::Up),
            Some(Button::Keyboard(Key::Down)) => snake.turn(Direction::Down),
            Some(Button::Keyboard(Key::Left)) => snake.turn(Direction::Left),
            Some(Button::Keyboard(Key::Right)) => snake.turn(Direction::Right),
            Some(Button::Mouse(MouseButton::Left)) => {
                if snake.dead {
                    let [mouse_x, mouse_y] = mouse_pos;
                    if mouse_x >= BUTTON_X
                        && mouse_x <= (BUTTON_X + BUTTON_WIDTH)
                        && mouse_y >= BUTTON_Y
                        && mouse_y <= (BUTTON_Y + BUTTON_HEIGHT)
                    {
                        // Restart the game
                        snake = Snake::default();
                        food.regenerate();
                    }
                }
            }
            _ => ()
        }

        if let Some(_) = event.update_args() {
            snake.move_ahead(&mut food);
        }
    }
}
