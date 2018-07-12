use bullet::bullets::Bullets;
use bullet::collision::Collision;
use cgmath;
use cgmath::Point2;
use character::controls::CharacterInputState;
use game::constants::{ASPECT_RATIO, BULLET_SPEED, VIEW_DISTANCE};
use gfx;
use gfx_app::{ColorFormat, DepthFormat};
use graphics::{camera::CameraInputState, can_move, dimensions::{Dimensions, get_projection, get_view_matrix}};
use graphics::can_move_to_tile;
use shaders::{bullet_pipeline, Position, Projection, VertexData};
use specs;
use specs::prelude::{Read, ReadStorage, WriteStorage};
use std::f32;

pub mod bullets;
pub mod collision;

const SHADER_VERT: &[u8] = include_bytes!("../shaders/bullet.v.glsl");
const SHADER_FRAG: &[u8] = include_bytes!("../shaders/bullet.f.glsl");

#[derive(Debug, Clone, PartialEq)]
pub struct BulletDrawable {
  projection: Projection,
  pub position: Position,
  previous_position: Position,
  offset_delta: Position,
  pub movement_direction: Point2<f32>,
  pub status: collision::Collision,
}

impl BulletDrawable {
  pub fn new(position: cgmath::Point2<f32>, movement_direction: Point2<f32>) -> BulletDrawable {
    let view = get_view_matrix(VIEW_DISTANCE);
    let projection = get_projection(view, ASPECT_RATIO);
    BulletDrawable {
      projection,
      position: Position::new([position.x, position.y]),
      previous_position: Position::new([0.0, 0.0]),
      offset_delta: Position::new([0.0, 0.0]),
      movement_direction,
      status: Collision::Flying,
    }
  }

  pub fn update(&mut self, world_to_clip: &Projection, ci: &CharacterInputState) {
    self.projection = *world_to_clip;

    self.offset_delta =
      if (ci.x_movement - self.previous_position.position[0]).abs() > f32::EPSILON ||
        (ci.y_movement - self.previous_position.position[1]).abs() > f32::EPSILON {
        Position::new([ci.x_movement - self.previous_position.position[0], ci.y_movement - self.previous_position.position[1]])
      } else {
        Position::new([self.offset_delta.position[0], self.offset_delta.position[1]])
      };

    self.previous_position = Position::new([
      ci.x_movement - (self.movement_direction.x * BULLET_SPEED / (5.0/3.0)),
      ci.y_movement + (self.movement_direction.y * BULLET_SPEED)]);

    self.position =
      Position::new([
        self.position.position[0] + self.offset_delta.position[0] + (self.movement_direction.x * BULLET_SPEED / (5.0/3.0)),
        self.position.position[1] + self.offset_delta.position[1] - (self.movement_direction.y * BULLET_SPEED)
      ]);

    let tile_pos = Position::new([ci.x_movement - self.position.position[0], ci.y_movement - self.position.position[1]]);

    if !can_move(self.position) {
      self.status = Collision::OutOfBounds;
    } else if !can_move_to_tile(tile_pos) {
      self.status = Collision::Hit;
    }
  }
}

pub struct BulletDrawSystem<R: gfx::Resources> {
  bundle: gfx::pso::bundle::Bundle<R, bullet_pipeline::Data<R>>,
}

impl<R: gfx::Resources> BulletDrawSystem<R> {
  pub fn new<F>(factory: &mut F,
                rtv: gfx::handle::RenderTargetView<R, ColorFormat>,
                dsv: gfx::handle::DepthStencilView<R, DepthFormat>) -> BulletDrawSystem<R>
                where F: gfx::Factory<R> {
    use gfx::traits::FactoryExt;

    let vertex_data: [VertexData; 4] = [
        VertexData::new([-1.5, -1.5], [0.0, 1.0]),
        VertexData::new([1.5, -1.5], [1.0, 1.0]),
        VertexData::new([1.5, 1.5], [1.0, 0.0]),
        VertexData::new([-1.5, 1.5], [0.0, 0.0]),
      ];

    let index_data: [u16; 6] = [0, 1, 2, 2, 3, 0];

    let (vertex_buf, slice) = factory.create_vertex_buffer_with_slice(&vertex_data, &index_data[..]);
    let pso = factory
      .create_pipeline_simple(SHADER_VERT,
                              SHADER_FRAG,
                              bullet_pipeline::new())
      .unwrap();

    let pipeline_data = bullet_pipeline::Data {
      vbuf: vertex_buf,
      projection_cb: factory.create_constant_buffer(1),
      position_cb: factory.create_constant_buffer(1),
      out_color: rtv,
      out_depth: dsv,
    };

    BulletDrawSystem {
      bundle: gfx::Bundle::new(slice, pso, pipeline_data),
    }
  }

  pub fn draw<C>(&mut self,
                 drawable: &BulletDrawable,
                 encoder: &mut gfx::Encoder<R, C>)
                 where C: gfx::CommandBuffer<R> {
    encoder.update_constant_buffer(&self.bundle.data.projection_cb, &drawable.projection);
    encoder.update_constant_buffer(&self.bundle.data.position_cb, &drawable.position);
    self.bundle.encode(encoder);
  }
}

#[derive(Debug)]
pub struct PreDrawSystem;

impl PreDrawSystem {
  pub fn new() -> PreDrawSystem {
    PreDrawSystem {}
  }
}

impl<'a> specs::prelude::System<'a> for PreDrawSystem {
  type SystemData = (ReadStorage<'a, CameraInputState>,
                     WriteStorage<'a, Bullets>,
                     ReadStorage<'a, CharacterInputState>,
                     Read<'a, Dimensions>);

  fn run(&mut self, (camera_input, mut bullets, character_input, dim): Self::SystemData) {
    use specs::join::Join;

    for (camera, bs, ci) in (&camera_input, &mut bullets, &character_input).join() {
      let world_to_clip = dim.world_to_projection(camera);

      for b in &mut bs.bullets {
        b.update(&world_to_clip, ci);
      }
    }
  }
}
