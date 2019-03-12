use winit;
use winit::{ControlFlow, KeyboardInput, VirtualKeyCode};

#[derive(PartialEq, Eq)]
pub enum WindowStatus {
  Open,
  Close,
}

pub struct Window {
  events_loop: winit::EventsLoop,
  window: winit::Window,
}

impl Window {
  pub fn new() -> Window {
    let mut events_loop = winit::EventsLoop::new();
    let window = winit::Window::new(&events_loop).expect("Window creation failed");
    Window {
      events_loop,
      window,
    }
  }

  pub fn run(&mut self) -> WindowStatus {
    let mut game_status = WindowStatus::Open;

    self.events_loop.poll_events(|event| {
      game_status = if let winit::Event::WindowEvent { event, .. } = event {
        match event {
          winit::WindowEvent::KeyboardInput { input, .. } => { process_keyboard_input(input) }
          winit::WindowEvent::CloseRequested => { WindowStatus::Close },
          _ => WindowStatus::Open
        }
      } else {
        WindowStatus::Open
      }
    });
    game_status
  }
}

fn process_keyboard_input(input: KeyboardInput) -> WindowStatus {
  if let Some(VirtualKeyCode::Escape) = input.virtual_keycode {
    WindowStatus::Close
  } else {
    WindowStatus::Open
  }
}
