extern crate find_folder;

use piston_window::{
    Filter, Flip, G2dTexture, G2dTextureContext, Glyphs, PistonWindow, TextureSettings,
};
use std::path::PathBuf;

pub struct Assets {
    pub font: Glyphs,
    pub brick_blue: G2dTexture,
    pub brick_brown: G2dTexture,
    pub brick_green: G2dTexture,
    pub brick_grey: G2dTexture,
    pub brick_orange: G2dTexture,
    pub brick_pink: G2dTexture,
    pub brick_red: G2dTexture,
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
        let _brick_green = load_texture("brick_green.png", &mut texture_context, &assets_folder);
        let _brick_grey = load_texture("brick_grey.png", &mut texture_context, &assets_folder);
        let _brick_orange = load_texture("brick_orange.png", &mut texture_context, &assets_folder);
        let _brick_pink = load_texture("brick_pink.png", &mut texture_context, &assets_folder);
        let _brick_red = load_texture("brick_red.png", &mut texture_context, &assets_folder);
        let _mainboard = load_texture("mainboard.png", &mut texture_context, &assets_folder);

        Assets {
            font: _font,
            brick_blue: _brick_blue,
            brick_brown: _brick_brown,
            brick_green: _brick_green,
            brick_grey: _brick_grey,
            brick_orange: _brick_orange,
            brick_pink: _brick_pink,
            brick_red: _brick_red,
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
