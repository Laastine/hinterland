use std;
use std::io::Cursor;
use std::mem;

use cgmath::Point2;
use rand::Rng;
use specs;
use specs::prelude::{Read, ReadStorage, WriteStorage};
use wgpu;
use wgpu::CommandBuffer;

use crate::character::{character_stats::CharacterStats, controls::CharacterInputState};
use crate::critter::{CharacterSprite, CritterData};
use crate::data;
use crate::game::constants::{AMMO_POSITIONS, ASPECT_RATIO, CHARACTER_SHEET_TOTAL_WIDTH, RUN_SPRITE_OFFSET, SPRITE_OFFSET, VIEW_DISTANCE};
use crate::graphics::{camera::CameraInputState, dimensions::{Dimensions, get_projection, get_view_matrix}, get_orientation_from_center, orientation::{Orientation, Stance}, overlaps};
use crate::graphics::mesh::create_vertices;
use crate::graphics::shaders::{CharacterSpriteSheet, load_glsl, Position, Projection, ShaderStage, Vertex};

pub mod controls;
mod character_stats;

const SHADER_VERT: &[u8] = include_bytes!("../shaders/character.v.glsl");
const SHADER_FRAG: &[u8] = include_bytes!("../shaders/character.f.glsl");

pub struct CharacterDrawable {
  pub stats: CharacterStats,
  pub projection: Projection,
  pub position: Position,
  pub orientation: Orientation,
  pub stance: Stance,
  pub direction: Orientation,
  pub critter_data: Vec<CritterData>,
}

impl CharacterDrawable {
  pub fn new() -> CharacterDrawable {
    let view = get_view_matrix(VIEW_DISTANCE);
    let projection = get_projection(view, ASPECT_RATIO);
    let stats = CharacterStats::new();

    let critter_data = data::load_character();
    CharacterDrawable {
      stats,
      projection,
      position: Position::origin(),
      orientation: Orientation::Right,
      stance: Stance::Walking,
      direction: Orientation::Right,
      critter_data,
    }
  }

  pub fn update(&mut self, world_to_clip: Projection, ci: &CharacterInputState) {
    self.projection = world_to_clip;

    if self.stance != Stance::NormalDeath {
      if ci.is_colliding {
        self.stance = Stance::Still;
      } else {
        self.stance = Stance::Walking;
        self.orientation = ci.orientation;
      }
    }
  }
}

impl Default for CharacterDrawable {
  fn default() -> Self {
    CharacterDrawable::new()
  }
}

impl specs::prelude::Component for CharacterDrawable {
  type Storage = specs::storage::HashMapStorage<CharacterDrawable>;
}

pub struct CharacterDrawSystem {
  vertex_buf: wgpu::Buffer,
  index_buf: wgpu::Buffer,
  index_count: usize,
  bind_group: wgpu::BindGroup,
  pub projection_buf: wgpu::Buffer,
  pub position_buf: wgpu::Buffer,
  pub character_sprite_buf: wgpu::Buffer,
  pipeline: wgpu::RenderPipeline,
}

impl CharacterDrawSystem {
  pub fn new(sc_desc: &wgpu::SwapChainDescriptor, device: &mut wgpu::Device) -> CharacterDrawSystem {
    let mut init_encoder =
      device.create_command_encoder(&wgpu::CommandEncoderDescriptor { todo: 0 });

    let vertex_size = mem::size_of::<Vertex>();
    let (vertex_data, index_data) = create_vertices(25.0, 35.0);

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
          visibility: wgpu::ShaderStageFlags::VERTEX | wgpu::ShaderStageFlags::FRAGMENT,
          ty: wgpu::BindingType::UniformBuffer,
        },
        wgpu::BindGroupLayoutBinding {
          binding: 1,
          visibility: wgpu::ShaderStageFlags::VERTEX | wgpu::ShaderStageFlags::FRAGMENT,
          ty: wgpu::BindingType::SampledTexture,
        },
        wgpu::BindGroupLayoutBinding {
          binding: 2,
          visibility: wgpu::ShaderStageFlags::VERTEX | wgpu::ShaderStageFlags::FRAGMENT,
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
        }
      ],
    });
    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
      bind_group_layouts: &[&bind_group_layout],
    });

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
      usage: wgpu::TextureUsageFlags::SAMPLED | wgpu::TextureUsageFlags::TRANSFER_DST,
    });
    let character_texture = texture.create_default_view();
    let temp_buf = device
      .create_buffer_mapped(img.len(), wgpu::BufferUsageFlags::TRANSFER_SRC)
      .fill_from_slice(img.into_raw().as_slice());

    init_encoder.copy_buffer_to_texture(
      wgpu::BufferCopyView {
        buffer: &temp_buf,
        offset: 0,
        row_pitch: 4 * width,
        image_height: 256,
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
      size: 1,
      usage: wgpu::BufferUsageFlags::UNIFORM | wgpu::BufferUsageFlags::TRANSFER_SRC,
    });

    let character_position = Position::origin();
    let position_buf = device
      .create_buffer_mapped(1, wgpu::BufferUsageFlags::UNIFORM | wgpu::BufferUsageFlags::TRANSFER_SRC)
      .fill_from_slice(&[character_position]);

    let character_sprite_buf = device
      .create_buffer_mapped(1, wgpu::BufferUsageFlags::UNIFORM | wgpu::BufferUsageFlags::TRANSFER_SRC)
      .fill_from_slice(&[CharacterSpriteSheet { x_div: 288.0, y_div: 0.0, row_idx: 0, index: 64 }]);

    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
      layout: &bind_group_layout,
      bindings: &[
        wgpu::Binding {
          binding: 0,
          resource: wgpu::BindingResource::Buffer {
            buffer: &projection_buf,
            range: 0..1,
          },
        },
        wgpu::Binding {
          binding: 1,
          resource: wgpu::BindingResource::TextureView(&character_texture),
        },
        wgpu::Binding {
          binding: 2,
          resource: wgpu::BindingResource::Sampler(&sampler),
        },
        wgpu::Binding {
          binding: 3,
          resource: wgpu::BindingResource::Buffer {
            buffer: &character_sprite_buf,
            range: 0..1,
          },
        },
        wgpu::Binding {
          binding: 4,
          resource: wgpu::BindingResource::Buffer {
            buffer: &position_buf,
            range: 0..1,
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
        depth_bias: 0,
        depth_bias_slope_scale: 0.0,
        depth_bias_clamp: 0.0,
      },
      primitive_topology: wgpu::PrimitiveTopology::TriangleList,
      color_states: &[wgpu::ColorStateDescriptor {
        format: sc_desc.format,
        color: wgpu::BlendDescriptor::REPLACE,
        alpha: wgpu::BlendDescriptor::REPLACE,
        write_mask: wgpu::ColorWriteFlags::ALL,
      }],
      depth_stencil_state: Some(wgpu::DepthStencilStateDescriptor {
        format: wgpu::TextureFormat::D32Float,
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

    CharacterDrawSystem {
      vertex_buf,
      index_buf,
      index_count: index_data.len(),
      bind_group,
      projection_buf,
      position_buf,
      character_sprite_buf,
      pipeline,
    }
  }

  pub fn draw(&mut self,
              render_pass: &mut wgpu::RenderPass) {
    render_pass.set_pipeline(&self.pipeline);
    render_pass.set_bind_group(0, &self.bind_group);
    render_pass.set_index_buffer(&self.index_buf, 0);
    render_pass.set_vertex_buffers(&[(&self.vertex_buf, 0)]);
    render_pass.draw_indexed(0..self.index_count as u32, 0, 0..1);
  }
}

pub struct PreDrawSystem;

impl<'a> specs::prelude::System<'a> for PreDrawSystem {
  type SystemData = (WriteStorage<'a, CharacterDrawable>,
                     ReadStorage<'a, CameraInputState>,
                     ReadStorage<'a, CharacterInputState>,
                     Read<'a, Dimensions>);

  fn run(&mut self, (mut character, camera_input, character_input, dim): Self::SystemData) {
    use specs::join::Join;

    for (c, camera, ci) in (&mut character, &camera_input, &character_input).join() {
      let world_to_clip = dim.world_to_projection(camera);
      c.update(world_to_clip, ci);
    }
  }
}
