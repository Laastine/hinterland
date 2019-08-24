use image::imageops::contrast;
use wgpu::{CommandEncoder, Device, SwapChain, SwapChainDescriptor};
use winit::{
  self,
  dpi::LogicalSize,
  event::ElementState::{Pressed, Released},
  event::Event,
  event::VirtualKeyCode::{A, D, Escape, R, S, W, X, Z},
  event::WindowEvent::KeyboardInput,
  event_loop::EventLoop,
  window::Window,
};
use winit::event_loop::{ControlFlow, EventLoopWindowTarget};
use winit::platform::desktop::EventLoopExtDesktop;

use crate::character::controls::CharacterControl;
use crate::game::constants::{RESOLUTION_X, RESOLUTION_Y};
use crate::gfx_app::controls::{Control, TilemapControls};
use crate::graphics::orientation::Stance::NormalDeath;

pub mod controls;
pub mod init;
pub mod system;

#[derive(PartialEq, Eq)]
pub enum WindowStatus {
  Open,
  Close,
}

pub struct WindowContext {
  events_loop: EventLoop<()>,
  controls: Option<TilemapControls>,
  window: Window,
}

impl WindowContext {
  pub fn new() -> WindowContext {
    let events_loop = EventLoop::new();
    let window = winit::window::Window::new(&events_loop).unwrap();
    window.set_title("Hinterland");
    let hidpi_factor = window.hidpi_factor();
    let size = window.inner_size().to_physical(hidpi_factor);

//    let instance = wgpu::Instance::new();
//    let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
//      power_preference: wgpu::PowerPreference::LowPower,
//    });
//    let device = adapter.request_device(&wgpu::DeviceDescriptor {
//      extensions: wgpu::Extensions {
//        anisotropic_filtering: false,
//      },
//      limits: wgpu::Limits::default(),
//    });
//    window.set_inner_size(LogicalSize::new(RESOLUTION_X as f64, RESOLUTION_Y as f64));
//    window.set_title("Hinterland");

    WindowContext {
      controls: None,
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
      self.window.hidpi_factor() as f32
    }
  }

  fn get_viewport_size(&self) -> (f32, f32) {
    if cfg!(feature = "windowed") {
      (RESOLUTION_X as f32, RESOLUTION_Y as f32)
    } else {
      let monitor = self.events_loop.available_monitors().nth(0).expect("No monitor found");
      let monitor_resolution = monitor.size();
      (monitor_resolution.width as f32, monitor_resolution.height as f32)
    }
  }

  pub fn poll_events(&mut self) {
    let controls = match self.controls {
      Some(ref mut c) => c,
      None => panic!("Terrain controls have not been initialized"),
    };
    // Waits winit 0.20.0 release
//    self.events_loop.run_return(|event, _, control_flow| {
//      if let winit::event::Event::WindowEvent { event, .. } = event {
//        match event {
//          winit::event::WindowEvent::KeyboardInput { input, .. } => process_keyboard_input(input, controls),
//          _ => (),
//        }
//      }
//    });
  }
}

fn process_keyboard_input(input: winit::event::KeyboardInput, controls: &mut TilemapControls) {
  match input {
    winit::event::KeyboardInput { state: Pressed, virtual_keycode: Some(Z), .. } => {
      controls.zoom(&Control::Negative);
    }
    winit::event::KeyboardInput { state: Pressed, virtual_keycode: Some(X), .. } => {
      controls.zoom(&Control::Plus);
    }
    winit::event::KeyboardInput { state: Released, virtual_keycode: Some(Z), .. } |
    winit::event::KeyboardInput { state: Released, virtual_keycode: Some(X), .. } => {
      controls.zoom(&Control::Released);
    }
    winit::event::KeyboardInput { state: Pressed, virtual_keycode: Some(W), .. } => {
      controls.move_character(CharacterControl::Up);
    }
    winit::event::KeyboardInput { state: Pressed, virtual_keycode: Some(S), .. } => {
      controls.move_character(CharacterControl::Down);
    }
    winit::event::KeyboardInput { state: Released, virtual_keycode: Some(W), .. } |
    winit::event::KeyboardInput { state: Released, virtual_keycode: Some(S), .. } => {
      controls.move_character(CharacterControl::YMoveStop);
    }
    winit::event::KeyboardInput { state: Pressed, virtual_keycode: Some(A), .. } => {
      controls.move_character(CharacterControl::Left);
    }
    winit::event::KeyboardInput { state: Pressed, virtual_keycode: Some(D), .. } => {
      controls.move_character(CharacterControl::Right);
    }
    winit::event::KeyboardInput { state: Released, virtual_keycode: Some(A), .. } |
    winit::event::KeyboardInput { state: Released, virtual_keycode: Some(D), .. } => {
      controls.move_character(CharacterControl::XMoveStop);
    }
    winit::event::KeyboardInput { state: Pressed, virtual_keycode: Some(R), .. } => {}
    winit::event::KeyboardInput { state: Released, virtual_keycode: Some(R), .. } => {}
    winit::event::KeyboardInput { state: Pressed, modifiers, .. } => {}
    winit::event::KeyboardInput { state: Released, modifiers, .. } => {}
  }
  if let Some(Escape) = input.virtual_keycode {
    std::process::exit(0);
  }
}
