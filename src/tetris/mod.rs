extern crate find_folder;
extern crate piston_window;

use piston_window::*;
use std::time::SystemTime;

mod assets;
mod config;
mod sprite;
mod structs;
mod tetromino;

pub struct Tetris {
    config: config::Config,
    window: PistonWindow,
    assets: assets::Assets,
    last_update: SystemTime,
}

impl Tetris {
    pub fn new() -> Tetris {
        let mut _config = config::Config::new();
        let mut _window: PistonWindow =
            WindowSettings::new(_config.window_title.clone(), _config.window_size.to_array())
                .exit_on_esc(true)
                .samples(4)
                .build()
                .unwrap();
        let _assets = assets::Assets::new(&mut _window);

        Tetris {
            config: _config,
            window: _window,
            assets: _assets,
            last_update: SystemTime::now(),
        }
    }

    pub fn run(&mut self) {
        let mut tetromino =
            tetromino::Tetromino::new(&self.assets.brick_red, tetromino::TetrominoShape::L);
        let sprite_block = sprite::Sprite::new(self.assets.brick_blue.clone());

        // game loop
        while let Some(event) = self.window.next() {
            self.update(&event, &mut tetromino);
            self.draw(&event, &sprite_block, &mut tetromino);
        }
    }

    fn update<E>(&mut self, event: &E, tetromino: &mut tetromino::Tetromino)
    where
        E: GenericEvent,
    {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            match key {
                Key::Up => tetromino.rotate(),
                Key::Down => {
                    // reset the time to prevent double down movement
                    self.last_update = SystemTime::now();
                    tetromino.move_down();
                }
                Key::Left => tetromino.move_left(),
                Key::Right => tetromino.move_right(),
                _ => (),
            }
        };

        if self.config.update_ms < self.last_update.elapsed().unwrap().as_millis() {
            tetromino.move_down();
            self.last_update = SystemTime::now();
        }


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

            sprite_block.draw(context.transform, graphics);
            tetromino.draw(context.transform, graphics);
        });
    }
}
