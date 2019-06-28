use super::tetromino::{create_tetromino_shapes, TetrominoShape};
use piston_window::math::Vec2d;
use std::collections::HashMap;

pub struct Config {
    pub window_title: String,
    pub window_size: Vec2d,
    pub bricks_horizontal: u8,
    pub bricks_vertical: u8,
    pub brick_size: Vec2d,
    pub update_ms: u128,
    pub drop_speed: f32,
    pub tetromino_shape_map: HashMap<TetrominoShape, [[bool; 4]; 4]>,
}

impl Config {
    pub fn new() -> Config {
        Config {
            window_title: String::from("Tetris"),
            window_size: [280.0, 504.0],
            bricks_horizontal: 10,
            bricks_vertical: 18,
            brick_size: [28.0, 28.0],
            update_ms: 400,
            drop_speed: 30.0,
            tetromino_shape_map: create_tetromino_shapes(),
        }
    }
}
