use std::f32::consts::PI;

use color_eyre::{Report, Result};

fn raycast(start_pos: [f32; 2], angle: f32, map: &[&[i32]]) -> Result<bool, Report> {
    let aTan = angle.atan();

    let tile_pos_x = (start_pos[0] / 32.0).ceil();
    let tile_pos_y = (start_pos[1] / 32.0).ceil();
    
    // if (angle == 0 || ra == PI)

    Ok(true)
}
