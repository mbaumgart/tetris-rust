extern crate find_folder;
extern crate piston_window;

use piston_window::*;
use GenericEvent;

mod assets;
mod config;
mod sprite;
mod structs;
mod tetromino;

pub struct Game {
    config: config::Config,
    window: PistonWindow,
    assets: assets::Assets,
}

impl Game {
    pub fn new() -> Game {
        let mut _config = config::Config::new();
        let mut _window: PistonWindow =
            WindowSettings::new(_config.window_title.clone(), _config.window_size.to_array())
                .exit_on_esc(true)
                .samples(4)
                .build()
                .unwrap();
        _window.set_lazy(true);
        let _assets = assets::Assets::new(&mut _window);

        Game {
            config: _config,
            window: _window,
            assets: _assets,
        }
    }

    pub fn run(&mut self) {
        self.config.set_brick_size(
            self.assets.brick_blue.get_width() as f64,
            self.assets.brick_blue.get_height() as f64,
        );

        let mut tetromino =
            tetromino::Tetromino::new(&self.assets.brick_red, tetromino::TetrominoShape::L);
        let mut sprite_block = sprite::Sprite::new(self.assets.brick_blue.clone());
        let sprite_block_2 = sprite::Sprite::new(self.assets.brick_green.clone());

        // game loop
        while let Some(event) = self.window.next() {
            self.update(&event, &mut sprite_block, &mut tetromino);
            self.draw(&event, &sprite_block, &sprite_block_2, &mut tetromino);
        }
    }

    fn update<E>(
        &mut self,
        event: &E,
        sprite_block: &mut sprite::Sprite,
        tetromino: &mut tetromino::Tetromino,
    ) where
        E: GenericEvent,
    {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            match key {
                Key::Up => tetromino.rotate(),
                Key::Down => tetromino.translate(0.0, self.config.brick_size.height),
                Key::Left => tetromino.translate(-self.config.brick_size.width, 0.0),
                Key::Right => tetromino.translate(self.config.brick_size.width, 0.0),
                _ => (),
            }
        };

        // if let Some(button) = event.release_args() {
        //     match button {
        //         Button::Keyboard(key) => println!("Released keyboard key '{:?}'", key),
        //         Button::Mouse(button) => println!("Released mouse button '{:?}'", button),
        //         Button::Controller(button) => println!("Released controller button '{:?}'", button),
        //         Button::Hat(hat) => println!("Released controller hat `{:?}`", hat),
        //     }
        // };

        // event.mouse_cursor(|pos| {
        //     println!("Mouse moved '{} {}'", pos[0], pos[1]);
        // });
    }

    fn draw<E>(
        &mut self,
        event: &E,
        sprite_block: &sprite::Sprite,
        sprite_block_2: &sprite::Sprite,
        tetromino: &mut tetromino::Tetromino,
    ) where
        E: GenericEvent,
    {
        let area_width = self.config.bricks_horizontal as f64 * self.config.brick_size.width;
        let area_height = self.config.bricks_vertical as f64 * self.config.brick_size.height;

        self.window.draw_2d(event, |context, graphics, _device| {
            clear([0.8; 4], graphics);

            rectangle(
                [0.0, 0.0, 0.0, 1.0], // color
                [0.0, 0.0, area_width, area_height],
                context.transform,
                graphics,
            );

            sprite_block_2.draw(context.transform, graphics);
            sprite_block.draw(context.transform, graphics);
            tetromino.draw(context.transform, graphics);
        });
    }
}
