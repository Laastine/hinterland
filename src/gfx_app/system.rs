use crate::terrain::{TerrainDrawSystem, TerrainDrawable};
use wgpu::{Device, SwapChainDescriptor, CommandEncoder, SwapChain};
use specs::{WriteStorage, Read};
use crate::graphics::DeltaTime;

pub struct DrawSystem {
  terrain_system: TerrainDrawSystem,
  swap_chain: SwapChain,
  device: Device,
}

impl DrawSystem {
  pub fn new(window: &winit::Window) -> DrawSystem {

    let instance = wgpu::Instance::new();
    let adapter = instance.get_adapter(&wgpu::AdapterDescriptor {
      power_preference: wgpu::PowerPreference::LowPower,
    });
    let mut device = adapter.create_device(&wgpu::DeviceDescriptor {
      extensions: wgpu::Extensions {
        anisotropic_filtering: false,
      },
    });
    let size = window
      .get_inner_size()
      .unwrap()
      .to_physical(window.get_hidpi_factor());

    let surface = instance.create_surface(&window);
    let sc_desc = wgpu::SwapChainDescriptor {
      usage: wgpu::TextureUsageFlags::OUTPUT_ATTACHMENT,
      format: wgpu::TextureFormat::Bgra8Unorm,
      width: size.width as u32,
      height: size.height as u32,
    };
    let swap_chain = device.create_swap_chain(&surface, &sc_desc);

    let terrain_system = TerrainDrawSystem::new(&sc_desc, &mut device);
    DrawSystem {
      terrain_system,
      swap_chain,
      device,
    }
  }
}

impl<'a> specs::prelude::System<'a> for DrawSystem {
  type SystemData = (WriteStorage<'a, TerrainDrawable>,
                    Read<'a, DeltaTime>);

  fn run(&mut self, (mut terrain, dt): Self::SystemData) {
    use specs::join::Join;

    let delta = dt.0;
    println!("delta {}", delta);
    for t in (&mut terrain).join() {
      self.terrain_system.draw(t, &self.swap_chain.get_next_texture(), &mut self.device);
    }
  }
}
