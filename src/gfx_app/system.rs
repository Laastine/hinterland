use std::{mem, thread, time};
use std::time::{Duration, Instant};

use crossbeam_channel as channel;
use specs::{Read, WriteStorage};
use wgpu::{CommandBuffer, CommandEncoder, Device, SwapChain, SwapChainDescriptor};

use crate::character::{CharacterDrawable, CharacterDrawSystem};
use crate::critter::CharacterSprite;
use crate::game::constants::{CHARACTER_SHEET_TOTAL_WIDTH, RUN_SPRITE_OFFSET, SPRITE_OFFSET};
use crate::gfx_app::WindowContext;
use crate::graphics::DeltaTime;
use crate::graphics::orientation::{Orientation, Stance};
use crate::graphics::shaders::{CharacterSpriteSheet, Position, Projection};
use crate::terrain::{TerrainDrawable, TerrainDrawSystem};

pub const COLOR_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Bgra8Unorm;
pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::D32Float;

pub struct ScreenTargets<'a> {
  pub extent: wgpu::Extent3d,
  pub color: &'a wgpu::TextureView,
  pub depth: &'a wgpu::TextureView,
}

pub struct DrawSystem {
  terrain_system: TerrainDrawSystem,
  character_system: CharacterDrawSystem,
  swap_chain: SwapChain,
  device: Device,
  depth_target: wgpu::TextureView,
  extent: wgpu::Extent3d,
  game_time: Instant,
  frames: u32,
  cool_down: f64,
  run_cool_down: f64,
  fire_cool_down: f64,
}

impl DrawSystem {
  pub fn new(window_context: &mut WindowContext) -> DrawSystem {
    let size = {
      let window = window_context.get_window();
      window
        .get_inner_size()
        .unwrap()
        .to_physical(window.get_hidpi_factor())
    };

    let instance = wgpu::Instance::new();
    let adapter = instance.get_adapter(&wgpu::AdapterDescriptor {
      power_preference: wgpu::PowerPreference::Default,
    });
    let mut device = adapter.create_device(&wgpu::DeviceDescriptor {
      extensions: wgpu::Extensions {
        anisotropic_filtering: false,
      },
    });

    let surface = instance.create_surface(&window_context.get_window());

    let extent = wgpu::Extent3d {
      width: size.width as u32,
      height: size.height as u32,
      depth: 1,
    };
    let sc_desc = wgpu::SwapChainDescriptor {
      usage: wgpu::TextureUsageFlags::OUTPUT_ATTACHMENT,
      format: wgpu::TextureFormat::Bgra8Unorm,
      width: extent.width,
      height: extent.height,
    };
    let depth_target = device
      .create_texture(&wgpu::TextureDescriptor {
        size: extent,
        array_size: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::D32Float,
        usage: wgpu::TextureUsageFlags::OUTPUT_ATTACHMENT,
      })
      .create_default_view();

    let swap_chain = device.create_swap_chain(&surface, &sc_desc);

    let terrain_system = TerrainDrawSystem::new(&sc_desc, &mut device);
    let character_system = CharacterDrawSystem::new(&sc_desc, &mut device);

    DrawSystem {
      terrain_system,
      character_system,
      swap_chain,
      device,
      depth_target,
      extent,
      game_time: Instant::now(),
      frames: 0,
      cool_down: 1.0,
      run_cool_down: 1.0,
      fire_cool_down: 1.0,
    }
  }

  fn update_terrain<'a>(&mut self, encoder: &'a mut CommandEncoder, drawable: &mut TerrainDrawable) {
    let new_projection_buf = self.device
      .create_buffer_mapped(1, wgpu::BufferUsageFlags::TRANSFER_SRC)
      .fill_from_slice(&[drawable.projection]);

    encoder.copy_buffer_to_buffer(
      &new_projection_buf,
      0,
      &self.terrain_system.projection_buf,
      0,
      1024,
    );

    let new_position_buf = self.device
      .create_buffer_mapped(1, wgpu::BufferUsageFlags::TRANSFER_SRC)
      .fill_from_slice(&[drawable.position]);

    encoder.copy_buffer_to_buffer(
      &new_position_buf,
      0,
      &self.terrain_system.position_buf,
      0,
      1024,
    );
  }

  fn get_next_sprite(&self, character_idx: usize, character_fire_idx: usize, drawable: &mut CharacterDrawable) -> CharacterSpriteSheet {
    let sprite_idx =
      if drawable.orientation == Orientation::Still && drawable.stance == Stance::Walking {
        (drawable.direction as usize * 28 + RUN_SPRITE_OFFSET)
      } else if drawable.stance == Stance::Walking {
        drawable.direction = drawable.orientation;
        (drawable.orientation as usize * 28 + character_idx + RUN_SPRITE_OFFSET)
      } else {
        (drawable.orientation as usize * 8 + character_fire_idx)
      } as usize;

    let elements_x = CHARACTER_SHEET_TOTAL_WIDTH / (drawable.critter_data[sprite_idx].data[2] + SPRITE_OFFSET);
    CharacterSpriteSheet {
      x_div: elements_x,
      y_div: 0.0,
      row_idx: 0,
      index: sprite_idx as u32,
    }
  }

  fn update_character<'a>(&mut self, encoder: &'a mut CommandEncoder, drawable: &mut CharacterDrawable) {
    let new_projection_buf = self.device
      .create_buffer_mapped(1, wgpu::BufferUsageFlags::TRANSFER_SRC)
      .fill_from_slice(&[drawable.projection]);

    encoder.copy_buffer_to_buffer(
      &new_projection_buf,
      0,
      &self.character_system.projection_buf,
      0,
      1024,
    );

    let new_position_buf = self.device
      .create_buffer_mapped(1, wgpu::BufferUsageFlags::TRANSFER_SRC)
      .fill_from_slice(&[drawable.position]);

    encoder.copy_buffer_to_buffer(
      &new_position_buf,
      0,
      &self.character_system.position_buf,
      0,
      1024,
    );

    let new_character_sprite_buf = self.device
      .create_buffer_mapped(1, wgpu::BufferUsageFlags::TRANSFER_SRC)
      .fill_from_slice(&[drawable.character_sprite]);

    encoder.copy_buffer_to_buffer(
      &new_character_sprite_buf,
      0,
      &self.character_system.character_sprite_buf,
      0,
      1024,
    );
  }
}

impl<'a> specs::prelude::System<'a> for DrawSystem {
  type SystemData = (WriteStorage<'a, TerrainDrawable>,
                     WriteStorage<'a, CharacterDrawable>,
                     WriteStorage<'a, CharacterSprite>,
                     Read<'a, DeltaTime>);

  fn run(&mut self, (mut terrain, mut character, mut character_sprite, dt): Self::SystemData) {
    use specs::join::Join;
    let mut last_time = time::Instant::now();
    let delta = dt.0;
    println!("delta {}", delta);

    let current_time = Instant::now();
    self.frames += 1;
    if cfg!(feature = "framerate") && (current_time.duration_since(self.game_time).as_secs()) >= 1 {
      println!("{:?} ms/frames", 1000.0 / f64::from(self.frames));
      self.frames = 0;
      self.game_time = Instant::now();
    }

    if self.cool_down == 0.0 {
      self.cool_down += 0.05;
    }
    if self.fire_cool_down == 0.0 {
      self.fire_cool_down += 0.2;
    }
    if self.run_cool_down == 0.0 {
      self.run_cool_down += 0.02;
    }
    self.cool_down = (self.cool_down - delta).max(0.0);
    self.run_cool_down = (self.run_cool_down - delta).max(0.0);
    self.fire_cool_down = (self.fire_cool_down - delta).max(0.0);

    let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { todo: 0 });

    // Update uniform buffers
    for (t, c, cs) in (&mut terrain, &mut character, &mut character_sprite).join() {
      self.update_terrain(&mut encoder, t);
      self.update_character(&mut encoder, c);

      if self.cool_down == 0.0 {
        if c.stance == Stance::Walking {
          cs.update_run();
        }
      } else if self.fire_cool_down == 0.0 && c.stance == Stance::Firing {
        cs.update_fire();
      }
    }

    {
      let mut render_pass = {
        let frame = self.swap_chain.get_next_texture();
        let targets = ScreenTargets {
          extent: self.extent,
          color: &frame.view,
          depth: &self.depth_target,
        };

        encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
          color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
            attachment: &targets.color,
            load_op: wgpu::LoadOp::Clear,
            store_op: wgpu::StoreOp::Store,
            clear_color: wgpu::Color {
              r: 0.1,
              g: 0.1,
              b: 0.1,
              a: 1.0,
            },
          }],
          depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachmentDescriptor {
            attachment: &targets.depth,
            depth_load_op: wgpu::LoadOp::Clear,
            depth_store_op: wgpu::StoreOp::Store,
            stencil_load_op: wgpu::LoadOp::Clear,
            stencil_store_op: wgpu::StoreOp::Store,
            clear_depth: 1.0,
            clear_stencil: 0,
          }),
        })
      };
      self.terrain_system.draw(&mut render_pass);
      self.character_system.draw(&mut render_pass);
    }

    // Nasty hack waits proper solution
    for _ in 0..2 {
      self.swap_chain.get_next_texture();
    }
    self.device.get_queue().submit(&[encoder.finish()]);
  }
}
