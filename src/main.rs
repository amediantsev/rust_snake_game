use piston_window::{clear, rectangle, Button, Key, PistonWindow, PressEvent, RenderEvent, WindowSettings};
use piston_window::{EventLoop, UpdateEvent};
use rand::Rng;


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

struct Food {
    x: f64,
    y: f64,
}

fn generate_food_position() -> (f64, f64) {
    let mut rng = rand::thread_rng();
    (
        (rng.gen_range(0..(WINDOW_SIZE as u32 / GRID_SIZE as u32)) * GRID_SIZE as u32) as f64,
        (rng.gen_range(0..(WINDOW_SIZE as u32 / GRID_SIZE as u32)) * GRID_SIZE as u32) as f64,
    )
}

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Snake", [WINDOW_SIZE, WINDOW_SIZE])
        .exit_on_esc(true)
        .build()
        .unwrap();
    window.set_ups(3);

    let mut snake = Snake { x: 0.0, y: 0.0, direction: Direction::Right };
    let mut food = Food {
        x: 0.0,
        y: 0.0,
    };
    let (food_x, food_y) = generate_food_position();
    food.x = food_x;
    food.y = food_y;


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
                // Draw the food
                rectangle(
                    [1.0, 0.0, 0.0, 1.0],
                    [food.x, food.y, GRID_SIZE, GRID_SIZE],
                    context.transform,
                    graphics,
                );

                if snake.x == food.x && snake.y == food.y {
                    // Increase the snake's length and generate a new food position
                    let (new_food_x, new_food_y) = generate_food_position();
                    food.x = new_food_x;
                    food.y = new_food_y;
                }
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
