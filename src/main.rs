use gfx_app::run;
use terrain::RenderSystem;

mod critter;
mod data;
mod game;
mod gfx_app;
mod graphics;
mod terrain;

pub fn main() {
  run::<RenderSystem>("Hinterland");
}
