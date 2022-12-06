use std::{default, f32::consts::PI};

use color_eyre::{Report, Result};
use raylib::math::Vector2;
use tracing::info;

#[derive(Debug, Default, Clone, Copy)]
pub struct RayResult {
    pub start_pos: Vector2,
    pub dir: Vector2,
    pub length: f32,
    pub angle: f32,
    pub is_horizontal: bool
}

impl RayResult {
    pub fn new(start_pos: Vector2, dir: Vector2, length: f32, angle: f32, is_horizontal: bool) -> Self {
        Self {
            start_pos,
            dir,
            length,
            angle,
            is_horizontal
        }
    }
}

pub fn raycast(
    start_pos: &Vector2,
    angle: f32,
    map: crate::world::Map,
) -> RayResult {
    let pos = Vector2::new(start_pos.x, start_pos.y);

    let dir_y: f32 = (angle).sin();
    let dir_x: f32 = (angle).cos();

    let mut tile_pos_x = (pos.x / map.tile_size as f32).floor();
    let mut tile_pos_y = (pos.y / map.tile_size as f32).floor();

    // if (angle == 0 || ra == PI)

    let dir_sings_x = if dir_x > 0.0 { 1 } else if dir_x == 0.0 {0} else { -1 };
    let dir_sings_y = if dir_y > 0.0 { 1 } else if dir_x == 0.0 {0} else { -1 };
    let tile_offset_x = i32::from(dir_x > 0.0);
    let tile_offset_y = i32::from(dir_y > 0.0);

    let mut dt_x = ((tile_pos_x + tile_offset_x as f32) * map.tile_size as f32 - pos.x) / dir_x;
    let mut dt_y = ((tile_pos_y + tile_offset_y as f32) * map.tile_size as f32 - pos.y) / dir_y;


    let mut length = 0.0;

    let mut found = false;
    
    while !found{
        if dt_x < dt_y {
            tile_pos_x += dir_sings_x as f32;
            let dt = dt_x;
            length += dt;
            dt_x = dt_x + dir_sings_x as f32 * map.tile_size as f32 / dir_x - dt;
            dt_y -= dt;
        } else {
            tile_pos_y += dir_sings_y as f32;
            let dt = dt_y;
            length += dt;
            dt_x -= dt;
            dt_y = dt_y + dir_sings_y as f32 * map.tile_size as f32 / dir_y - dt;
        }
        if tile_pos_x >= 0.0
            && tile_pos_x < map.layout.len() as f32
            && tile_pos_y >= 0.0
            && tile_pos_y < map.layout[0].len() as f32
            && map.layout[tile_pos_y as usize][tile_pos_x as usize] > 0
        {
            found = true;
        }
    }


    RayResult::new(pos, Vector2::new(dir_x, dir_y), length, angle, false)
}
