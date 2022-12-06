use raylib::prelude::*;

pub struct Map {
    pub tile_size: i32,
    pub layout: &'static [&'static [i32]] ,
}

impl Map {
    pub fn new(tile_size: i32, layout: &'static [&'static [i32]]) -> Self {
        Self { tile_size, layout }
    }
}

impl Default for Map {
    fn default() -> Self {
        Self {
            tile_size: 64,
            layout: &[
                &[1, 1, 1, 1, 1, 1],
                &[1, 0, 0, 0, 0, 1],
                &[1, 0, 1, 1, 0, 1],
                &[1, 0, 0, 0, 0, 1],
                &[1, 0, 0, 0, 0, 1],
                &[1, 1, 1, 1, 1, 1],
            ],
        }
    }
}

pub fn draw_map(draw_handler: &mut RaylibDrawHandle, map: &Map) {
    map.layout.iter().enumerate().for_each(|(y, row)| {
        row.iter().enumerate().for_each(|(x, key)| {
            if key == &1 {
                let offset = [x as i32 * map.tile_size, y as i32 * map.tile_size];
                draw_handler.draw_rectangle(offset[0], offset[1], map.tile_size, map.tile_size, Color::WHITE);
            }
        });
    });
}
