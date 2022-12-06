use std::f32::consts::PI;

use raylib::{consts::*, math::Vector2, prelude::*};
use rayon::prelude::{IntoParallelIterator, IntoParallelRefIterator};
use tracing::info;

use crate::entity::Entity;
use crate::utils::{raycast, RayResult};

#[derive(Clone, Copy, Debug, Default)]
pub enum Weapons {
    #[default]
    Pistol,
}
#[derive(Clone, Debug, Default)]
pub struct Player {
    pub current_gun: Weapons,
    pub x: f32,
    pub y: f32,
    rotation: f32,
    raycast_pos: Vec<RayResult>,
    fov: i32
}

impl Entity for Player {
    fn ready(&mut self) {
        self.rotation = 0.1;
        self.x = 182.0 * 2.0;
        self.y = 182.0 * 2.0;

        self.fov = 70;
    }

    fn update(&mut self, delta: &f32, rl: &RaylibHandle) {
        let speed = 120.0;
        let rot_speed = 20.0;

        self.rotation += {
            if rl.is_key_down(KeyboardKey::KEY_LEFT) {
                -0.1
            } else if rl.is_key_down(KeyboardKey::KEY_RIGHT) {
                0.1
            } else {
                0.
            }
        } * delta
            * rot_speed;

        if self.rotation < 0.0 {
            self.rotation += 2.0 * PI;
        } else if self.rotation > 2.0 * PI {
            self.rotation -= 2.0 * PI;
        }

        let mut axis: Vector2 = Vector2::zero();
        if rl.is_key_down(KeyboardKey::KEY_W) {
            axis.x += self.rotation.cos();
            axis.y += self.rotation.sin();
        } else if rl.is_key_down(KeyboardKey::KEY_S) {
            axis.x -= self.rotation.cos();
            axis.y -= self.rotation.sin()
        }

        // if rl.is_key_down(KeyboardKey::KEY_D) {
        //     axis.x += 2.0;
        //     axis.y += 2.0;
        // } else if rl.is_key_down(KeyboardKey::KEY_A) {
        //     axis.x += -2.0;
        //     axis.y += -2.0;
        // }

        self.x += axis.x * 100.0 * delta;
        self.y += axis.y * 100.0 * delta;


        self.raycast_pos = (-self.fov/2..self.fov/2)
            .map(|i| {
                raycast(
                    &Vector2::new(self.x + 2.0, self.y + 2.0),
                    self.rotation + (i as f32).to_radians(),
                    crate::MAP,
                )
            })
            .collect();
    }

    fn draw(&self, d: &mut RaylibDrawHandle) {
        let td_mode = false;

        // 3D rendering
        self.raycast_pos.iter().enumerate().for_each(|(i, ray)| {
            let mut ca = self.rotation - ray.angle;
            if ca < 0.0 {
                ca += 2.0 * PI;
            }
            if ca > 2.0 * PI {
                ca -= 2.0 * PI;
            }
            let dist = ray.length * ca.cos();

            let line_hight = ((d.get_screen_width() * crate::MAP.tile_size) as f32 / dist)
                .min(d.get_screen_width() as f32);
            let line_off = (d.get_screen_height() / 2) as f32 - (line_hight / 2.0);
            let x_offset = d.get_screen_width() as f32 / self.raycast_pos.len() as f32;

            d.draw_line_ex(
                Vector2::new((i as f32 + 0.5) * x_offset, line_off),
                Vector2::new((i as f32 + 0.5) * x_offset, line_hight + line_off),
                x_offset,
                Color::new(255 - (0.2 * dist) as u8, 0, 0, 255),
            );
        });

        // 2D redering
        if td_mode {
            self.raycast_pos.iter().for_each(|ray| {
                d.draw_line(
                    (self.x + 2.0) as i32,
                    (self.y + 2.0) as i32,
                    (ray.start_pos.x + (ray.dir.x * ray.length)) as i32,
                    (ray.start_pos.y + (ray.dir.y * ray.length)) as i32,
                    Color::RED,
                );
            });

            d.draw_rectangle((self.x) as i32, (self.y) as i32, 4, 4, Color::YELLOW);

            let end_pos_x = (self.x + 2. + 10.0 * self.rotation.cos()) as i32;
            let end_pos_y = (self.y + 2. + 10.0 * self.rotation.sin()) as i32;

            d.draw_line(
                (self.x + 2.) as i32,
                (self.y + 2.) as i32,
                end_pos_x,
                end_pos_y,
                Color::YELLOW,
            );
        }
    }
}
