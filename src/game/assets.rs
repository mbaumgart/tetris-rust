extern crate find_folder;

use piston_window::{
    Filter, Flip, G2dTexture, G2dTextureContext, Glyphs, PistonWindow, TextureSettings,
};
use std::path::PathBuf;

pub struct Assets {
    pub font: Glyphs,
    pub brick_blue: G2dTexture,
    pub brick_brown: G2dTexture,
    pub mainboard: G2dTexture,
}

impl Assets {
    pub fn new(window: &mut PistonWindow) -> Assets {
        let assets_folder = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("assets")
            .unwrap();

        let mut _font = window
            .load_font(assets_folder.join("FiraSans-Regular.ttf"))
            .unwrap();

        let mut texture_context = window.create_texture_context();

        let _brick_blue = load_texture("brick_blue.png", &mut texture_context, &assets_folder);
        let _brick_brown = load_texture("brick_brown.png", &mut texture_context, &assets_folder);
        let _mainboard = load_texture("mainboard.png", &mut texture_context, &assets_folder);

        Assets {
            font: _font,
            brick_blue: _brick_blue,
            brick_brown: _brick_brown,
            mainboard: _mainboard,
        }
    }
}

fn load_texture(
    file_name: &str,
    texture_context: &mut G2dTextureContext,
    assets_folder: &PathBuf,
) -> G2dTexture {
    G2dTexture::from_path(
        texture_context,
        assets_folder.join(file_name),
        Flip::None,
        &TextureSettings::new().mag(Filter::Nearest),
    )
    .unwrap()
}
