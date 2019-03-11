#[macro_use]
extern crate gfx;

mod audio;
mod bullet;
mod gfx_app;
mod game;
mod data;
mod critter;
pub mod graphics;
mod hud;
mod terrain_object;
mod terrain;
mod character;
mod shaders;
mod window;
mod zombie;

pub fn main() {
  let mut window = window::Window::new();
  window.run();
}
