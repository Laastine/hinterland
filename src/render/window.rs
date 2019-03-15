use wgpu::winit::{Event, EventsLoop, Window, WindowEvent};

#[allow(dead_code)]
pub fn cast_slice<T>(data: &[T]) -> &[u8] {
  use std::mem::size_of;
  use std::slice::from_raw_parts;

  unsafe { from_raw_parts(data.as_ptr() as *const u8, data.len() * size_of::<T>()) }
}

#[derive(PartialEq, Eq)]
pub enum WindowStatus {
  Open,
  Close,
}

pub trait GameWindow {
  fn init(sc_desc: &wgpu::SwapChainDescriptor, device: &mut wgpu::Device) -> Self;
  fn resize(&mut self, sc_desc: &wgpu::SwapChainDescriptor, device: &mut wgpu::Device);
  fn update(&mut self, event: wgpu::winit::WindowEvent) -> WindowStatus;
  fn render(&mut self, frame: &wgpu::SwapChainOutput, device: &mut wgpu::Device);
}

pub fn run<W: GameWindow>(title: &str) {
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
  window.set_title(title);
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

  let mut game_window = W::init(&sc_desc, &mut device);

  let mut game_status = WindowStatus::Open;

  loop {
    events_loop.poll_events(|event| match event {
      Event::WindowEvent {
        event: WindowEvent::Resized(size),
        ..
      } => {
        let physical = size.to_physical(window.get_hidpi_factor());
        sc_desc.width = physical.width as u32;
        sc_desc.height = physical.height as u32;
        swap_chain = device.create_swap_chain(&surface, &sc_desc);
        game_window.resize(&sc_desc, &mut device);
      }
      Event::WindowEvent { event, .. } => match event {
        WindowEvent::CloseRequested => {
          game_status = WindowStatus::Close;
        }
        _ => {
          game_status = game_window.update(event);
        }
      },
      _ => (),
    });

    let frame = swap_chain.get_next_texture();
    game_window.render(&frame, &mut device);

    if let WindowStatus::Close = game_status {
      break;
    }
  }
}
