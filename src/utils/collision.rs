use std::f32::consts::PI;

use color_eyre::{Report, Result};


fn raycast(start_pos: [i32; 2], angle: f32) -> Result<bool, Report> {
    let aTan = angle.atan();
    if angle > PI {
        
    }
    Ok(true)
}