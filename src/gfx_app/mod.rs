use image::imageops::contrast;
use wgpu::{CommandEncoder, Device, SwapChain, SwapChainDescriptor};
use wgpu::winit::{Event, EventsLoop, Window, WindowEvent};
use winit::dpi::LogicalSize;
use winit::ElementState::{Pressed, Released};
use winit::KeyboardInput;
use winit::VirtualKeyCode::{A, D, Escape, R, S, W, X, Z};

use crate::game::constants::{RESOLUTION_X, RESOLUTION_Y};
use crate::gfx_app::controls::{Control, TilemapControls};
use crate::graphics::orientation::Stance::NormalDeath;
use crate::character::controls::CharacterControl;

pub mod controls;
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
  controls: Option<TilemapControls>,
  window: Window,
//  device: wgpu::Device,
//  instance: wgpu::Instance,
}

impl WindowContext {
  pub fn new() -> WindowContext {
    let events_loop = EventsLoop::new();
    let window = Window::new(&events_loop).unwrap();
    let controls = None;
    window.set_inner_size(LogicalSize::new(RESOLUTION_X as f64, RESOLUTION_Y as f64));
    window.set_title("Hinterland");

    WindowContext {
      controls,
      events_loop,
      window,
//      device,
//      instance,
    }
  }

//  pub fn get_device(&mut self) -> &mut wgpu::Device {
//    &mut self.device
//  }
//
//  pub fn get_instance(&self) -> &wgpu::Instance {
//    &self.instance
//  }

  pub fn get_window(&self) -> &Window {
    &self.window
  }

  fn set_controls(&mut self, controls: controls::TilemapControls) {
    self.controls = Some(controls);
  }

  pub fn get_hidpi_factor(&self) -> f32 {
    if cfg!(feature = "windowed") {
      1.0
    } else {
      self.window.get_hidpi_factor() as f32
    }
  }

  fn get_viewport_size(&self) -> (f32, f32) {
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

    let controls = match self.controls {
      Some(ref mut c) => c,
      None => panic!("Terrain controls have not been initialized"),
    };

    self.events_loop.poll_events(|event| match event {
      Event::WindowEvent { event, .. } => match event {
        WindowEvent::CloseRequested => {
          game_status = WindowStatus::Close;
        }
        _ => {
          game_status = update(event, controls);
        }
      },
      _ => (),
    });
    game_status
  }
}

fn update(window_event: wgpu::winit::WindowEvent, controls: &mut TilemapControls) -> WindowStatus {
  match window_event {
    winit::WindowEvent::KeyboardInput { input, .. } => { process_keyboard_input(input, controls) }
    _ => WindowStatus::Open
  }
}

fn process_keyboard_input(input: winit::KeyboardInput, controls: &mut TilemapControls) -> WindowStatus {
  match input {
    KeyboardInput { state: Pressed, virtual_keycode: Some(Z), .. } => {
      controls.zoom(&Control::Negative);
    }
    KeyboardInput { state: Pressed, virtual_keycode: Some(X), .. } => {
      controls.zoom(&Control::Plus);
    }
    KeyboardInput { state: Released, virtual_keycode: Some(Z), .. } |
    KeyboardInput { state: Released, virtual_keycode: Some(X), .. } => {
      controls.zoom(&Control::Released);
    }
    KeyboardInput { state: Pressed, virtual_keycode: Some(W), .. } => {
      controls.move_character(CharacterControl::Up);
    }
    KeyboardInput { state: Pressed, virtual_keycode: Some(S), .. } => {
      controls.move_character(CharacterControl::Down);
    }
    KeyboardInput { state: Released, virtual_keycode: Some(W), .. } |
    KeyboardInput { state: Released, virtual_keycode: Some(S), .. } => {
      controls.move_character(CharacterControl::YMoveStop);
    }
    KeyboardInput { state: Pressed, virtual_keycode: Some(A), .. } => {
      controls.move_character(CharacterControl::Left);
    }
    KeyboardInput { state: Pressed, virtual_keycode: Some(D), .. } => {
      controls.move_character(CharacterControl::Right);
    }
    KeyboardInput { state: Released, virtual_keycode: Some(A), .. } |
    KeyboardInput { state: Released, virtual_keycode: Some(D), .. } => {
      controls.move_character(CharacterControl::XMoveStop);
    }
    KeyboardInput { state: Pressed, virtual_keycode: Some(R), .. } => {}
    KeyboardInput { state: Released, virtual_keycode: Some(R), .. } => {}
    KeyboardInput { state: Pressed, modifiers, .. } => {}
    KeyboardInput { state: Released, modifiers, .. } => {}
  }
  if let Some(Escape) = input.virtual_keycode {
    WindowStatus::Close
  } else {
    WindowStatus::Open
  }
}
