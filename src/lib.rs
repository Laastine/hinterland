extern crate cgmath;
extern crate genmesh;
#[macro_use]
extern crate gfx;
extern crate gfx_device_gl;
extern crate gfx_window_glutin;
extern crate glutin;
extern crate image;
extern crate json;
extern crate pathfinding;
extern crate rand;
extern crate rodio;
extern crate rusttype;
extern crate specs;
extern crate tiled;

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
  let mut window = gfx_app::GlutinWindow::new();
  gfx_app::init::run(&mut window);
}
