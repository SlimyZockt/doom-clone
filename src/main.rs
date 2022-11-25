pub mod entity;
pub mod player;
mod utils;


use entity::Entity;
use player::*;
use raylib::prelude::Color as RColor;
use raylib::prelude::*;
use tracing_subscriber::FmtSubscriber;
use tracing::{info, error, Level};

pub const MAP: &[&[i32; 6]; 6] = &[
    &[1, 1, 1, 1, 1, 1],
    &[1, 0, 0, 0, 0, 1],
    &[1, 0, 1, 1, 0, 1],
    &[1, 0, 0, 0, 0, 1],
    &[1, 0, 0, 0, 0, 1],
    &[1, 1, 1, 1, 1, 1],
];

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
        draw_map::<6>(&mut d, MAP);
        d.clear_background(RColor::GRAY);

        entities.iter().for_each(|val| {
            val.draw(&mut d);
        });
    }

}


fn draw_map<const T: usize>(draw_handler: &mut RaylibDrawHandle, map: &[&'static [i32; T]; T]) {
    map.iter().enumerate().for_each(|(y, row)| {
        row.iter().enumerate().for_each(|(x, key)| {
            if key == &1 {
                let offset = [x as i32 * 32, y as i32 * 32];
                draw_handler.draw_rectangle(offset[0], offset[1], 32, 32, RColor::WHITE);
            }
        });
    });
}


