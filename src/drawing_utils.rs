use bevy::prelude::*;

pub fn draw_rect(size: Vec2, position: Vec2, color: Color) -> SpriteBundle {
    SpriteBundle {
        sprite: Sprite {
            color,
            custom_size: Some(size),
            ..default()
        },
        transform: Transform::from_translation(position.extend(0.0)),
        ..Default::default()
    }
}
