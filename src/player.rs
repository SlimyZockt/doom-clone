use bevy::ecs::system::Command;
use bevy::sprite::SpriteBatch;
use bevy::{log::info, prelude::*};

use crate::drawing_utils::draw_rect;
use crate::Game;

#[derive(Clone, Copy)]
pub enum Weapons {
    Gun,
}

impl Default for Weapons {
    fn default() -> Self {
        Self::Gun
    }
}
#[derive(Default, Clone)]
pub struct Player {
    pub entity: Option<Entity>,
    pub current_weapon: Weapons,
    pub x: f32,
    pub y: f32,
    pub rotation: f32,
    pub sprite: Sprite,
}

impl Player {
    pub fn new(self: &mut Player, commands: &mut Commands) -> Player {
        let look_pos = commands
            .spawn()
            .insert_bundle(
                draw_rect(
                    Vec2::new(2.0, 16.0),
                    Vec2::new(0.0, 8.0),
                    Color::YELLOW,
                )
            ).id();

        self.entity = Some(
            commands
                .spawn_bundle(
                    draw_rect(
                    Vec2::new(8.0, 8.0),
                    Vec2::new(0.0, 0.0),
                    Color::YELLOW,
                )
            )
            .add_child(look_pos)
            .id(),
        );
        self.clone()
    }

    pub fn movement(
        mut commands: Commands,
        keyboard_input: Res<Input<KeyCode>>,
        mut game: ResMut<Game>,
        mut transforms: Query<&mut Transform>,
    ) {
        let speed = 4.0;

        let mut direction_y = 0;
        let mut direction_x = 0;

        if keyboard_input.pressed(KeyCode::A) {
            direction_x -= 1;
        }
        else if keyboard_input.pressed(KeyCode::D) {
            direction_x += 1;
        }
        if keyboard_input.pressed(KeyCode::W) {
            direction_y += 1;
        }
        else if keyboard_input.pressed(KeyCode::S) {
            direction_y -= 1;
        }

        game.player.x += direction_x as f32 * speed;
        game.player.y += direction_y as f32 * speed;

        *transforms.get_mut(game.player.entity.unwrap()).unwrap() = Transform {
            translation: Vec3::new(game.player.x as f32, game.player.y as f32, 0.0),
            ..default()
        }
    }
}
