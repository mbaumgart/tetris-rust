use super::config::*;
use super::sprite::Sprite;
use piston_window::math::{Matrix2d, Scalar};
use piston_window::{G2d, G2dTexture};
use std::collections::HashMap;

pub struct Tetromino {
    pub sprites: Vec<Sprite>,
}

impl Tetromino {
    pub fn new(texture: &G2dTexture, tetromino_shape: TetrominoShape) -> Tetromino {
        let mut _sprites: Vec<Sprite> = vec![];
        let shapes = create_tetromino_shapes();
        let shape_matrix_value = shapes.get(&tetromino_shape);

        match shape_matrix_value {
            Some(shape_matrix) => {
                for (y, row) in shape_matrix.iter().enumerate() {
                    for (x, col) in row.iter().enumerate() {
                        // if we find a positive flag in the matrix create a sprite and position it
                        if *col {
                            let mut sprite = Sprite::new(texture.clone());
                            let pos_x = GRID_CELL_SIZE[0] * x as f64;
                            let pos_y = GRID_CELL_SIZE[1] * (y - 1) as f64;
                            sprite.position = [pos_x, pos_y];
                            _sprites.push(sprite);
                        }
                    }
                }
            }
            None => (),
        }

        Tetromino { sprites: _sprites }
    }

    pub fn draw(&self, transform: Matrix2d, graphics: &mut G2d) {
        for sprite in &self.sprites {
            sprite.draw(transform, graphics);
        }
    }

    pub fn translate(&mut self, x: Scalar, y: Scalar) {
        for sprite in &mut self.sprites {
            sprite.translate(x, y);
        }
    }

    pub fn move_left(&mut self) {
        self.translate(-GRID_CELL_SIZE[0], 0.0);
    }

    pub fn move_right(&mut self) {
        self.translate(GRID_CELL_SIZE[0], 0.0);
    }

    pub fn move_down(&mut self) {
        self.translate(0.0, GRID_CELL_SIZE[1]);
    }

    pub fn rotate(&self) {}

    pub fn is_blocked_down(&self, board: &Vec<Sprite>) -> bool {
        let mut is_blocked = false;
        for tetromino_sprite in &self.sprites {
            for board_sprite in board {
                let is_same_column = tetromino_sprite.position[0] == board_sprite.position[0];
                let is_row_below =
                    tetromino_sprite.position[1] + GRID_CELL_SIZE[1] == board_sprite.position[1];
                if is_same_column && is_row_below {
                    is_blocked = true;
                }
            }
        }

        is_blocked
    }

    pub fn is_blocked_left(&self, board: &Vec<Sprite>) -> bool {
        let mut is_blocked = false;
        for tetromino_sprite in &self.sprites {
            for board_sprite in board {
                let is_column_left =
                    tetromino_sprite.position[0] - GRID_CELL_SIZE[0] == board_sprite.position[0];
                let is_same_row = tetromino_sprite.position[1] == board_sprite.position[1];
                if is_column_left && is_same_row {
                    is_blocked = true;
                }
            }
        }

        is_blocked
    }

    pub fn is_blocked_right(&self, board: &Vec<Sprite>) -> bool {
        let mut is_blocked = false;
        for tetromino_sprite in &self.sprites {
            for board_sprite in board {
                let is_column_right =
                    tetromino_sprite.position[0] + GRID_CELL_SIZE[0] == board_sprite.position[0];
                let is_same_row = tetromino_sprite.position[1] == board_sprite.position[1];
                if is_column_right && is_same_row {
                    is_blocked = true;
                }
            }
        }

        is_blocked
    }
}

#[derive(Hash, Eq, PartialEq)]
pub enum TetrominoShape {
    L,
    J,
    D,
    S,
    Z,
    I,
    O,
}

pub fn create_tetromino_shapes() -> HashMap<TetrominoShape, [[bool; 4]; 4]> {
    let mut layouts = HashMap::new();
    layouts.insert(
        TetrominoShape::L,
        [
            [false, false, false, false],
            [true, true, true, false],
            [true, false, false, false],
            [false, false, false, false],
        ],
    );
    layouts.insert(
        TetrominoShape::J,
        [
            [true, false, false, false],
            [true, true, true, false],
            [false, false, false, false],
            [false, false, false, false],
        ],
    );
    layouts.insert(
        TetrominoShape::D,
        [
            [false, false, false, false],
            [true, true, true, false],
            [false, true, false, false],
            [false, false, false, false],
        ],
    );
    layouts.insert(
        TetrominoShape::S,
        [
            [false, false, false, false],
            [false, true, true, false],
            [true, true, false, false],
            [false, false, false, false],
        ],
    );
    layouts.insert(
        TetrominoShape::Z,
        [
            [false, false, false, false],
            [true, true, false, false],
            [false, true, true, false],
            [false, false, false, false],
        ],
    );
    layouts.insert(
        TetrominoShape::I,
        [
            [false, true, false, false],
            [false, true, false, false],
            [false, true, false, false],
            [false, true, false, false],
        ],
    );
    layouts.insert(
        TetrominoShape::O,
        [
            [false, false, false, false],
            [false, true, true, false],
            [false, true, true, false],
            [false, false, false, false],
        ],
    );

    layouts
}
