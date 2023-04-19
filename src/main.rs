use piston_window::{
    clear, Button, Key, PistonWindow, PressEvent, EventLoop, UpdateEvent, Flip,
    RenderEvent, WindowSettings, CharacterCache, Glyphs, TextureSettings, Transformed,
    MouseCursorEvent, MouseButton, Texture, text, rectangle,
};

mod snake;
mod food;

use snake::{Snake, Direction};
use food::Food;

const GRID_SIZE: f64 = 30.0;
const WINDOW_SIZE: f64 = 600.0;

const BUTTON_WIDTH: f64 = 150.0;
const BUTTON_HEIGHT: f64 = 50.0;
const BUTTON_X: f64 = (WINDOW_SIZE - BUTTON_WIDTH) / 2.0;
const BUTTON_Y: f64 = 350.0;
const BUTTON_TEXT_SIZE: u32 = 24;


fn main() {
    let mut window: PistonWindow = WindowSettings::new("Snake", [WINDOW_SIZE, WINDOW_SIZE])
        .exit_on_esc(true)
        .build()
        .unwrap();
    window.set_ups(8);

    let mut food = Food::new(WINDOW_SIZE, GRID_SIZE);
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
