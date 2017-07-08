extern crate json;
extern crate conv;
extern crate tiled;

extern crate cgmath;
#[macro_use]
extern crate gfx;
extern crate rand;
extern crate specs;
extern crate genmesh;
extern crate gfx_device_gl;
extern crate gfx_window_glutin;
extern crate glutin;
extern crate noise;
extern crate image;
extern crate winit;

mod gfx_app;
mod game;
mod data;
mod physics;
mod terrain;

fn main() {
  let mut window = gfx_app::GlutinWindow::new();
  loop {
    match gfx_app::init::run(&mut window) {
      Quit => {
        println!("Game was quit");
        break;
      }
      Render => println!("Render..."),
    }
  }
}
