extern crate find_folder;
extern crate piston_window;
extern crate sprite;

use ai_behavior::Action;
use piston_window::*;
use sprite::*;
use std::rc::Rc;

mod assets;
mod config;

pub struct Game {
    config: config::Config,
    window: PistonWindow,
    assets: assets::Assets,
}

impl Game {
    pub fn new() -> Game {
        let _config = config::Config::new();

        let mut _window = WindowSettings::new(
            _config.window_title.clone(),
            [_config.window_width, _config.window_height],
        )
        .exit_on_esc(true)
        .samples(4)
        .build()
        .unwrap();

        let _assets = assets::Assets::new(&mut _window);

        Game {
            config: _config,
            window: _window,
            assets: _assets,
        }
    }

    pub fn run(&mut self) {
        let assets = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("assets")
            .unwrap();

        let tex_mainboard = Rc::new(self.assets.mainboard.clone());
        let mut sprite_mainboard = Sprite::from_texture(tex_mainboard);
        sprite_mainboard.set_scale(0.5, 0.5);
        sprite_mainboard.set_position(
            self.config.window_width as f64 / 2.0,
            self.config.window_height as f64 / 2.0,
        );

        let mut scene = Scene::new();
        scene.add_child(sprite_mainboard);

        // let mut board = board::Board::new();

        let tex_block = Rc::new(self.assets.brick_blue.clone());
        let mut sprite_block = Sprite::from_texture(tex_block);
        sprite_block.set_scale(1.0, 1.0);
        sprite_block.set_position(
            self.config.window_width as f64 / 2.0,
            self.config.window_height as f64 / 2.0,
        );
        let id_block = scene.add_child(sprite_block);

        let anim_move_up = Action(MoveBy(0.1, 0.0, -100.0));
        let anim_move_down = Action(MoveBy(0.1, 0.0, 100.0));
        let anim_move_left = Action(MoveBy(0.1, -100.0, 0.0));
        let anim_move_right = Action(MoveBy(0.1, 100.0, 0.0));

        // let mut cursor = [0.0, 0.0];

        // update loop
        while let Some(event) = self.window.next() {
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

            self.window.draw_2d(&event, |context, graphics, device| {
                clear([0.8; 4], graphics);

                scene.draw(context.transform, graphics);
                // board.draw(context.transform, graphics);

                rectangle(
                    [1.0, 0.0, 0.0, 1.0], // red
                    [200.0, 200.0, 100.0, 100.0],
                    context.transform,
                    graphics,
                );

                // let text_transform = context.transform.trans(100.0, 100.0);
                // Text::new_color([0.0, 0.0, 1.0, 1.0], 32)
                //     .draw(
                //         "Hello world!",
                //         &mut self.assets.font,
                //         &context.draw_state,
                //         text_transform,
                //         graphics,
                //     )
                //     .unwrap();
                // let glyphs = &self.assets.font;
                // glyphs.factory.encoder.flush(device);
            });
        }
    }
}
