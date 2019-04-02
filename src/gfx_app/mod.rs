use wgpu::{Device, SwapChain, SwapChainDescriptor, CommandEncoder};
use wgpu::winit::{Event, EventsLoop, Window, WindowEvent};
use winit::dpi::LogicalSize;

use crate::game::constants::{RESOLUTION_X, RESOLUTION_Y};

pub mod init;
//pub mod renderer;
pub mod system;

#[derive(PartialEq, Eq)]
pub enum WindowStatus {
  Open,
  Close,
}

pub struct WindowContext {
  events_loop: EventsLoop,
  window: Window,
}

impl WindowContext {
  pub fn new() -> WindowContext {
    let events_loop = EventsLoop::new();
    let window = Window::new(&events_loop).unwrap();

    window.set_inner_size(LogicalSize::new(RESOLUTION_X as f64, RESOLUTION_Y as f64));
    window.set_title("Hinterland");

    WindowContext {
      events_loop,
      window,
    }
  }

  pub fn get_window(&self) -> & Window {
    &self.window
  }

  pub fn get_hidpi_factor(&self) -> f32 {
    if cfg!(feature = "windowed") {
      1.0
    } else {
      self.window.get_hidpi_factor() as f32
    }
  }

  fn get_viewport_size(&mut self) -> (f32, f32) {
    if cfg!(feature = "windowed") {
      (RESOLUTION_X as f32, RESOLUTION_Y as f32)
    } else {
      let monitor = self.events_loop.get_available_monitors().nth(0).expect("No monitor found");
      let monitor_resolution = monitor.get_dimensions();
      (monitor_resolution.width as f32, monitor_resolution.height as f32)
    }
  }

  pub fn poll_events(&mut self) -> WindowStatus {
    let mut game_status = WindowStatus::Open;

    self.events_loop.poll_events(|event| match event {
      Event::WindowEvent { event, .. } => match event {
        WindowEvent::CloseRequested => {
          game_status = WindowStatus::Close;
        }
        _ => {
          game_status = update(event);
        }
      },
      _ => (),
    });
    game_status
  }
}

fn update(window_event: wgpu::winit::WindowEvent) -> WindowStatus {
  match window_event {
    winit::WindowEvent::KeyboardInput { input, .. } => { process_keyboard_input(input) }
    _ => WindowStatus::Open
  }
}

fn process_keyboard_input(input: winit::KeyboardInput) -> WindowStatus {
  if let Some(winit::VirtualKeyCode::Escape) = input.virtual_keycode {
    WindowStatus::Close
  } else {
    WindowStatus::Open
  }
}
