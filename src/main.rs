pub mod entity;
pub mod player;
mod utils;
mod world;

use entity::Entity;
use player::*;
use raylib::prelude::*;
use tracing::{error, info, Level};
use tracing_subscriber::FmtSubscriber;
use rayon::prelude::*;

const MAP: world::Map = world::Map {
    tile_size: 182,
    layout: &[
        &[1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        &[1, 0, 0, 0, 1, 1, 0, 0, 0, 1],
        &[1, 0, 0, 0, 1, 1, 0, 0, 0, 1],
        &[1, 0, 0, 0, 1, 1, 0, 0, 0, 1],
        &[1, 0, 0, 1, 1, 1, 1, 0, 0, 1],
        &[1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        &[1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        &[1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        &[1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        &[1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    ],
};

fn main() {
    // let opt:  = None;
    let mut map = world::Map::default();
    let mut player = Player::default();

    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    game(&mut [&mut player]);
    info!("game started");
}

fn game(entities: &mut [&mut dyn Entity]) {
    let (mut rl, thread) = raylib::init().size(1280, 720).title("Doom Cone").build();

    entities.iter_mut().for_each(|val| {
        val.ready();
    });

    while !rl.window_should_close() {
        entities.iter_mut().for_each(|val| {
            val.update(&rl.get_frame_time(), &rl);
        });

        let mut d = rl.begin_drawing(&thread);
        info!("{}", &d.get_fps());
        world::draw_map(&mut d, &MAP);
        d.clear_background(Color::GRAY);

        entities.iter().for_each(|val| {
            val.draw(&mut d);
        });
    }
}
