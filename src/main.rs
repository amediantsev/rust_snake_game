use piston_window::{clear, rectangle, Button, Key, PistonWindow, PressEvent, RenderEvent, WindowSettings, CharacterCache, G2d};
use piston_window::{EventLoop, UpdateEvent};
use piston_window::{text, Glyphs, TextureSettings, Transformed};
use piston_window::{MouseCursorEvent, MouseButton};
use piston_window::types::Color;
use piston_window::math::Matrix2d;
use rand::Rng;

const GRID_SIZE: f64 = 20.0;
const WINDOW_SIZE: f64 = 600.0;

const BUTTON_WIDTH: f64 = 150.0;
const BUTTON_HEIGHT: f64 = 50.0;
const BUTTON_X: f64 = (WINDOW_SIZE - BUTTON_WIDTH) / 2.0;
const BUTTON_Y: f64 = 350.0;
const BUTTON_TEXT_SIZE: u32 = 24;
const BRIGHT_GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];


enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Cell {
    x: f64,
    y: f64,
    color: Color,
}

impl Cell {
    fn draw(&self, transform: Matrix2d, graphics: &mut G2d) {
        rectangle(
            self.color,
            [self.x, self.y, GRID_SIZE, GRID_SIZE],
            transform,
            graphics,
        );
    }
}

impl PartialEq for Cell {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

struct Snake {
    coordinates: Vec<Cell>,
    direction: Direction,
    dead: bool,
}

impl Default for Snake {
    fn default() -> Self {
        Snake {
            coordinates: vec![Cell { x: 0.0, y: 0.0, color: BRIGHT_GREEN }],
            direction: Direction::Right,
            dead: false,
        }
    }
}

impl Snake {
    fn current_head(&self) -> &Cell { self.coordinates.last().unwrap() }

    fn calculate_new_head(&self) -> Cell {
        let position = self.current_head();
        match self.direction {
            Direction::Up => {
                let new_y = position.y - GRID_SIZE;
                Cell { x: position.x, y: if new_y < 0.0 { WINDOW_SIZE } else { new_y }, color: BRIGHT_GREEN }
            }
            Direction::Down => {
                let new_y = position.y + GRID_SIZE;
                Cell { x: position.x, y: if new_y > WINDOW_SIZE { 0.0 } else { new_y }, color: BRIGHT_GREEN }
            }
            Direction::Left => {
                let new_x = position.x - GRID_SIZE;
                Cell { x: if new_x < 0.0 { WINDOW_SIZE } else { new_x }, y: position.y, color: BRIGHT_GREEN }
            }
            Direction::Right => {
                let new_x = position.x + GRID_SIZE;
                Cell { x: if new_x > WINDOW_SIZE { 0.0 } else { new_x }, y: position.y, color: BRIGHT_GREEN }
            }
        }
    }
}

fn generate_food() -> Cell {
    let mut rng = rand::thread_rng();
    Cell {
        x: (rng.gen_range(0..(WINDOW_SIZE as u32 / GRID_SIZE as u32)) * GRID_SIZE as u32) as f64,
        y: (rng.gen_range(0..(WINDOW_SIZE as u32 / GRID_SIZE as u32)) * GRID_SIZE as u32) as f64,
        color: [1.0, 0.0, 0.0, 1.0],
    }
}

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Snake", [WINDOW_SIZE, WINDOW_SIZE])
        .exit_on_esc(true)
        .build()
        .unwrap();
    window.set_ups(7);

    let mut snake = Snake::default();
    let mut food = generate_food();
    let texture_context = window.create_texture_context();
    let mut glyphs = Glyphs::new("/Library/Fonts/Arial Unicode.ttf", texture_context, TextureSettings::new()).unwrap();
    let mut mouse_pos = [0.0, 0.0];

    while let Some(event) = window.next() {
        if let Some(_) = event.render_args() {
            window.draw_2d(&event, |context, graphics, device| {
                // Clear the window
                clear([0.0, 0.0, 0.0, 1.0], graphics);

                // Draw the snake
                let head = snake.current_head();
                for body_part in &snake.coordinates {
                    body_part.draw(context.transform, graphics);
                }
                // Draw the food
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

                if head.x == food.x && head.y == food.y {
                    // Increase the snake's length and generate a new food position
                    food = generate_food();
                }
            });
        }

        if let Some(pos) = event.mouse_cursor_args() {
            mouse_pos = pos;
        }

        match event.press_args() {
            Some(Button::Keyboard(Key::Up)) => if let Direction::Down = snake.direction {} else {
                snake.direction = Direction::Up
            },
            Some(Button::Keyboard(Key::Down)) => if let Direction::Up = snake.direction {} else {
                snake.direction = Direction::Down
            },
            Some(Button::Keyboard(Key::Left)) => if let Direction::Right = snake.direction {} else {
                snake.direction = Direction::Left
            },
            Some(Button::Keyboard(Key::Right)) => if let Direction::Left = snake.direction {} else {
                snake.direction = Direction::Right
            },
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
                        food = generate_food();
                    }
                }
            }
            _ => ()
        }

        if let Some(_) = event.update_args() {
            let new_head = snake.calculate_new_head();

            // Check if the snake's new head position encounters its own body
            if snake.coordinates.contains(&new_head) {
                snake.dead = true;
            } else {
                if new_head.x == food.x && new_head.y == food.y {
                    // Increase the snake's length and generate a new food position
                    food = generate_food();
                } else {
                    // Remove the tail
                    snake.coordinates.remove(0);
                }

                // Add the new head position
                snake.coordinates.push(new_head);
            }
        }
    }
}
