use raylib::prelude::*;

pub fn get_map() -> Vec<Vec<i32>>{
    vec![
        vec![1, 1, 1, 1, 1, 1],
        vec![1, 0, 0, 0, 0, 1],
        vec![1, 0, 1, 1, 0, 1],
        vec![1, 0, 0, 0, 0, 1],
        vec![1, 0, 0, 0, 0, 1],
        vec![1, 1, 1, 1, 1, 1],
    ]
}

pub fn draw_map(draw_handler: &mut RaylibDrawHandle, map: &Vec<Vec<i32>>) {
    map.iter().enumerate().for_each(|(y, row)| {
        row.iter().enumerate().for_each(|(x, key)| {
            if key == &1 {
                let offset = [x as i32 * 32, y as i32 * 32];
                draw_handler.draw_rectangle(offset[0], offset[1], 32, 32, Color::WHITE);
            }
        });
    });
}
