use std::mem;

use crossbeam_channel as channel;
use specs::{Read, WriteStorage};
use wgpu::{CommandBuffer, CommandEncoder, Device, SwapChain, SwapChainDescriptor};

use crate::character::{CharacterDrawable, CharacterDrawSystem};
use crate::critter::CharacterSprite;
use crate::game::constants::{CHARACTER_SHEET_TOTAL_WIDTH, RUN_SPRITE_OFFSET, SPRITE_OFFSET};
//use crate::gfx_app::renderer::EncoderQueue;
use crate::gfx_app::WindowContext;
use crate::graphics::DeltaTime;
use crate::graphics::orientation::{Orientation, Stance};
use crate::graphics::shaders::{CharacterSpriteSheet, Position, Projection};
use crate::terrain::{TerrainDrawable, TerrainDrawSystem};

pub struct DrawSystem {
  terrain_system: TerrainDrawSystem,
  character_system: CharacterDrawSystem,
  swap_chain: SwapChain,
  device: Device,
//  encoder_queue: EncoderQueue,
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
      power_preference: wgpu::PowerPreference::LowPower,
    });
    let mut device = adapter.create_device(&wgpu::DeviceDescriptor {
      extensions: wgpu::Extensions {
        anisotropic_filtering: false,
      },
    });

    let surface = instance.create_surface(&window_context.get_window());

    let sc_desc = wgpu::SwapChainDescriptor {
      usage: wgpu::TextureUsageFlags::OUTPUT_ATTACHMENT,
      format: wgpu::TextureFormat::Bgra8Unorm,
      width: size.width as u32,
      height: size.height as u32,
    };

    let swap_chain = device.create_swap_chain(&surface, &sc_desc);

    let terrain_system = TerrainDrawSystem::new(&sc_desc, &mut device);
    let character_system = CharacterDrawSystem::new(&sc_desc, &mut device);

    DrawSystem {
      terrain_system,
      character_system,
      swap_chain,
      device,
//      encoder_queue,
    }
  }

  fn update_terrain<'a>(&mut self, encoder: &'a mut CommandEncoder, drawable: &mut TerrainDrawable) {
    let new_projection_buf = self.device
      .create_buffer_mapped(1, wgpu::BufferUsageFlags::TRANSFER_DST)
      .fill_from_slice(&[drawable.projection]);

    encoder.copy_buffer_to_buffer(
      &new_projection_buf,
      0,
      &self.terrain_system.projection_buf,
      0,
      1024,
    );

    let new_position_buf = self.device
      .create_buffer_mapped(1, wgpu::BufferUsageFlags::TRANSFER_DST)
      .fill_from_slice(&[drawable.position]);

    encoder.copy_buffer_to_buffer(
      &new_position_buf,
      0,
      &self.terrain_system.position_buf,
      0,
      1024,
    );
  }

  fn get_next_sprite(&self, character_idx: usize, character_fire_idx: usize, drawable: &CharacterDrawable) -> CharacterSpriteSheet {
    let sprite_idx =
      if drawable.orientation == Orientation::Still && drawable.stance == Stance::Walking {
        (drawable.direction as usize * 28 + RUN_SPRITE_OFFSET)
      } else if drawable.stance == Stance::Walking {
//        drawable.direction = drawable.orientation;
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

  fn update_character<'a>(&mut self, encoder: &'a mut CommandEncoder, drawable: &mut CharacterDrawable, character: &CharacterSprite) {
    let new_projection_buf = self.device
      .create_buffer_mapped(1, wgpu::BufferUsageFlags::TRANSFER_DST)
      .fill_from_slice(&[drawable.projection]);

    encoder.copy_buffer_to_buffer(
      &new_projection_buf,
      0,
      &self.character_system.projection_buf,
      0,
      1024,
    );

    let new_position_buf = self.device
      .create_buffer_mapped(1, wgpu::BufferUsageFlags::TRANSFER_DST)
      .fill_from_slice(&[drawable.position]);

    encoder.copy_buffer_to_buffer(
      &new_position_buf,
      0,
      &self.character_system.position_buf,
      0,
      1024,
    );
    let next_sprite = self.get_next_sprite(character.character_idx, character.character_fire_idx, &drawable);
    println!("next_sprite {:?}", next_sprite);
    let new_sprite_sheet_buf = self.device
      .create_buffer_mapped(1, wgpu::BufferUsageFlags::TRANSFER_DST)
      .fill_from_slice(&[next_sprite]);

    encoder.copy_buffer_to_buffer(
      &new_sprite_sheet_buf,
      0,
      &self.character_system.character_sprite_sheet_buf,
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

    let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { todo: 0 });

    for (t, c, cs) in (&mut terrain, &mut character, &mut character_sprite).join() {
      self.update_terrain(&mut encoder, t);
      self.update_character(&mut encoder, c, cs)
    }
    {
      let mut render_pass = {
        let texture_view = &self.swap_chain.get_next_texture().view;
        encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
          color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
            attachment: &texture_view,
            load_op: wgpu::LoadOp::Clear,
            store_op: wgpu::StoreOp::Store,
            clear_color: wgpu::Color {
              r: 16.0 / 256.0,
              g: 16.0 / 256.0,
              b: 20.0 / 256.0,
              a: 1.0,
            },
          }],
          depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachmentDescriptor {
            attachment: &texture_view,
            depth_load_op: wgpu::LoadOp::Clear,
            depth_store_op: wgpu::StoreOp::Store,
            stencil_load_op: wgpu::LoadOp::Clear,
            stencil_store_op: wgpu::StoreOp::Store,
            clear_depth: 1.0,
            clear_stencil: 0,
          }),
        })
      };

      let delta = dt.0;
      println!("delta {}", delta);

      self.swap_chain.get_next_texture();
      self.swap_chain.get_next_texture();
      for (t, c) in (&mut terrain, &mut character).join() {
        self.character_system.draw(c, &mut render_pass);
        self.terrain_system.draw(t, &mut render_pass);
      }
    }
    self.device.get_queue().submit(&[encoder.finish()]);
//    let _ = self.encoder_queue.sender.send(encoder);
  }
}
