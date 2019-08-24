use std::io::Cursor;
use std::mem;

use cgmath::Point2;
use specs;
use specs::prelude::{Read, ReadStorage, WriteStorage};

use crate::character::CharacterDrawable;
use crate::character::controls::CharacterInputState;
use crate::critter::{CharacterSprite, CritterData};
use crate::data;
use crate::game::constants::{ASPECT_RATIO, NORMAL_DEATH_SPRITE_OFFSET, SPRITE_OFFSET, VIEW_DISTANCE, ZOMBIE_SHEET_TOTAL_WIDTH, ZOMBIE_STILL_SPRITE_OFFSET};
use crate::game::get_random_bool;
use crate::graphics::{add_random_offset_to_screen_pos, calc_hypotenuse, camera::CameraInputState, can_move_to_tile, direction, direction_movement, direction_movement_180, GameTime, orientation_to_direction, overlaps};
use crate::graphics::dimensions::{Dimensions, get_projection, get_view_matrix};
use crate::graphics::mesh::create_vertices;
use crate::graphics::orientation::{Orientation, Stance};
use crate::graphics::shaders::{CharacterSpriteSheet, load_glsl, Position, Projection, ShaderStage, Vertex};
use crate::terrain::path_finding::calc_next_movement;
use crate::zombie::zombies::Zombies;

pub mod zombies;

pub struct ZombieDrawable {
  pub projection: Projection,
  pub position: Position,
  previous_position: Position,
  orientation: Orientation,
  pub stance: Stance,
  direction: Orientation,
  last_decision: i64,
  pub movement_direction: Point2<f32>,
  zombie_idx: usize,
  zombie_death_idx: usize,
  movement_speed: f32,
  health: f32,
  pub character_sprite: CharacterSpriteSheet,
  pub critter_data: Vec<CritterData>,
}

impl ZombieDrawable {
  pub fn new(position: Position) -> ZombieDrawable {
    let view = get_view_matrix(VIEW_DISTANCE);
    let projection = get_projection(view, ASPECT_RATIO);

    let critter_data = data::load_zombie();
    ZombieDrawable {
      projection,
      position,
      previous_position: Position::origin(),
      orientation: Orientation::Left,
      stance: Stance::Still,
      direction: Orientation::Left,
      last_decision: -2,
      movement_direction: Point2::new(0.0, 0.0),
      zombie_idx: 0,
      zombie_death_idx: 0,
      movement_speed: 0.0,
      health: 1.0,
      character_sprite: CharacterSpriteSheet::new(0, &critter_data.as_slice()),
      critter_data,
    }
  }


  fn get_next_sprite(&mut self) -> CharacterSpriteSheet {
    let sprite_idx = match self.stance {
      Stance::Still => {
        (self.direction as usize * 4 + self.zombie_idx)
      }
      Stance::Walking if self.orientation != Orientation::Still => {
        (self.direction as usize * 8 + self.zombie_idx + ZOMBIE_STILL_SPRITE_OFFSET)
      }
      Stance::Running if self.orientation != Orientation::Still => {
        (self.direction as usize * 8 + self.zombie_idx + ZOMBIE_STILL_SPRITE_OFFSET)
      }
      Stance::NormalDeath if self.orientation != Orientation::Still => {
        (self.direction as usize * 6 + self.zombie_death_idx + NORMAL_DEATH_SPRITE_OFFSET)
      }
      Stance::CriticalDeath if self.orientation != Orientation::Still => {
        (self.direction as usize * 8 + self.zombie_death_idx)
      }
      _ => {
        self.direction = self.orientation;
        (self.orientation as usize * 8 + self.zombie_idx + ZOMBIE_STILL_SPRITE_OFFSET)
      }
    } as usize;

    let (y_div, row_idx) =
      if self.stance == Stance::NormalDeath || self.stance == Stance::CriticalDeath {
        (0.0, 2)
      } else {
        (1.0, 2)
      };

    let elements_x = ZOMBIE_SHEET_TOTAL_WIDTH / (self.critter_data[sprite_idx].data[2] + SPRITE_OFFSET);
    CharacterSpriteSheet {
      x_div: elements_x,
      y_div,
      row_idx,
      index: sprite_idx as u32,
    }
  }

  pub fn update(&mut self, world_to_clip: Projection, ci: &CharacterInputState, game_time: u64) {
    self.projection = world_to_clip;
    self.character_sprite = self.get_next_sprite();

    let offset_delta = ci.movement - self.previous_position;
    self.previous_position = ci.movement;

    let x_y_distance_to_player = self.position - offset_delta;

    let distance_to_player = calc_hypotenuse(x_y_distance_to_player.x().abs(), x_y_distance_to_player.y().abs());

    let is_alive = self.health > 0.0 && self.stance != Stance::NormalDeath && self.stance != Stance::CriticalDeath;

    if is_alive {
      let zombie_pos = ci.movement - self.position;

      if distance_to_player < 400.0 {
        let dir = calc_next_movement(zombie_pos, self.previous_position) as f32;
        self.direction = orientation_to_direction(dir);
        self.movement_direction = direction_movement(dir);
        self.stance = Stance::Running;
        self.movement_speed = 2.4 * self.health;
      } else {
        self.idle_direction_movement(zombie_pos, game_time as i64);
        self.movement_speed = 1.2 * self.health;
      }
    } else {
      self.movement_direction = Point2::new(0.0, 0.0);
    }

    self.position = Position::new(self.movement_direction.x * self.movement_speed,
                                  self.movement_direction.y * self.movement_speed) + self.position + offset_delta;
  }

  fn idle_direction_movement(&mut self, zombie_pos: Position, game_time: i64) {
    if !can_move_to_tile(zombie_pos) {
      let dir = direction(self.movement_direction, Point2::new(0.0, 0.0));
      self.movement_direction = direction_movement_180(self.movement_direction);
      self.orientation = orientation_to_direction(dir);
      self.direction = orientation_to_direction(dir);
    }

    if self.last_decision + 2 < game_time {
      self.stance = Stance::Walking;
      self.last_decision = game_time;
      let end_point = add_random_offset_to_screen_pos(zombie_pos);
      let dir = calc_next_movement(zombie_pos, end_point) as f32;
      self.movement_direction = direction_movement(dir);
      self.direction = orientation_to_direction(dir);
    }
  }

  pub fn update_alive_idx(&mut self, max_idx: usize) {
    if self.zombie_idx < max_idx {
      self.zombie_idx += 1;
    } else {
      self.zombie_idx = 0;
    }
  }

  pub fn update_death_idx(&mut self, max_idx: usize) {
    if self.zombie_death_idx < max_idx {
      self.zombie_death_idx += 1;
    }
  }
}

pub struct ZombieDrawSystem {
  vertex_buf: wgpu::Buffer,
  index_buf: wgpu::Buffer,
  index_count: usize,
  bind_group: wgpu::BindGroup,
  pub projection_buf: wgpu::Buffer,
  pub position_buf: wgpu::Buffer,
  pub zombie_sprite_buf: wgpu::Buffer,
  pipeline: wgpu::RenderPipeline,
}

impl ZombieDrawSystem {
  pub fn new(sc_desc: &wgpu::SwapChainDescriptor, device: &mut wgpu::Device) -> ZombieDrawSystem {
    let mut init_encoder =
      device.create_command_encoder(&wgpu::CommandEncoderDescriptor { todo: 0 });

    let vertex_size = mem::size_of::<Vertex>();
    let (vertex_data, index_data) = create_vertices(25.0, 35.0);

    let vertex_buf = device
      .create_buffer_mapped(vertex_data.len(), wgpu::BufferUsage::VERTEX)
      .fill_from_slice(&vertex_data);

    let index_buf = device
      .create_buffer_mapped(index_data.len(), wgpu::BufferUsage::INDEX)
      .fill_from_slice(&index_data);

    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
      bindings: &[
        wgpu::BindGroupLayoutBinding {
          binding: 0,
          visibility: wgpu::ShaderStage::VERTEX | wgpu::ShaderStage::FRAGMENT,
          ty: wgpu::BindingType::UniformBuffer {
            dynamic: false
          },
        },
        wgpu::BindGroupLayoutBinding {
          binding: 1,
          visibility: wgpu::ShaderStage::VERTEX | wgpu::ShaderStage::FRAGMENT,
          ty: wgpu::BindingType::SampledTexture {
            multisampled: false,
            dimension: wgpu::TextureViewDimension::D2
          },
        },
        wgpu::BindGroupLayoutBinding {
          binding: 2,
          visibility: wgpu::ShaderStage::VERTEX | wgpu::ShaderStage::FRAGMENT,
          ty: wgpu::BindingType::Sampler,
        },
        wgpu::BindGroupLayoutBinding {
          binding: 3,
          visibility: wgpu::ShaderStage::VERTEX | wgpu::ShaderStage::FRAGMENT,
          ty: wgpu::BindingType::UniformBuffer {
            dynamic: false
          },
        },
        wgpu::BindGroupLayoutBinding {
          binding: 4,
          visibility: wgpu::ShaderStage::VERTEX | wgpu::ShaderStage::FRAGMENT,
          ty: wgpu::BindingType::UniformBuffer {
            dynamic: false
          },
        }
      ],
    });
    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
      bind_group_layouts: &[&bind_group_layout],
    });

    let texels = &include_bytes!("../../assets/zombie.png")[..];
    let img = image::load(Cursor::new(texels), image::PNG).unwrap().to_rgba();
    let (width, height) = img.dimensions();

    let texture_extent = wgpu::Extent3d {
      width,
      height,
      depth: 1,
    };
    let texture = device.create_texture(&wgpu::TextureDescriptor {
      size: texture_extent,
      array_layer_count: 1,
      mip_level_count: 1,
      sample_count: 1,
      dimension: wgpu::TextureDimension::D2,
      format: wgpu::TextureFormat::Rgba8Unorm,
      usage: wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::COPY_DST,
    });
    let character_texture = texture.create_default_view();
    let temp_buf = device
      .create_buffer_mapped(img.len(), wgpu::BufferUsage::COPY_SRC)
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
        mip_level: 0,
        array_layer: 0,
        origin: wgpu::Origin3d {
          x: 0.0,
          y: 0.0,
          z: 0.0,
        },
      },
      texture_extent,
    );

    let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
      address_mode_u: wgpu::AddressMode::ClampToEdge,
      address_mode_v: wgpu::AddressMode::ClampToEdge,
      address_mode_w: wgpu::AddressMode::ClampToEdge,
      mag_filter: wgpu::FilterMode::Nearest,
      min_filter: wgpu::FilterMode::Linear,
      mipmap_filter: wgpu::FilterMode::Nearest,
      lod_min_clamp: -100.0,
      lod_max_clamp: 100.0,
      compare_function: wgpu::CompareFunction::Always,
    });

    let projection_buf = device.create_buffer(&wgpu::BufferDescriptor {
      size: 1,
      usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_SRC,
    });

    let zombie_position = Position::origin();
    let position_buf = device
      .create_buffer_mapped(1, wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_SRC)
      .fill_from_slice(&[zombie_position]);

    let zombie_sprite_buf = device
      .create_buffer_mapped(1, wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_SRC)
      .fill_from_slice(&[CharacterSpriteSheet { x_div: 112.0, y_div: 1.0, row_idx: 2, index: 16 }]);

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
            buffer: &zombie_sprite_buf,
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

    let vs_bytes = load_glsl(include_str!("../shaders/character.v.glsl"), ShaderStage::Vertex);
    let fs_bytes = load_glsl(include_str!("../shaders/character.f.glsl"), ShaderStage::Fragment);
    let vs_module = device.create_shader_module(&vs_bytes);
    let fs_module = device.create_shader_module(&fs_bytes);

    let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
      layout: &pipeline_layout,
      vertex_stage: wgpu::ProgrammableStageDescriptor {
        module: &vs_module,
        entry_point: "main",
      },
      fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
        module: &fs_module,
        entry_point: "main",
      }),
      rasterization_state: Some(wgpu::RasterizationStateDescriptor {
        front_face: wgpu::FrontFace::Cw,
        cull_mode: wgpu::CullMode::Back,
        depth_bias: 0,
        depth_bias_slope_scale: 0.0,
        depth_bias_clamp: 0.0,
      }),
      primitive_topology: wgpu::PrimitiveTopology::TriangleList,
      color_states: &[wgpu::ColorStateDescriptor {
        format: sc_desc.format,
        color_blend: wgpu::BlendDescriptor::REPLACE,
        alpha_blend: wgpu::BlendDescriptor::REPLACE,
        write_mask: wgpu::ColorWrite::ALL,
      }],
      depth_stencil_state: Some(wgpu::DepthStencilStateDescriptor {
        format: wgpu::TextureFormat::R32Float,
        depth_write_enabled: true,
        depth_compare: wgpu::CompareFunction::Less,
        stencil_front: wgpu::StencilStateFaceDescriptor::IGNORE,
        stencil_back: wgpu::StencilStateFaceDescriptor::IGNORE,
        stencil_read_mask: 0,
        stencil_write_mask: 0,
      }),
      index_format: wgpu::IndexFormat::Uint16,
      vertex_buffers: &[wgpu::VertexBufferDescriptor {
        stride: vertex_size as wgpu::BufferAddress,
        step_mode: wgpu::InputStepMode::Vertex,
        attributes: &[
          wgpu::VertexAttributeDescriptor {
            shader_location: 0,
            format: wgpu::VertexFormat::Float4,
            offset: 0,
          },
          wgpu::VertexAttributeDescriptor {
            shader_location: 1,
            format: wgpu::VertexFormat::Float2,
            offset: 4 * 4,
          },
        ],
      }],
      sample_count: 1,
      sample_mask: 1,
      alpha_to_coverage_enabled: false,
    });

    device.get_queue().submit(&[init_encoder.finish()]);

    ZombieDrawSystem {
      vertex_buf,
      index_buf,
      index_count: index_data.len(),
      bind_group,
      projection_buf,
      position_buf,
      zombie_sprite_buf,
      pipeline,
    }
  }

  pub fn draw(&mut self,
              render_pass: &mut wgpu::RenderPass) {
    render_pass.set_pipeline(&self.pipeline);
    render_pass.set_bind_group(0, &self.bind_group, &[]);
    render_pass.set_index_buffer(&self.index_buf, 0);
    render_pass.set_vertex_buffers(0,&[(&self.vertex_buf, 0)]);
    render_pass.draw_indexed(0..self.index_count as u32, 0, 0..1);
  }
}

pub struct PreDrawSystem;

impl<'a> specs::prelude::System<'a> for PreDrawSystem {
  type SystemData = (WriteStorage<'a, Zombies>,
                     ReadStorage<'a, CameraInputState>,
                     ReadStorage<'a, CharacterInputState>,
                     Read<'a, Dimensions>,
                     Read<'a, GameTime>);

  fn run(&mut self, (mut zombies, camera_input, character_input, dim, gt): Self::SystemData) {
    use specs::join::Join;

    for (zs, camera, ci) in (&mut zombies, &camera_input, &character_input).join() {
      let world_to_clip = dim.world_to_projection(camera);

      for z in &mut zs.zombies {
        z.update(world_to_clip, ci, gt.0);
      }
    }
  }
}

