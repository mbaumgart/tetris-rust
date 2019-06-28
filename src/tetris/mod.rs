extern crate find_folder;
extern crate piston_window;

mod assets;
mod config;
mod sprite;
mod tetromino;

use piston_window::*;
use std::time::SystemTime;

pub struct Tetris {
    window: PistonWindow,
    assets: assets::Assets,
    board: Vec<sprite::Sprite>,
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
            board: Vec::new(),
            last_update: SystemTime::now(),
        }
    }

    pub fn run(&mut self) {
        let mut tetromino =
            tetromino::Tetromino::new(&self.assets.brick_red, tetromino::TetrominoShape::L);
        let mut sprite_block = sprite::Sprite::new(self.assets.brick_blue.clone());
        sprite_block.translate(
            config::GRID_CELL_SIZE[0] * 4.0,
            config::GRID_CELL_SIZE[1] * 10.0,
        );
        self.board.push(sprite_block);

        // game loop
        while let Some(event) = self.window.next() {
            self.update(&event, &mut tetromino);
            self.draw(&event, &mut tetromino);
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
                    self.last_update = SystemTime::now();
                    if !tetromino.is_blocked_down(&self.board) {
                        tetromino.move_down();
                    }
                }
                Key::Left => {
                    if !tetromino.is_blocked_left(&self.board) {
                        tetromino.move_left()
                    }
                }
                Key::Right => {
                    if !tetromino.is_blocked_right(&self.board) {
                        tetromino.move_right()
                    }
                }
                _ => (),
            }
        };

        if config::UPDATE_MS < self.last_update.elapsed().unwrap().as_millis() {
            self.last_update = SystemTime::now();
            if tetromino.is_blocked_down(&self.board) {
                // TODO: lock in place
            } else {
                tetromino.move_down();
            }
        }
    }

    fn draw<E>(&mut self, event: &E, tetromino: &mut tetromino::Tetromino)
    where
        E: GenericEvent,
    {
        let area_width = config::GRID_CELLS_HORIZONTAL as f64 * config::GRID_CELL_SIZE[0];
        let area_height = config::GRID_CELLS_VERTICAL as f64 * config::GRID_CELL_SIZE[1];
        let sprite_block = self.board.get(0).unwrap();

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
