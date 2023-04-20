use piston_window::{clear, Button, Key, PistonWindow, PressEvent, EventLoop, UpdateEvent, Flip, RenderEvent, WindowSettings, CharacterCache, Glyphs, TextureSettings, Transformed, MouseCursorEvent, MouseButton, Texture, text, rectangle, G2dTextureContext, G2dTexture, Context, G2d, GfxDevice};

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
const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];


fn create_texture(texture_context: &mut G2dTextureContext, image_path: &str) -> G2dTexture {
    Texture::from_path(
        texture_context,
        image_path,
        Flip::None,
        &TextureSettings::new(),
    ).unwrap()
}

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Snake", [WINDOW_SIZE, WINDOW_SIZE])
        .exit_on_esc(true)
        .build()
        .unwrap();
    window.set_ups(4);

    let texture_context = window.create_texture_context();
    let mut glyphs = Glyphs::new("/Library/Fonts/Arial Unicode.ttf", texture_context, TextureSettings::new()).unwrap();
    let mut mouse_pos = [0.0, 0.0];
    let mut texture_context = window.create_texture_context();
    let head_texture = create_texture(&mut texture_context, "images/huilo.png");
    let body_piece_texture = create_texture(&mut texture_context, "images/poop_horizontal.png");
    let snake_angle_piece_texture = create_texture(&mut texture_context, "images/poop_angle.png");

    let mut snake = Snake::default();
    let mut food = Food::new(
        WINDOW_SIZE,
        GRID_SIZE,
        create_texture(&mut texture_context, "images/poop_food.png"),
    );

    while let Some(event) = window.next() {
        if let Some(_) = event.render_args() {
            window.draw_2d(&event, |context, graphics, device| {
                clear([0.0, 0.0, 0.0, 1.0], graphics);

                snake.draw(context, graphics, &head_texture, &body_piece_texture, &snake_angle_piece_texture);
                food.draw(context, graphics);

                if snake.dead {
                    draw_game_over(context, graphics, &mut glyphs, device);
                }
            });
        }

        if let Some(pos) = event.mouse_cursor_args() {
            mouse_pos = pos;
        }

        handle_input_events(&event, &mut snake, &mut food, &mut mouse_pos);

        if let Some(_) = event.update_args() {
            snake.move_ahead(&mut food);
        }
    }
}


fn handle_input_events(
    event: &piston_window::Event,
    snake: &mut Snake,
    food: &mut Food,
    mouse_pos: &mut [f64; 2],
) {
    if let Some(Button::Keyboard(key)) = event.press_args() {
        if !snake.dead {
            match key {
                Key::Up => snake.turn(Direction::Up),
                Key::Down => snake.turn(Direction::Down),
                Key::Left => snake.turn(Direction::Left),
                Key::Right => snake.turn(Direction::Right),
                _ => (),
            }
        }
    }

    if let Some(Button::Mouse(MouseButton::Left)) = event.press_args() {
        if snake.dead {
            let x = mouse_pos[0];
            let y = mouse_pos[1];

            if x >= BUTTON_X
                && x <= BUTTON_X + BUTTON_WIDTH
                && y >= BUTTON_Y
                && y <= BUTTON_Y + BUTTON_HEIGHT
            {
                snake.reset();
                food.regenerate();
            }
        }
    }

    if let Some(_) = event.update_args() {
        snake.move_ahead(food);
    }
}


fn draw_game_over(context: Context, graphics: &mut G2d, glyphs: &mut Glyphs, device: &mut GfxDevice) {
    let game_over_text = "GAME OVER";
    let game_over_text_style = text::Text::new_color([1.0, 1.0, 1.0, 1.0], 32);
    let game_over_text_width = glyphs.width(BUTTON_TEXT_SIZE, game_over_text).unwrap();
    let game_over_text_x = (WINDOW_SIZE - game_over_text_width) / 2.0;
    let game_over_text_y = 250.0;

    let restart_button = rectangle::Rectangle::new(RED);
    let restart_button_text = "RESTART";
    let restart_button_text_style = text::Text::new_color([1.0, 1.0, 1.0, 1.0], BUTTON_TEXT_SIZE);
    let restart_button_text_width = glyphs.width(BUTTON_TEXT_SIZE, restart_button_text).unwrap();
    let restart_button_text_x = (WINDOW_SIZE - restart_button_text_width) / 2.0;
    let restart_button_text_y = BUTTON_Y + BUTTON_HEIGHT - 10.0;

    game_over_text_style.draw(
        game_over_text,
        glyphs,
        &context.draw_state,
        context.transform.trans(game_over_text_x, game_over_text_y),
        graphics,
    ).unwrap();

    restart_button.draw(
        [BUTTON_X, BUTTON_Y, BUTTON_WIDTH, BUTTON_HEIGHT],
        &context.draw_state,
        context.transform,
        graphics,
    );

    restart_button_text_style.draw(
        restart_button_text,
        glyphs,
        &context.draw_state,
        context.transform.trans(restart_button_text_x, restart_button_text_y),
        graphics,
    ).unwrap();

    glyphs.factory.encoder.flush(device);
}
