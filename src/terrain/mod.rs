use std::io::Cursor;
use std::mem;

use cgmath::Point2;
use genmesh::{generators::{IndexedPolygon, Plane, SharedVertex}, Triangulate, Vertices};
use image;
use specs;
use specs::{Read, ReadStorage, WriteStorage};
use wgpu::{CommandBuffer, CommandEncoder};

use crate::character::controls::CharacterInputState;
use crate::game::constants::{ASPECT_RATIO, TILE_SIZE, TILES_PCS_H, TILES_PCS_W, VIEW_DISTANCE};
use crate::graphics::{can_move_to_tile, coords_to_tile};
use crate::graphics::camera::CameraInputState;
use crate::graphics::dimensions::{Dimensions, get_projection, get_view_matrix};
use crate::graphics::shaders::{load_glsl, Position, Projection, ShaderStage, Vertex};

mod tile_map;
pub mod window;

pub struct TerrainDrawable {
  pub projection: Projection,
  pub position: Position,
  pub tile_position: Point2<i32>,
}

impl TerrainDrawable {
  pub fn new() -> TerrainDrawable {
    let view = get_view_matrix(VIEW_DISTANCE);
    let projection = get_projection(view, ASPECT_RATIO);
    TerrainDrawable {
      projection,
      position: Position::origin(),
      tile_position: coords_to_tile(Position::origin()),
    }
  }

  pub fn update(&mut self, world_to_clip: Projection, ci: &mut CharacterInputState) {
    self.projection = world_to_clip;
    if can_move_to_tile(ci.movement) {
      ci.is_colliding = false;
      self.position = ci.movement;
      self.tile_position = coords_to_tile(self.position);
    } else {
      ci.is_colliding = true;
    }
  }
}

impl specs::prelude::Component for TerrainDrawable {
  type Storage = specs::storage::HashMapStorage<TerrainDrawable>;
}

fn cartesian_to_isometric(point_x: f32, point_y: f32) -> (f32, f32) {
  ((point_x - point_y), (point_x + point_y) / 1.78)
}

fn create_vertices() -> (Vec<Vertex>, Vec<u16>) {
  let plane = Plane::subdivide(TILES_PCS_W, TILES_PCS_H);
  let vertex_data: Vec<Vertex> =
    plane.shared_vertex_iter()
      .map(|vertex| {
        let (raw_x, raw_y) = cartesian_to_isometric(vertex.pos.x, vertex.pos.y);
        let vertex_x = (TILE_SIZE * (TILES_PCS_W as f32) / 2.0) * raw_x;
        let vertex_y = (TILE_SIZE * (TILES_PCS_H as f32) / 2.0) * raw_y;

        let (u_pos, v_pos) = ((raw_x / 4.0 - raw_y / 2.25) + 0.5, (raw_x / 4.0 + raw_y / 2.25) + 0.5);
        let tile_map_x = u_pos * TILES_PCS_W as f32;
        let tile_map_y = v_pos * TILES_PCS_H as f32;

        Vertex::new([vertex_x, vertex_y, 0.0], [tile_map_x, tile_map_y])
      })
      .collect();

  let index_data = plane.indexed_polygon_iter()
    .triangulate()
    .vertices()
    .map(|i| i as u16)
    .collect::<Vec<u16>>();

  (vertex_data.to_vec(), index_data.to_vec())
}

pub struct TerrainDrawSystem {
  vertex_buf: wgpu::Buffer,
  index_buf: wgpu::Buffer,
  index_count: usize,
  bind_group: wgpu::BindGroup,
  pub projection_buf: wgpu::Buffer,
  pub position_buf: wgpu::Buffer,
  pipeline: wgpu::RenderPipeline,
}

impl TerrainDrawSystem {
  pub fn new(sc_desc: &wgpu::SwapChainDescriptor, device: &mut wgpu::Device) -> TerrainDrawSystem {
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
          visibility: wgpu::ShaderStageFlags::VERTEX,
          ty: wgpu::BindingType::UniformBuffer,
        }
      ],
    });
    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
      bind_group_layouts: &[&bind_group_layout],
    });

    let size = 256u32;
    let texels = &include_bytes!("../../assets/maps/terrain.png")[..];
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
    let texture_view = texture.create_default_view();
    let temp_buf = device
      .create_buffer_mapped(img.len(), wgpu::BufferUsageFlags::TRANSFER_SRC)
      .fill_from_slice(img.into_raw().as_slice());

    init_encoder.copy_buffer_to_texture(
      wgpu::BufferCopyView {
        buffer: &temp_buf,
        offset: 0,
        row_pitch: 4 * width,
        image_height: size,
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

    let projection_buf = device
      .create_buffer(&wgpu::BufferDescriptor {
        size: 1,
        usage: wgpu::BufferUsageFlags::UNIFORM | wgpu::BufferUsageFlags::TRANSFER_DST,
      });

    let terrain = tile_map::Terrain::new();

    let terrain_buf = device
      .create_buffer_mapped(
        4096,
        wgpu::BufferUsageFlags::UNIFORM | wgpu::BufferUsageFlags::TRANSFER_DST)
      .fill_from_slice(&terrain.tiles.as_slice());

    let terrain_position = Position::origin();
    let position_buf = device
      .create_buffer_mapped(1, wgpu::BufferUsageFlags::UNIFORM | wgpu::BufferUsageFlags::TRANSFER_DST)
      .fill_from_slice(&[terrain_position]);

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
          resource: wgpu::BindingResource::TextureView(&texture_view),
        },
        wgpu::Binding {
          binding: 2,
          resource: wgpu::BindingResource::Sampler(&sampler),
        },
        wgpu::Binding {
          binding: 3,
          resource: wgpu::BindingResource::Buffer {
            buffer: &terrain_buf,
            range: 0..64,
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

    let vs_bytes = load_glsl("src/shaders/terrain.v.glsl", ShaderStage::Vertex);
    let fs_bytes = load_glsl("src/shaders/terrain.f.glsl", ShaderStage::Fragment);
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

    TerrainDrawSystem {
      vertex_buf,
      index_buf,
      index_count: index_data.len(),
      bind_group,
      projection_buf,
      position_buf,
      pipeline,
    }
  }

  pub fn draw(&mut self,
              drawable: &mut TerrainDrawable,
              render_pass: &mut wgpu::RenderPass) {
    render_pass.set_pipeline(&self.pipeline);
    render_pass.set_bind_group(0, &self.bind_group);
    render_pass.set_index_buffer(&self.index_buf, 0);
    render_pass.set_vertex_buffers(&[(&self.vertex_buf, 0)]);
    render_pass.draw_indexed(0..self.index_count as u32, 0, 0..1);
  }
}

pub struct PreDrawSystem;

impl PreDrawSystem {
  pub fn new() -> PreDrawSystem {
    PreDrawSystem {}
  }
}

impl<'a> specs::prelude::System<'a> for PreDrawSystem {
  type SystemData = (WriteStorage<'a, TerrainDrawable>,
                     ReadStorage<'a, CameraInputState>,
                     WriteStorage<'a, CharacterInputState>,
                     Read<'a, Dimensions>);

  fn run(&mut self, (mut terrain, camera_input, mut character_input, dim): Self::SystemData) {
    use specs::join::Join;

    for (t, camera, ci) in (&mut terrain, &camera_input, &mut character_input).join() {
      let world_to_clip = dim.world_to_projection(camera);
      t.update(world_to_clip, ci);
    }
  }
}
