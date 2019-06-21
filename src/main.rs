extern crate find_folder;
extern crate piston;
extern crate piston_window;
extern crate sprite;

use ai_behavior::Action;
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
    // window.set_capture_cursor(true);

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();

    let id_block;
    let mut scene = Scene::new();

    let mut glyphs = window
        .load_font(assets.join("FiraSans-Regular.ttf"))
        .unwrap();

    let mut texture_context = window.create_texture_context();

    let tex_block = Rc::new(
        Texture::from_path(
            &mut texture_context,
            assets.join("block.png"),
            Flip::None,
            &TextureSettings::new().mag(Filter::Nearest),
        )
        .unwrap(),
    );
    let mut sprite_block = Sprite::from_texture(tex_block.clone());
    sprite_block.set_scale(4.0, 4.0);
    sprite_block.set_position(width as f64 / 2.0, height as f64 / 2.0);
    id_block = scene.add_child(sprite_block);

    let anim_move_up = Action(MoveBy(0.1, 0.0, -100.0));
    let anim_move_down = Action(MoveBy(0.1, 0.0, 100.0));
    let anim_move_left = Action(MoveBy(0.1, -100.0, 0.0));
    let anim_move_right = Action(MoveBy(0.1, 100.0, 0.0));

    // update loop
    while let Some(event) = window.next() {
        scene.event(&event);

        if let Some(Button::Keyboard(key)) = event.press_args() {
            match key {
                Key::W => scene.run(id_block, &anim_move_up),
                Key::S => scene.run(id_block, &anim_move_down),
                Key::A => scene.run(id_block, &anim_move_left),
                Key::D => scene.run(id_block, &anim_move_right),
                _ => println!(""),
            }
        };

        window.draw_2d(&event, |context, graphics, _device| {
            clear([0.8; 4], graphics);

            // rectangle(
            //     [1.0, 0.0, 0.0, 1.0], // red
            //     [200.0, 200.0, 100.0, 100.0],
            //     context.transform,
            //     graphics,
            // );

            // draw(&context, &mut graphics);
            scene.draw(context.transform, graphics);

            Text::new_color([0.0, 1.0, 0.0, 1.0], 32)
                .draw(
                    "Hello world!",
                    &mut glyphs,
                    &context.draw_state,
                    context.transform,
                    graphics,
                )
                .unwrap();
            glyphs.factory.encoder.flush(_device);
        });
    }
}
