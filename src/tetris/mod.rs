extern crate find_folder;
extern crate piston_window;

use piston_window::*;
use std::time::SystemTime;

mod assets;
mod config;
mod sprite;
mod tetromino;

pub struct Tetris {
    window: PistonWindow,
    assets: assets::Assets,
    last_update: SystemTime,
}

impl Tetris {
    pub fn new() -> Tetris {
        let mut _window: PistonWindow =
            WindowSettings::new(config::WINDOW_TITLE, config::WINDOW_SIZE)
                .exit_on_esc(true)
                .samples(4)
                .build()
                .unwrap();
        let _assets = assets::Assets::new(&mut _window);

        Tetris {
            window: _window,
            assets: _assets,
            last_update: SystemTime::now(),
        }
    }

    pub fn run(&mut self) {
        let mut tetromino =
            tetromino::Tetromino::new(&self.assets.brick_red, tetromino::TetrominoShape::L);
        let mut sprite_block = sprite::Sprite::new(self.assets.brick_blue.clone());
        sprite_block.translate(
            config::GRID_CELL_SIZE[0] * 1.0,
            config::GRID_CELL_SIZE[1] * 4.0,
        );

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

        if config::UPDATE_MS < self.last_update.elapsed().unwrap().as_millis() {
            tetromino.move_down();
            self.last_update = SystemTime::now();
        }
    }

    fn draw<E>(
        &mut self,
        event: &E,
        sprite_block: &sprite::Sprite,
        tetromino: &mut tetromino::Tetromino,
    ) where
        E: GenericEvent,
    {
        let area_width = config::GRID_CELLS_HORIZONTAL as f64 * config::GRID_CELL_SIZE[0];
        let area_height = config::GRID_CELLS_VERTICAL as f64 * config::GRID_CELL_SIZE[1];

        self.window.draw_2d(event, |context, graphics, _device| {
            clear([0.8; 4], graphics);

            rectangle(
                [0.2, 0.2, 0.4, 1.0], // color
                [0.0, 0.0, area_width, area_height],
                context.transform,
                graphics,
            );

            sprite_block.draw(context.transform, graphics);
            tetromino.draw(context.transform, graphics);
        });
    }
}
