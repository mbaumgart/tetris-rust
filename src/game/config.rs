use std::collections::HashMap;

pub struct Config {
    pub window_title: String,
    pub window_width: u32,
    pub window_height: u32,
    pub bricks_horizontal: u8,
    pub bricks_vertical: u8,
    pub gravity: f32,
    pub accelerated_gravity: f32,
    pub brick_shapes: HashMap<BrickTypes, [[bool; 4]; 4]>,
}

impl Config {
    pub fn new() -> Config {
        Config {
            window_title: String::from("Tetris Game"),
            window_width: 400,
            window_height: 800,
            bricks_horizontal: 10,
            bricks_vertical: 18,
            gravity: 0.015,
            accelerated_gravity: 30.0,
            brick_shapes: create_brick_shapes(),
        }
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
