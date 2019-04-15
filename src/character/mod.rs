use std;
use std::io::Cursor;
use std::mem;

use cgmath::Point2;
use rand::Rng;
use specs;
use specs::prelude::{Read, ReadStorage, WriteStorage};
use wgpu;

use crate::character::{character_stats::CharacterStats, controls::CharacterInputState};
use crate::critter::{CharacterSprite, CritterData};
use crate::data;
use crate::game::constants::{AMMO_POSITIONS, ASPECT_RATIO, CHARACTER_SHEET_TOTAL_WIDTH, RUN_SPRITE_OFFSET, SPRITE_OFFSET, VIEW_DISTANCE};
use crate::graphics::{camera::CameraInputState, dimensions::{Dimensions, get_projection, get_view_matrix}, get_orientation_from_center, orientation::{Orientation, Stance}, overlaps};
use crate::graphics::shaders::{CharacterSpriteSheet, load_glsl, Position, Projection, ShaderStage, Vertex};
use wgpu::CommandBuffer;

pub mod controls;
mod character_stats;

const SHADER_VERT: &[u8] = include_bytes!("../shaders/character.v.glsl");
const SHADER_FRAG: &[u8] = include_bytes!("../shaders/character.f.glsl");

#[derive(Clone)]
pub struct CharacterDrawable {
  pub stats: CharacterStats,
  projection: Projection,
  pub position: Position,
  orientation: Orientation,
  pub stance: Stance,
  direction: Orientation,
}

impl CharacterDrawable {
  pub fn new() -> CharacterDrawable {
    let view = get_view_matrix(VIEW_DISTANCE);
    let projection = get_projection(view, ASPECT_RATIO);
    let stats = CharacterStats::new();
    CharacterDrawable {
      stats,
      projection,
      position: Position::origin(),
      orientation: Orientation::Right,
      stance: Stance::Walking,
      direction: Orientation::Right,
    }
  }

  pub fn update(&mut self, world_to_clip: &Projection, ci: &CharacterInputState, dimensions: &Dimensions) {
    self.projection = world_to_clip.clone();
  }
}

impl Default for CharacterDrawable {
  fn default() -> Self {
    CharacterDrawable::new()
  }
}

impl specs::prelude::Component for CharacterDrawable {
  type Storage = specs::storage::VecStorage<CharacterDrawable>;
}

fn create_vertices() -> (Vec<Vertex>, Vec<u16>) {
  let w = 20.0;
  let h = 28.0;
  let vertex_data: &[Vertex; 4] = &[
    Vertex::new([-w, -h, 0.0], [0.0, 1.0]),
    Vertex::new([w, -h, 0.0], [1.0, 1.0]),
    Vertex::new([w, h, 0.0], [1.0, 0.0]),
    Vertex::new([-w, h, 0.0], [0.0, 0.0]),
  ];

  let index_data = &[0, 1, 2, 2, 3, 0];

  (vertex_data.to_vec(), index_data.to_vec())
}

pub struct CharacterDrawSystem {
  vertex_buf: wgpu::Buffer,
  index_buf: wgpu::Buffer,
  index_count: usize,
  bind_group: wgpu::BindGroup,
  pub projection_buf: wgpu::Buffer,
  pub position_buf: wgpu::Buffer,
  pub character_sprite_sheet_buf: wgpu::Buffer,
  pipeline: wgpu::RenderPipeline,
  data: Vec<CritterData>,
}

impl CharacterDrawSystem {
  pub fn new(sc_desc: &wgpu::SwapChainDescriptor, device: &mut wgpu::Device) -> CharacterDrawSystem {
    let mut init_encoder =
      device.create_command_encoder(&wgpu::CommandEncoderDescriptor { todo: 0 });

    let vertex_size = mem::size_of::<Vertex>();
    let (vertex_data, index_data) = create_vertices();
    let vertex_buf = device
      .create_buffer_mapped(vertex_data.len(), wgpu::BufferUsageFlags::VERTEX)
      .fill_from_slice(&vertex_data);

    let index_buf = device
      .create_buffer_mapped(index_data.len(), wgpu::BufferUsageFlags::INDEX)
      .fill_from_slice(&index_data);

    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
      bindings: &[
        wgpu::BindGroupLayoutBinding {
          binding: 0,
          visibility: wgpu::ShaderStageFlags::VERTEX,
          ty: wgpu::BindingType::UniformBuffer,
        },
        wgpu::BindGroupLayoutBinding {
          binding: 1,
          visibility: wgpu::ShaderStageFlags::FRAGMENT,
          ty: wgpu::BindingType::SampledTexture,
        },
        wgpu::BindGroupLayoutBinding {
          binding: 2,
          visibility: wgpu::ShaderStageFlags::FRAGMENT,
          ty: wgpu::BindingType::Sampler,
        },
        wgpu::BindGroupLayoutBinding {
          binding: 3,
          visibility: wgpu::ShaderStageFlags::VERTEX | wgpu::ShaderStageFlags::FRAGMENT,
          ty: wgpu::BindingType::UniformBuffer,
        },
        wgpu::BindGroupLayoutBinding {
          binding: 4,
          visibility: wgpu::ShaderStageFlags::VERTEX | wgpu::ShaderStageFlags::FRAGMENT,
          ty: wgpu::BindingType::UniformBuffer,
        },
        wgpu::BindGroupLayoutBinding {
          binding: 5,
          visibility: wgpu::ShaderStageFlags::VERTEX | wgpu::ShaderStageFlags::FRAGMENT,
          ty: wgpu::BindingType::UniformBuffer,
        }
      ],
    });
    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
      bind_group_layouts: &[&bind_group_layout],
    });

    let size = 256u32;
    let texels = &include_bytes!("../../assets/character.png")[..];
    let img = image::load(Cursor::new(texels), image::PNG).unwrap().to_rgba();
    let (width, height) = img.dimensions();

    let texture_extent = wgpu::Extent3d {
      width,
      height,
      depth: 1,
    };
    let texture = device.create_texture(&wgpu::TextureDescriptor {
      size: texture_extent,
      array_size: 1,
      dimension: wgpu::TextureDimension::D2,
      format: wgpu::TextureFormat::Rgba8Unorm,
      usage: wgpu::TextureUsageFlags::TRANSFER_DST,
    });
    let texture_view = texture.create_default_view();
    let temp_buf = device
      .create_buffer_mapped(img.len(), wgpu::BufferUsageFlags::TRANSFER_SRC)
      .fill_from_slice(img.into_raw().as_slice());

    init_encoder.copy_buffer_to_texture(
      wgpu::BufferCopyView {
        buffer: &temp_buf,
        offset: 0,
        row_pitch: 4 * width,
        image_height: height,
      },
      wgpu::TextureCopyView {
        texture: &texture,
        level: 0,
        slice: 0,
        origin: wgpu::Origin3d {
          x: 0.0,
          y: 0.0,
          z: 0.0,
        },
      },
      texture_extent,
    );

    let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
      r_address_mode: wgpu::AddressMode::ClampToEdge,
      s_address_mode: wgpu::AddressMode::ClampToEdge,
      t_address_mode: wgpu::AddressMode::ClampToEdge,
      mag_filter: wgpu::FilterMode::Nearest,
      min_filter: wgpu::FilterMode::Linear,
      mipmap_filter: wgpu::FilterMode::Nearest,
      lod_min_clamp: -100.0,
      lod_max_clamp: 100.0,
      max_anisotropy: 0,
      compare_function: wgpu::CompareFunction::Always,
      border_color: wgpu::BorderColor::TransparentBlack,
    });

    let projection_buf = device.create_buffer(&wgpu::BufferDescriptor {
      size: 1024,
      usage: wgpu::BufferUsageFlags::UNIFORM | wgpu::BufferUsageFlags::TRANSFER_DST,
    });

    let character_position = Position::origin();
    let position_buf = device
      .create_buffer_mapped(16, wgpu::BufferUsageFlags::UNIFORM | wgpu::BufferUsageFlags::TRANSFER_DST)
      .fill_from_slice(&character_position.as_raw());

    let character_sprite = CharacterSprite::new();
    let character_sprite_sheet_buf = device
      .create_buffer(&wgpu::BufferDescriptor {size: 64, usage: wgpu::BufferUsageFlags::UNIFORM | wgpu::BufferUsageFlags::TRANSFER_DST });

    let character_sprite = CharacterSpriteSheet::new(0.0, 0.0, 0, 0);
    let character_sprite_buf = device
      .create_buffer_mapped(
        64,
        wgpu::BufferUsageFlags::UNIFORM | wgpu::BufferUsageFlags::TRANSFER_DST)
      .fill_from_slice(&character_sprite.as_raw());

    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
      layout: &bind_group_layout,
      bindings: &[
        wgpu::Binding {
          binding: 0,
          resource: wgpu::BindingResource::Buffer {
            buffer: &projection_buf,
            range: 0..1024,
          },
        },
        wgpu::Binding {
          binding: 1,
          resource: wgpu::BindingResource::TextureView(&texture_view),
        },
        wgpu::Binding {
          binding: 2,
          resource: wgpu::BindingResource::Sampler(&sampler),
        },
        wgpu::Binding {
          binding: 3,
          resource: wgpu::BindingResource::Buffer {
            buffer: &character_sprite_sheet_buf,
            range: 0..64,
          },
        },
        wgpu::Binding {
          binding: 4,
          resource: wgpu::BindingResource::Buffer {
            buffer: &character_sprite_buf,
            range: 0..64,
          },
        },
        wgpu::Binding {
          binding: 5,
          resource: wgpu::BindingResource::Buffer {
            buffer: &position_buf,
            range: 0..16,
          },
        },
      ],
    });

    let vs_bytes = load_glsl("src/shaders/character.v.glsl", ShaderStage::Vertex);
    let fs_bytes = load_glsl("src/shaders/character.f.glsl", ShaderStage::Fragment);
    let vs_module = device.create_shader_module(&vs_bytes);
    let fs_module = device.create_shader_module(&fs_bytes);

    let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
      layout: &pipeline_layout,
      vertex_stage: wgpu::PipelineStageDescriptor {
        module: &vs_module,
        entry_point: "main",
      },
      fragment_stage: wgpu::PipelineStageDescriptor {
        module: &fs_module,
        entry_point: "main",
      },
      rasterization_state: wgpu::RasterizationStateDescriptor {
        front_face: wgpu::FrontFace::Cw,
        cull_mode: wgpu::CullMode::Back,
        depth_bias: 2,
        depth_bias_slope_scale: 2.0,
        depth_bias_clamp: 0.0,
      },
      primitive_topology: wgpu::PrimitiveTopology::TriangleList,
      color_states: &[wgpu::ColorStateDescriptor {
        format: sc_desc.format,
        color: wgpu::BlendDescriptor::REPLACE,
        alpha: wgpu::BlendDescriptor::REPLACE,
        write_mask: wgpu::ColorWriteFlags::ALL,
      }],
      //      color_states: &[],
      depth_stencil_state: Some(wgpu::DepthStencilStateDescriptor {
        format: wgpu::TextureFormat::Bgra8Unorm,
        depth_write_enabled: true,
        depth_compare: wgpu::CompareFunction::Less,
        stencil_front: wgpu::StencilStateFaceDescriptor::IGNORE,
        stencil_back: wgpu::StencilStateFaceDescriptor::IGNORE,
        stencil_read_mask: 0,
        stencil_write_mask: 0,
      }),
      index_format: wgpu::IndexFormat::Uint16,
      vertex_buffers: &[wgpu::VertexBufferDescriptor {
        stride: vertex_size as u32,
        step_mode: wgpu::InputStepMode::Vertex,
        attributes: &[
          wgpu::VertexAttributeDescriptor {
            attribute_index: 0,
            format: wgpu::VertexFormat::Float4,
            offset: 0,
          },
          wgpu::VertexAttributeDescriptor {
            attribute_index: 1,
            format: wgpu::VertexFormat::Float2,
            offset: 4 * 4,
          },
        ],
      }],
      sample_count: 1,
    });

    let init_command_buf = init_encoder.finish();
    device.get_queue().submit(&[init_command_buf]);

    let data = data::load_character();

    CharacterDrawSystem {
      vertex_buf,
      index_buf,
      index_count: index_data.len(),
      bind_group,
      projection_buf,
      position_buf,
      character_sprite_sheet_buf,
      pipeline,
      data,
    }
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

    let elements_x = CHARACTER_SHEET_TOTAL_WIDTH / (self.data[sprite_idx].data[2] + SPRITE_OFFSET);
    CharacterSpriteSheet {
      x_div: elements_x,
      y_div: 0.0,
      row_idx: 0,
      index: sprite_idx as u32,
    }
  }

  pub fn draw(&mut self,
              mut drawable: &mut CharacterDrawable,
              character: &CharacterSprite,
              frame: &wgpu::SwapChainOutput,
              render_pass: &mut wgpu::RenderPass) {
//    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
//      todo: 0,
//    });

//      let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
//        color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
//          attachment: &frame.view,
//          load_op: wgpu::LoadOp::Clear,
//          store_op: wgpu::StoreOp::Store,
//          clear_color: wgpu::Color {
//            r: 16.0 / 256.0,
//            g: 16.0 / 256.0,
//            b: 20.0 / 256.0,
//            a: 1.0,
//          },
//        }],
//        depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachmentDescriptor {
//          attachment: &frame.view,
//          depth_load_op: wgpu::LoadOp::Clear,
//          depth_store_op: wgpu::StoreOp::Store,
//          stencil_load_op: wgpu::LoadOp::Clear,
//          stencil_store_op: wgpu::StoreOp::Store,
//          clear_depth: 1.0,
//          clear_stencil: 0,
//        }),
//      });

      let next_sprite = self.get_next_sprite(character.character_idx, character.character_fire_idx, &mut drawable);

      render_pass.set_pipeline(&self.pipeline);
      render_pass.set_bind_group(0, &self.bind_group);
      render_pass.set_index_buffer(&self.index_buf, 0);
      self.projection_buf.set_sub_data(0, &drawable.projection.as_raw());
      self.position_buf.set_sub_data(0, &drawable.position.as_raw());
      self.character_sprite_sheet_buf.set_sub_data(0, &next_sprite.as_raw());
      render_pass.set_vertex_buffers(&[(&self.vertex_buf, 0)]);
//    device.get_queue().submit(&[encoder.finish()]);
//    encoder.finish()
  }
}

pub struct PreDrawSystem;

impl PreDrawSystem {
  pub fn new() -> PreDrawSystem {
    PreDrawSystem {}
  }
}

impl<'a> specs::prelude::System<'a> for PreDrawSystem {
  type SystemData = (WriteStorage<'a, CharacterDrawable>,
                     ReadStorage<'a, CameraInputState>,
                     ReadStorage<'a, CharacterInputState>,
                     Read<'a, Dimensions>);

  fn run(&mut self, (mut character, camera_input, character_input, dim): Self::SystemData) {
    use specs::join::Join;

    for (c, camera, ci) in (&mut character, &camera_input, &character_input).join() {
      let world_to_clip = dim.world_to_projection(camera);
      c.update(&world_to_clip, ci, &dim);
    }
  }
}
