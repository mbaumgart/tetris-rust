extern crate find_folder;
extern crate piston;
extern crate piston_window;
extern crate sprite;

use ai_behavior::Action;
use piston::input::*;
use piston_window::*;
use sprite::*;
use std::rc::Rc;

// mod board;

pub struct Game {
    window_width: u32,
    window_height: u32,
}

impl Game {
    pub fn new() -> Game {
        Game {
            window_width: 400,
            window_height: 600,
        }
    }

    pub fn run(&mut self) {
        let mut window: PistonWindow = WindowSettings::new("Tetris Game", [self.window_width, self.window_height])
            .exit_on_esc(true)
            .samples(4)
            .build()
            .unwrap();
        let assets = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("assets")
            .unwrap();

        let mut glyphs = window
            .load_font(assets.join("FiraSans-Regular.ttf"))
            .unwrap();

        let mut texture_context = window.create_texture_context();

        let tex_mainboard = Rc::new(
            Texture::from_path(
                &mut texture_context,
                assets.join("mainboard.png"),
                Flip::None,
                &TextureSettings::new().mag(Filter::Nearest),
            )
            .unwrap(),
        );
        let mut sprite_mainboard = Sprite::from_texture(tex_mainboard.clone());
        sprite_mainboard.set_scale(0.5, 0.5);
        sprite_mainboard.set_position(self.window_width as f64 / 2.0, self.window_height as f64 / 2.0);

        let mut scene = Scene::new();
        scene.add_child(sprite_mainboard);

        // let mut board = board::Board::new();

        let tex_block = Rc::new(
            Texture::from_path(
                &mut texture_context,
                assets.join("brick_blue.png"),
                Flip::None,
                &TextureSettings::new().mag(Filter::Nearest),
            )
            .unwrap(),
        );
        let mut sprite_block = Sprite::from_texture(tex_block.clone());
        sprite_block.set_scale(1.0, 1.0);
        sprite_block.set_position(width as f64 / 2.0, height as f64 / 2.0);
        let id_block = scene.add_child(sprite_block);

        let anim_move_up = Action(MoveBy(0.1, 0.0, -100.0));
        let anim_move_down = Action(MoveBy(0.1, 0.0, 100.0));
        let anim_move_left = Action(MoveBy(0.1, -100.0, 0.0));
        let anim_move_right = Action(MoveBy(0.1, 100.0, 0.0));

        // let mut cursor = [0.0, 0.0];

        // update loop
        while let Some(event) = window.next() {
            scene.event(&event);
            // board.event(&event);

            // if let Some(button) = event.release_args() {
            //     match button {
            //         Button::Keyboard(key) => println!("Released keyboard key '{:?}'", key),
            //         Button::Mouse(button) => println!("Released mouse button '{:?}'", button),
            //         Button::Controller(button) => println!("Released controller button '{:?}'", button),
            //         Button::Hat(hat) => println!("Released controller hat `{:?}`", hat),
            //     }
            // };

            if let Some(Button::Keyboard(key)) = event.press_args() {
                match key {
                    Key::Up => scene.run(id_block, &anim_move_up),
                    Key::Down => scene.run(id_block, &anim_move_down),
                    Key::Left => scene.run(id_block, &anim_move_left),
                    Key::Right => scene.run(id_block, &anim_move_right),
                    _ => (),
                }
            };

            // event.mouse_cursor(|pos| {
            //     cursor = pos;
            //     println!("Mouse moved '{} {}'", pos[0], pos[1]);
            // });

            window.draw_2d(&event, |context, graphics, device| {
                clear([0.8; 4], graphics);

                scene.draw(context.transform, graphics);
                // board.draw(context.transform, graphics);

                // rectangle(
                //     [1.0, 0.0, 0.0, 1.0], // red
                //     [200.0, 200.0, 100.0, 100.0],
                //     context.transform,
                //     graphics,
                // );

                let text_transform = context.transform.trans(100.0, 100.0);
                Text::new_color([0.0, 0.0, 1.0, 1.0], 32)
                    .draw(
                        "Hello world!",
                        &mut glyphs,
                        &context.draw_state,
                        text_transform,
                        graphics,
                    )
                    .unwrap();
                glyphs.factory.encoder.flush(device);
            });
        }
    }
}
