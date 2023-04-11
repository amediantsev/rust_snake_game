extern crate piston_window;
use piston_window::{clear, rectangle, Button, Key, PistonWindow, PressEvent, RenderEvent, WindowSettings, Transformed};

const GRID_SIZE: f64 = 20.0;
const WINDOW_SIZE: f64 = 600.0;

struct Snake {
    x: f64,
    y: f64,
}

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Snake", [WINDOW_SIZE, WINDOW_SIZE])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut snake = Snake { x: 0.0, y: 0.0 };

    while let Some(event) = window.next() {
        if let Some(_) = event.render_args() {
            window.draw_2d(&event, |c, g, _| {
                // Clear the window
                // clear([0.0, 0.0, 0.0, 1.0], g);

                // Draw the snake
                // rectangle(
                //     [1.0, 1.0, 1.0, 1.0],
                //     [0.0, 0.0, GRID_SIZE, GRID_SIZE],
                //     c.transform.trans(0.0, 0.0),
                //     g,
                // );
                rectangle(
                    [1.0, 1.0, 1.0, 1.0],
                    [snake.x, snake.y, GRID_SIZE, GRID_SIZE],
                    c.transform,
                    g,
                );

            });
        }

        if let Some(button) = event.press_args() {
            if button == Button::Keyboard(Key::W) {
                snake.y -= GRID_SIZE;
            } else if button == Button::Keyboard(Key::S) {
                snake.y += GRID_SIZE;
            } else if button == Button::Keyboard(Key::A) {
                snake.x -= GRID_SIZE;
            } else if button == Button::Keyboard(Key::D) {
                snake.x += GRID_SIZE;
            }
        }
    }
}
