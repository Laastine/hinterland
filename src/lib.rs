extern crate json;
extern crate tiled;

extern crate cgmath;
extern crate genmesh;
#[macro_use]
extern crate gfx;
extern crate gfx_device_gl;
extern crate gfx_window_glutin;
extern crate glutin;
extern crate image;
extern crate rand;
extern crate rodio;
extern crate specs;

mod bullet;
mod gfx_app;
mod game;
mod data;
mod critter;
pub mod graphics;
mod terrain;
mod character;
mod shaders;
mod zombie;

pub fn main() {
  let mut window = gfx_app::GlutinWindow::new();
  #[cfg_attr(feature = "cargo-clippy", allow(never_loop))]
  loop {
    match gfx_app::init::run(&mut window) {
      gfx_app::GameStatus::Quit => break,
    }
  }
}
