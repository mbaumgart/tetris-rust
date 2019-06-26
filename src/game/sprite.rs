use piston_window::math::{Matrix2d, Vec2d};
use piston_window::{DrawState, G2d, G2dTexture, Image, Transformed};

pub struct Sprite {
    pub texture: G2dTexture,
    pub position: Vec2d,
    pub scale: Vec2d,
    pub color: [f32; 3],
    pub opacity: f32,
}

impl Sprite {
    pub fn new(texture: G2dTexture) -> Sprite {
        Sprite {
            texture: texture,
            position: [0.0, 0.0],
            scale: [1.0, 1.0],
            color: [1.0, 1.0, 1.0],
            opacity: 1.0,
        }
    }

    pub fn draw(&self, transform: Matrix2d, graphics: &mut G2d) {
        let t = transform
            .trans(self.position[0], self.position[1])
            .scale(self.scale[0], self.scale[1]);
            
        Image::new()
            .color([self.color[0], self.color[1], self.color[2], self.opacity])
            .draw(&self.texture, &DrawState::default(), t, graphics);
    }
}
