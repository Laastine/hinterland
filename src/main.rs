use window::WindowStatus;

mod window;

pub fn main() {
  let mut window = window::Window::new();
  loop {
    if let WindowStatus::Close = window.run() {
      break;
    }
  }
}
