extern crate find_folder;
extern crate piston_window;

use piston_window::*;
use GenericEvent;

mod assets;
mod config;
mod sprite;
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
        self.config.set_brick_size(
            self.assets.brick_blue.get_width() as f64,
            self.assets.brick_blue.get_height() as f64,
        );

        let mut sprite_block = sprite::Sprite::new(self.assets.brick_blue.clone());

        // game loop
        while let Some(event) = self.window.next() {
            self.update(&event, &mut sprite_block);
            self.draw(&event, &sprite_block);
        }
    }

    fn update<E>(&mut self, event: &E, sprite_block: &mut sprite::Sprite)
    where
        E: GenericEvent,
    {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            match key {
                // Key::Up => scene.run(id_block, &anim_move_up),
                // Key::Down => scene.run(id_block, &anim_move_down),
                // Key::Left => scene.run(id_block, &anim_move_left),
                Key::Right => sprite_block.position = [self.config.brick_size.width, 0.0],
                _ => (),
            }
        };

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

    fn draw<E>(&mut self, event: &E, sprite_block: &sprite::Sprite)
    where
        E: GenericEvent,
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

            sprite_block.draw(context.transform, graphics);
        });
    }
}
