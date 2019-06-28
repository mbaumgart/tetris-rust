extern crate find_folder;
extern crate piston_window;

mod assets;
mod config;
mod sprite;
mod tetromino;

use self::assets::*;
use self::config::*;
use self::sprite::*;
use self::tetromino::*;
use piston_window::*;
use std::time::SystemTime;

pub struct Tetris {
    window: PistonWindow,
    assets: Assets,
    board: Vec<Sprite>,
    tetromino: Tetromino,
    last_update: SystemTime,
}

impl Tetris {
    pub fn new() -> Tetris {
        let mut _window: PistonWindow = WindowSettings::new(WINDOW_TITLE, WINDOW_SIZE)
            .exit_on_esc(true)
            .samples(4)
            .build()
            .unwrap();
        let _assets = Assets::new(&mut _window);
        let _tetromino =
            tetromino::Tetromino::new(&_assets.brick_red, tetromino::TetrominoShape::L);

        Tetris {
            window: _window,
            assets: _assets,
            board: Vec::new(),
            tetromino: _tetromino,
            last_update: SystemTime::now(),
        }
    }

    pub fn run(&mut self) {
        let mut sprite_block = Sprite::new(self.assets.brick_blue.clone());
        sprite_block.translate(GRID_CELL_SIZE[0] * 4.0, GRID_CELL_SIZE[1] * 10.0);
        self.board.push(sprite_block);

        // game loop
        while let Some(event) = self.window.next() {
            self.update(&event);
            self.draw(&event);
        }
    }

    fn update<E>(&mut self, event: &E)
    where
        E: GenericEvent,
    {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            match key {
                Key::Up => self.tetromino.rotate(),
                Key::Down => {
                    self.last_update = SystemTime::now();
                    if !self.tetromino.is_blocked_down(&self.board) {
                        self.tetromino.move_down();
                    }
                }
                Key::Left => {
                    if !self.tetromino.is_blocked_left(&self.board) {
                        self.tetromino.move_left()
                    }
                }
                Key::Right => {
                    if !self.tetromino.is_blocked_right(&self.board) {
                        self.tetromino.move_right()
                    }
                }
                _ => (),
            }
        };

        if UPDATE_MS < self.last_update.elapsed().unwrap().as_millis() {
            self.last_update = SystemTime::now();
            if self.tetromino.is_blocked_down(&self.board) {
                self.tetromino.detach(&mut self.board);
                self.tetromino =
                    tetromino::Tetromino::new(&self.assets.brick_red, tetromino::TetrominoShape::L);
            } else {
                self.tetromino.move_down();
            }
        }
    }

    fn draw<E>(&mut self, event: &E)
    where
        E: GenericEvent,
    {
        let area_width = GRID_CELLS_HORIZONTAL as f64 * GRID_CELL_SIZE[0];
        let area_height = GRID_CELLS_VERTICAL as f64 * GRID_CELL_SIZE[1];
        let board = &self.board;
        let tetromino = &self.tetromino;

        self.window.draw_2d(event, |context, graphics, _device| {
            clear([0.8; 4], graphics);

            rectangle(
                [0.2, 0.2, 0.4, 1.0], // color
                [0.0, 0.0, area_width, area_height],
                context.transform,
                graphics,
            );

            for sprite in board {
                sprite.draw(context.transform, graphics);
            }
            tetromino.draw(context.transform, graphics);
        });
    }
}
