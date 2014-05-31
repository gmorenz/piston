
#![feature(globs)]

extern crate graphics;
extern crate piston;

use graphics::*;
use piston::{
    AssetStore,
    GameIterator,
    GameIteratorSettings,
    GameWindow,
    GameWindowSDL2,
    GameWindowSettings,
    Gl,
    Render,
};

#[start]
fn start(argc: int, argv: **u8) -> int {
    // Run gui on the main thread.
    native::start(argc, argv, main)
}

fn main() {
    let mut window: GameWindowSDL2 = GameWindow::new(
        GameWindowSettings {
            title: "Image".to_string(),
            size: [300, 300],
            fullscreen: false,
            exit_on_esc: true,
            background_color: [1.0, 1.0, 1.0, 1.0],
        }
    );

    let mut asset_store = AssetStore::from_folder("assets");

    let image = asset_store.load_image("rust-logo.png").unwrap();
    let game_iter_settings = GameIteratorSettings {
            updates_per_second: 120,
            max_frames_per_second: 60,
        };
    let mut game_iter = GameIterator::new(&mut window, &game_iter_settings);
    loop {
        match game_iter.next() {
            None => { break },
            Some(e) => match e {
                Render(args) => {
                    let c = Context::abs(args.width as f64, args.height as f64);
                    let mut gl = Gl::new(args.gl_data);
                    c.image(image).draw(&mut gl);
                },
                _ => {},       
            },
        }
    }
}


