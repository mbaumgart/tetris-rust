use super::sprite::Sprite;
use piston_window::math::Matrix2d;
use piston_window::{G2d, G2dTexture, ImageSize, Transformed};
use std::collections::HashMap;

pub struct Tetromino {
    sprites: Vec<Sprite>,
}

impl Tetromino {
    pub fn new(texture: &G2dTexture, tetromino_shape: TetrominoShape) -> Tetromino {
        let mut _sprites: Vec<Sprite> = vec![];
        let shapes = create_tetromino_shapes();
        let shape_matrix = shapes.get(&tetromino_shape);
        let (offset_x, offset_y) = texture.get_size();

        // I don't get why I need three loops to iterate a two dimensional array
        for row in shape_matrix.iter() {
            for (y, col) in row.iter().enumerate() {
                for (x, entry) in col.iter().enumerate() {
                    // if we find a positive flag in the matrix create a sprite and position it
                    if *entry {
                        let mut sprite = Sprite::new(texture.clone());
                        let pos_x = offset_x as f64 * x as f64;
                        let pos_y = offset_y as f64 * (y - 1) as f64;
                        sprite.position = [pos_x, pos_y];
                        _sprites.push(sprite);
                    }
                }
            }
        }

        Tetromino { sprites: _sprites }
    }

    pub fn draw(&self, transform: Matrix2d, graphics: &mut G2d) {
        for sprite in &self.sprites {
            sprite.draw(transform, graphics);
        }
    }

    pub fn rotate(&mut self) {}
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
