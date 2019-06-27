use super::sprite::Sprite;
use piston_window::math::{Matrix2d, Scalar};
use piston_window::{G2d, G2dTexture, ImageSize};
use std::collections::HashMap;

pub struct Tetromino {
    pub sprites: Vec<Sprite>,
}

impl Tetromino {
    pub fn new(texture: &G2dTexture, tetromino_shape: TetrominoShape) -> Tetromino {
        let mut _sprites: Vec<Sprite> = vec![];
        let shapes = create_tetromino_shapes();
        let shape_matrix_value = shapes.get(&tetromino_shape);
        let (offset_x, offset_y) = texture.get_size();

        match shape_matrix_value {
            Some(shape_matrix) => {
                for (y, row) in shape_matrix.iter().enumerate() {
                    for (x, col) in row.iter().enumerate() {
                        // if we find a positive flag in the matrix create a sprite and position it
                        if *col {
                            let mut sprite = Sprite::new(texture.clone());
                            let pos_x = offset_x as f64 * x as f64;
                            let pos_y = offset_y as f64 * (y - 1) as f64;
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
        let (x, _y) = self.sprites[0].texture.get_size();
        self.translate(-(x as f64), 0.0);
    }

    pub fn move_right(&mut self) {
        let (x, _y) = self.sprites[0].texture.get_size();
        self.translate(x as f64, 0.0);
    }

    pub fn move_down(&mut self) {
        let (_x, y) = self.sprites[0].texture.get_size();
        self.translate(0.0, y as f64);
    }

    pub fn rotate(&self) {}
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
