use crate::Game;
use raylib::consts::*;
use raylib::prelude::*;

use crate::entity::{Entity};

#[derive(Clone, Copy, Debug, Default)]
pub enum Weapons {
    #[default]
    Pistol,
}

#[derive(Clone, Debug, Copy)]
pub struct Player {
    pub current_gun: Weapons,
    pub x: f32,
    pub y: f32,
    rotation: f32,
}

impl Default for Player {
    fn default() -> Self {
        Player {
            ..Default::default()
        }
    }
}

impl Entity for Player {
    fn ready(&self) {}

    fn update(&mut self, delta: f32,  rl: &RaylibHandle) {
        let speed = 64.0;
        let rot_speed = 1.;

        let axis_x = {
            if rl.is_key_down(KeyboardKey::KEY_A) {
                -1.0
            } else if rl.is_key_down(KeyboardKey::KEY_D) {
                1.
            } else {
                0.
            }
        };
        let axis_y = {
            if rl.is_key_down(KeyboardKey::KEY_W) {
                -1.
            } else if rl.is_key_down(KeyboardKey::KEY_S) {
                1.
            } else {
                0.
            }
        };
        self.rotation += {
            if rl.is_key_down(KeyboardKey::KEY_LEFT) {
                -0.1
            } else if rl.is_key_down(KeyboardKey::KEY_RIGHT) {
                0.1
            } else {
                0.
            }
        } * delta * rot_speed;


        self.x += axis_x*speed*delta;
        self.y += axis_y*speed*delta;

    }

    fn draw(&self, d: &mut RaylibDrawHandle) {

        d.draw_rectangle((self.x) as i32, (self.y) as i32, 16, 16, Color::YELLOW);

        //TODO: Calc pos from angle
        let end_pos_x = 2;
        let end_pos_y = 2;

        d.draw_line((self.x) as i32, (self.y) as i32, end_pos_x, end_pos_y, Color::YELLOW);
    }
}
