use render::RenderSystem;

mod critter;
mod data;
mod game;
mod render;

pub fn main() {
  render::window::run::<RenderSystem>("Hinterland");
}
