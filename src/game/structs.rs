pub struct Size {
    pub width: f64,
    pub height: f64
}

impl Size {
    pub fn new(width: f64, height: f64) -> Size {
        Size {
            width,
            height
        }
    }
    pub fn to_array(&self) -> [f64; 2] {
        [self.width, self.height]
    }
}
