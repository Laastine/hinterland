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
mod zombie;

pub fn main() {
  let mut window = gfx_app::WindowContext::new();
  gfx_app::init::run(&mut window);
}
