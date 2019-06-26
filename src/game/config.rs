use super::structs::Size;
use std::collections::HashMap;

pub struct Config {
    pub window_title: String,
    pub window_size: Size,
    pub bricks_horizontal: u8,
    pub bricks_vertical: u8,
    pub brick_size: Size,
    pub gravity: f32,
    pub drop_speed: f32,
    pub brick_shapes: HashMap<BrickTypes, [[bool; 4]; 4]>,
}

impl Config {
    pub fn new() -> Config {
        Config {
            window_title: String::from("Tetris Game"),
            window_size: Size {
                width: 400.0,
                height: 600.0,
            },
            bricks_horizontal: 10,
            bricks_vertical: 18,
            brick_size: Size {
                width: 16.0,
                height: 16.0,
            },
            gravity: 0.015,
            drop_speed: 30.0,
            brick_shapes: create_brick_shapes(),
        }
    }
    // let (height, width) = self.get_brick_size()
    // let (width, height) = myconfig.brick_size
    // fn get_brick_size(&self) -> (u8,u8)

    pub fn set_brick_size(&mut self, width: f64, height: f64) {
        self.brick_size = Size {
            width: width,
            height: height,
        };
    }
}

#[derive(Hash, Eq, PartialEq)]
pub enum BrickTypes {
    L,
    J,
    D,
    S,
    Z,
    I,
    O,
}

fn create_brick_shapes() -> HashMap<BrickTypes, [[bool; 4]; 4]> {
    let mut layouts = HashMap::new();
    layouts.insert(
        BrickTypes::L,
        [
            [false, false, false, false],
            [true, true, true, false],
            [true, false, false, false],
            [false, false, false, false],
        ],
    );
    layouts.insert(
        BrickTypes::J,
        [
            [true, false, false, false],
            [true, true, true, false],
            [false, false, false, false],
            [false, false, false, false],
        ],
    );
    layouts.insert(
        BrickTypes::D,
        [
            [false, false, false, false],
            [true, true, true, false],
            [false, true, false, false],
            [false, false, false, false],
        ],
    );
    layouts.insert(
        BrickTypes::S,
        [
            [false, false, false, false],
            [false, true, true, false],
            [true, true, false, false],
            [false, false, false, false],
        ],
    );
    layouts.insert(
        BrickTypes::Z,
        [
            [false, false, false, false],
            [true, true, false, false],
            [false, true, true, false],
            [false, false, false, false],
        ],
    );
    layouts.insert(
        BrickTypes::I,
        [
            [false, true, false, false],
            [false, true, false, false],
            [false, true, false, false],
            [false, true, false, false],
        ],
    );
    layouts.insert(
        BrickTypes::O,
        [
            [false, false, false, false],
            [false, true, true, false],
            [false, true, true, false],
            [false, false, false, false],
        ],
    );

    layouts
}
