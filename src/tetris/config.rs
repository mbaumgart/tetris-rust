use super::structs::Size;
use super::tetromino::TetrominoShape;
use super::tetromino::create_tetromino_shapes;
use std::collections::HashMap;

pub struct Config {
    pub window_title: String,
    pub window_size: Size,
    pub bricks_horizontal: u8,
    pub bricks_vertical: u8,
    pub brick_size: Size,
    pub gravity: f32,
    pub drop_speed: f32,
    pub tetromino_shape_map: HashMap<TetrominoShape, [[bool; 4]; 4]>,
}

impl Config {
    pub fn new() -> Config {
        Config {
            window_title: String::from("Tetris Game"),
            window_size: Size::new(400.0, 600.0),
            bricks_horizontal: 10,
            bricks_vertical: 18,
            brick_size: Size::new(28.0, 28.0),
            gravity: 0.015,
            drop_speed: 30.0,
            tetromino_shape_map: create_tetromino_shapes(),
        }
    }
}
