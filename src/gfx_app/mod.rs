use wgpu::winit::{Event, EventsLoop, Window, WindowEvent};
use winit::dpi::LogicalSize;

use crate::terrain::{TerrainDrawSystem, TerrainDrawable};
use crate::graphics::dimensions::{get_view_matrix, get_projection};
use crate::game::constants::{VIEW_DISTANCE, RESOLUTION_Y, RESOLUTION_X};

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
    let mut events_loop = EventsLoop::new();
    let window = Window::new(&events_loop).unwrap();
    window.set_inner_size(LogicalSize::new(RESOLUTION_X as f64, RESOLUTION_Y as f64));
    window.set_title("Hinterland");

    WindowContext {
      events_loop,
      window,
    }
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

  pub fn run(&mut self) {
    let instance = wgpu::Instance::new();
    let adapter = instance.get_adapter(&wgpu::AdapterDescriptor {
      power_preference: wgpu::PowerPreference::LowPower,
    });
    let mut device = adapter.create_device(&wgpu::DeviceDescriptor {
      extensions: wgpu::Extensions {
        anisotropic_filtering: false,
      },
    });
    let size = self.window
      .get_inner_size()
      .unwrap()
      .to_physical(self.window.get_hidpi_factor());

    let surface = instance.create_surface(&self.window);
    let mut sc_desc = wgpu::SwapChainDescriptor {
      usage: wgpu::TextureUsageFlags::OUTPUT_ATTACHMENT,
      format: wgpu::TextureFormat::Bgra8Unorm,
      width: size.width as u32,
      height: size.height as u32,
    };
    let mut swap_chain = device.create_swap_chain(&surface, &sc_desc);

    let mut terrain_draw_system = TerrainDrawSystem::new(&sc_desc, &mut device);
    let mut terrain_drawable = TerrainDrawable::new();

    let mut game_status = WindowStatus::Open;

    loop {
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

      let frame = swap_chain.get_next_texture();
      terrain_draw_system.render(&mut terrain_drawable, &frame, &mut device);

      if let WindowStatus::Close = game_status {
        break;
      }
    }
  }
}

fn update(window_event: wgpu::winit::WindowEvent) -> WindowStatus {
  match window_event {
    winit::WindowEvent::KeyboardInput { input, .. } => { process_keyboard_input(input) },
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
