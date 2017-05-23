extern crate json;
extern crate conv;
extern crate tiled;

extern crate cgmath;
#[macro_use]
extern crate gfx;
extern crate rand;
extern crate genmesh;
extern crate gfx_device_gl;
extern crate gfx_window_glutin;
extern crate glutin;
extern crate noise;
extern crate image;
extern crate winit;

mod gfx_app;
mod game;
mod tilemap;
mod data;

fn main() {
  use gfx_app::Application;
  let wb = winit::WindowBuilder::new().with_dimensions(1280, 720).with_title("Zombie shooter");
  tilemap::TileMap::launch_default(wb);
}
