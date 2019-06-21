extern crate find_folder;
extern crate piston;
extern crate piston_window;
extern crate sprite;

use ai_behavior::{Action, Sequence, Wait, WaitForever, While};
use piston::input::*;
use piston_window::*;
use sprite::*;
use std::rc::Rc;


fn main() {
    let (width, height) = (800, 500);
    let mut window: PistonWindow = WindowSettings::new("Tetris Game", [width, height])
        .exit_on_esc(true)
        .samples(4)
        .build()
        .unwrap();

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();

    let id_mainboard;
    let mut scene = Scene::new();

    let mut glyphs = window
        .load_font(assets.join("FiraSans-Regular.ttf"))
        .unwrap();

    let tex_mainboard = Rc::new(
        Texture::from_path(
            &mut window.create_texture_context(),
            assets.join("mainboard.png"),
            Flip::None,
            &TextureSettings::new().min(Filter::Nearest),
        )
        .unwrap(),
    );
    let mut sprite_mainboard = Sprite::from_texture(tex_mainboard.clone());
    sprite_mainboard.set_position(width as f64 / 2.0, height as f64 / 2.0);
    id_mainboard = scene.add_child(sprite_mainboard);

    // let anim_rotate = Action(Ease(
    //     EaseFunction::ExponentialInOut,
    //     Box::new(RotateTo(2.0, 360.0)),
    // ));
    // let anim_rotate = Action(MoveBy(3.0, 0.0, 100.0));
    let anim_rotate = Action(Ease(
        EaseFunction::CubicInOut,
        Box::new(MoveBy(2.0, 0.0, 100.0)),
    ));
    scene.run(id_mainboard, &anim_rotate);

    // update loop
    while let Some(event) = window.next() {
        scene.event(&event);
        window.draw_2d(&event, |context, mut graphics, _device| {
            draw(&context, &mut graphics);
            scene.draw(context.transform, graphics);

            // image(&mainboard, context.transform, graphics);

            // Text::new_color([0.0, 1.0, 0.0, 1.0], 32)
            //     .draw(
            //         "Hello world!",
            //         &mut glyphs,
            //         &context.draw_state,
            //         context.transform,
            //         graphics,
            //     )
            //     .unwrap();
            // glyphs.factory.encoder.flush(_device);
        });
    }
}

fn draw(context: &Context, graphics: &mut G2d) {
    clear([0.8; 4], graphics);

    rectangle(
        [1.0, 0.0, 0.0, 1.0], // red
        [200.0, 200.0, 100.0, 100.0],
        context.transform,
        graphics,
    );
}
