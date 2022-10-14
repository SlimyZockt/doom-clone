pub mod drawing_utils;
pub mod player;

use bevy::{
    log::info,
    prelude::*,
    time::FixedTimestep,
    window::{CreateWindow, WindowId},
};

use color_eyre::Result;

use drawing_utils::draw_rect;
use player::*;



fn main() -> Result<()> {
    App::new()
        .init_resource::<Game>()
        .add_state(GameState::Playing)
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system_set(SystemSet::on_update(GameState::Playing).with_system(Player::movement))
        .add_system(bevy::window::close_on_esc)
        .run();

    Ok(())
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    Playing,
    GameOver,
}

#[derive(Default)]
pub struct Game {
    player: Player,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut create_window_events: EventWriter<CreateWindow>,
    mut game: ResMut<Game>
) {
    commands.spawn_bundle(Camera2dBundle::default());

    let window_id = WindowId::new();

    // sends out a "CreateWindow" event, which will be received by the windowing backend
    let map: &[&[i32]] = &[
        &[1, 1, 1, 1, 1, 1],
        &[1, 0, 0, 0, 0, 1],
        &[1, 0, 1, 1, 0, 1],
        &[1, 0, 0, 0, 0, 1],
        &[1, 0, 0, 0, 0, 1],
        &[1, 1, 1, 1, 1, 1],
    ];

    let draw_map = |map: &[&[i32]], commands: &mut Commands| -> Result<()> {
        map.iter().enumerate().for_each(|(y, row)| {
            row.iter().enumerate().for_each(|(x, key)| {
                if key == &1 {
                    let offset = [x as f32 * 32.0, y as f32 * 32.0];
                    commands.spawn_bundle(draw_rect(
                        Vec2::new(32.0, 32.0),
                        Vec2::from_array(offset),
                        Color::WHITE,
                    ));
                }
            })
        });

        Ok(())
    };

    let _ = draw_map(map, &mut commands);

    game.player.new(&mut commands);
}
