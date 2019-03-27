use terrain::TerrainDrawSystem;

use crate::gfx_app::WindowContext;

mod critter;
mod data;
mod game;
mod gfx_app;
mod graphics;
mod terrain;

pub fn main() {
  let mut window = WindowContext::new();
  window.run();
}
