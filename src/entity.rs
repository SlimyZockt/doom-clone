use raylib::{core::texture::Image, prelude::RaylibDrawHandle, RaylibHandle};


pub trait Entity {
    fn ready(&self);
    fn draw(&self, draw_handler: &mut RaylibDrawHandle);
    fn update(&mut self, delta: f32 , handle: &RaylibHandle);
}