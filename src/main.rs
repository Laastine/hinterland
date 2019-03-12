#[macro_use]
extern crate gfx;

mod window;

pub fn main() {
  let mut window = window::Window::new();
  window.run();
}
