extern crate find_folder;
extern crate piston_window;

use piston_window::*;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Tetris Game", [640, 480])
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

    let mainboard = Texture::from_path(
        &mut window.create_texture_context(),
        assets.join("mainboard.png"),
        Flip::None,
        &TextureSettings::new().min(Filter::Nearest),
    )
    .unwrap();

    // update loop
    while let Some(event) = window.next() {
        // window.draw_2d(&event, draw);
        window.draw_2d(&event, |context, mut graphics, _device| {
            draw(&context, &mut graphics);

            image(&mainboard, context.transform, graphics);

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

fn draw(context: &Context, graphics: &mut G2d) {
    clear([0.8; 4], graphics);

    rectangle(
        [1.0, 0.0, 0.0, 1.0], // red
        [200.0, 200.0, 100.0, 100.0],
        context.transform,
        graphics,
    );
}
