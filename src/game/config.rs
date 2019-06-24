pub struct Config {
    pub window_title: String,
    pub window_width: u32,
    pub window_height: u32,
}

impl Config {
    pub fn new() -> Config {
        Config {
            window_title: "Tetris Game".to_string(),
            window_width: 400,
            window_height: 800,
        }
    }
}
