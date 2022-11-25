pub mod entity;
pub mod player;
mod utils;
mod world;


use entity::Entity;
use player::*;
use raylib::prelude::*;
use tracing_subscriber::FmtSubscriber;
use tracing::{info, error, Level};



fn main() {
    // let opt:  = None;
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    let mut player = Player::default();
    game(&mut [&mut player]);
    info!("game started");
}

fn game(entities: &mut [&mut impl Entity]) {
    let (mut rl, thread) = raylib::init().size(640, 480).title("Doom Cone").build();

        entities.iter_mut().for_each(|val|{
            val.ready();
        });

    while !rl.window_should_close() {
        entities.iter_mut().for_each(|val|{
            val.update(&rl.get_frame_time(), &rl);
        });

        let mut d = rl.begin_drawing(&thread);
        world::draw_map(&mut d, &world::get_map());
        d.clear_background(Color::GRAY);

        entities.iter().for_each(|val| {
            val.draw(&mut d);
        });
    }

}





