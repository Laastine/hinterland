use crate::gfx_app::WindowContext;

mod character;
mod critter;
mod data;
mod game;
mod gfx_app;
mod graphics;
mod terrain;

pub fn main() {
  let mut window = WindowContext::new();
  gfx_app::init::run(&mut window);
}
