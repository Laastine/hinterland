use wgpu::winit::{Event, EventsLoop, Window, WindowEvent};
use winit::dpi::LogicalSize;

use crate::terrain::{TerrainDrawSystem, TerrainDrawable};
use crate::graphics::dimensions::{get_view_matrix, get_projection};
use crate::game::constants::VIEW_DISTANCE;

#[derive(PartialEq, Eq)]
pub enum WindowStatus {
  Open,
  Close,
}

pub fn run() {
  let instance = wgpu::Instance::new();
  let adapter = instance.get_adapter(&wgpu::AdapterDescriptor {
    power_preference: wgpu::PowerPreference::LowPower,
  });
  let mut device = adapter.create_device(&wgpu::DeviceDescriptor {
    extensions: wgpu::Extensions {
      anisotropic_filtering: false,
    },
  });

  let mut events_loop = EventsLoop::new();
  let window = Window::new(&events_loop).unwrap();
  window.set_inner_size(LogicalSize::new(1920.0, 1080.0));
  window.set_title("Hinterland");
  let size = window
    .get_inner_size()
    .unwrap()
    .to_physical(window.get_hidpi_factor());

  let surface = instance.create_surface(&window);
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
    events_loop.poll_events(|event| match event {
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
