extern crate find_folder;
extern crate piston_window;
extern crate sprite;

use ai_behavior::Action;
use piston_window::*;
use sprite::*;
use std::rc::Rc;
use GenericEvent;

mod assets;
mod config;
mod structs;

pub struct Game {
    config: config::Config,
    window: PistonWindow,
    assets: assets::Assets,
}

impl Game {
    pub fn new() -> Game {
        let mut _config = config::Config::new();
        let mut _window =
            WindowSettings::new(_config.window_title.clone(), _config.window_size.to_array())
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
        let mut sprite_mainboard = Sprite::from_texture(Rc::new(self.assets.mainboard.clone()));
        sprite_mainboard.set_scale(0.5, 0.5);
        sprite_mainboard.set_position(
            self.config.window_size.width / 2.0,
            self.config.window_size.height / 2.0,
        );

        let mut scene = Scene::new();
        scene.add_child(sprite_mainboard);

        self.config.set_brick_size(
            self.assets.brick_blue.get_width() as f64,
            self.assets.brick_blue.get_height() as f64,
        );

        let mut sprite_block = Sprite::from_texture(Rc::new(self.assets.brick_blue.clone()));
        sprite_block.set_position(
            self.config.brick_size.width / 2.0,
            self.config.brick_size.height / 2.0,
        );
        let id_block = scene.add_child(sprite_block);

        let anim_move_up = Action(MoveBy(0.1, 0.0, -self.config.brick_size.height));
        let anim_move_down = Action(MoveBy(0.1, 0.0, self.config.brick_size.height));
        let anim_move_left = Action(MoveBy(0.1, -self.config.brick_size.width, 0.0));
        let anim_move_right = Action(MoveBy(0.1, self.config.brick_size.width, 0.0));

        // update loop
        while let Some(event) = self.window.next() {
            self.update(&event);
            scene.event(&event);

            if let Some(Button::Keyboard(key)) = event.press_args() {
                match key {
                    Key::Up => scene.run(id_block, &anim_move_up),
                    Key::Down => scene.run(id_block, &anim_move_down),
                    Key::Left => scene.run(id_block, &anim_move_left),
                    Key::Right => scene.run(id_block, &anim_move_right),
                    _ => (),
                }
            };

            self.draw(&event, &scene);
            self.window.draw_2d(&event, |context, graphics, _device| {
                scene.draw(context.transform, graphics);
            });
        }
    }

    fn update<E>(&mut self, event: &E)
    where
        E: GenericEvent,
    {
        // if let Some(button) = event.release_args() {
        //     match button {
        //         Button::Keyboard(key) => println!("Released keyboard key '{:?}'", key),
        //         Button::Mouse(button) => println!("Released mouse button '{:?}'", button),
        //         Button::Controller(button) => println!("Released controller button '{:?}'", button),
        //         Button::Hat(hat) => println!("Released controller hat `{:?}`", hat),
        //     }
        // };

        // event.mouse_cursor(|pos| {
        //     println!("Mouse moved '{} {}'", pos[0], pos[1]);
        // });
    }

    fn draw<E, I>(&mut self, event: &E, scene: &Scene<I>)
    where
        E: GenericEvent,
        I: ImageSize,
    {
        let area_width = self.config.bricks_horizontal as f64 * self.config.brick_size.width;
        let area_height = self.config.bricks_vertical as f64 * self.config.brick_size.height;

        self.window.draw_2d(event, |context, graphics, _device| {
            clear([0.8; 4], graphics);

            rectangle(
                [0.5, 0.9, 0.5, 1.0], // color
                [0.0, 0.0, area_width, area_height],
                context.transform,
                graphics,
            );

            // scene.draw(context.transform, graphics);
        });
    }
}
