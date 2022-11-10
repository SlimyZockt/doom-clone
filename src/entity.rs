use raylib::{prelude::RaylibDrawHandle, RaylibHandle};


pub trait Entity {
    fn ready(&mut self);
    fn draw(&self, draw_handler: &mut RaylibDrawHandle);
    fn update(&mut self, delta: f32 , handle: &RaylibHandle);
}