extern crate piston_window;

use piston_window::{clear, rectangle, Button, Key, PistonWindow, PressEvent, RenderEvent, WindowSettings};
use piston_window::{EventLoop, UpdateEvent};

const GRID_SIZE: f64 = 20.0;
const WINDOW_SIZE: f64 = 600.0;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Snake {
    x: f64,
    y: f64,
    direction: Direction,
}

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Snake", [WINDOW_SIZE, WINDOW_SIZE])
        .exit_on_esc(true)
        .build()
        .unwrap();
    window.set_ups(3);

    let mut snake = Snake { x: 0.0, y: 0.0, direction: Direction::Right };

    while let Some(event) = window.next() {
        if let Some(_) = event.render_args() {
            window.draw_2d(&event, |context, graphics, _| {
                // Clear the window
                clear([0.0, 0.0, 0.0, 1.0], graphics);

                rectangle(
                    [1.0, 1.0, 1.0, 1.0],
                    [snake.x, snake.y, GRID_SIZE, GRID_SIZE],
                    context.transform,
                    graphics,
                );
            });
        }

        match event.press_args() {
            Some(Button::Keyboard(Key::Up)) => {
                snake.direction = Direction::Up;
            }
            Some(Button::Keyboard(Key::Down)) => {
                snake.direction = Direction::Down;
            }
            Some(Button::Keyboard(Key::Left)) => {
                snake.direction = Direction::Left;
            }
            Some(Button::Keyboard(Key::Right)) => {
                snake.direction = Direction::Right;
            }
            _ => ()
        }

        if let Some(_) = event.update_args() {
            match snake.direction {
                Direction::Up => snake.y -= GRID_SIZE,
                Direction::Down => snake.y += GRID_SIZE,
                Direction::Left => snake.x -= GRID_SIZE,
                Direction::Right => snake.x += GRID_SIZE,
            }
        }
    }
}
