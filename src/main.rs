extern crate json;
extern crate tiled;

extern crate cgmath;
#[macro_use]
extern crate gfx;
extern crate specs;
extern crate genmesh;
extern crate gfx_device_gl;
extern crate gfx_window_glutin;
extern crate glutin;
extern crate image;
extern crate rodio;

mod bullet;
mod gfx_app;
mod game;
mod data;
mod critter;
mod graphics;
mod terrain;
mod character;
mod shaders;
mod zombie;

fn main() {
  let mut window = gfx_app::GlutinWindow::new();
  loop {
    match gfx_app::init::run(&mut window) {
      gfx_app::GameStatus::Quit => break,
    }
  }
}
