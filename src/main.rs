#[macro_use]
extern crate json;
extern crate sdl2;
extern crate conv;
extern crate tiled;

extern crate cgmath;
#[macro_use]
extern crate gfx;
extern crate gfx_app;
extern crate rand;
extern crate genmesh;
extern crate noise;
extern crate image;
extern crate winit;

mod game;
mod views;
mod data;

fn main() {
  use gfx_app::Application;
  let wb = winit::WindowBuilder::new().with_title("Zombie shooter");
  views::TileMap::launch_default(wb);
}
