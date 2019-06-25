pub struct Size {
    pub width: f64,
    pub height: f64
}

impl Size {
    pub fn to_array(&self) -> [f64; 2] {
        [self.width, self.height]
    }
}
