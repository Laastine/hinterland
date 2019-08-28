use cgmath::Point2;
use specs::{Read, ReadStorage, WriteStorage};

use crate::character::controls::CharacterInputState;
use crate::game::constants::{ASPECT_RATIO, VIEW_DISTANCE};
use crate::gfx_app::{ColorFormat, DepthFormat};
use crate::graphics::camera::CameraInputState;
use crate::graphics::dimensions::{Dimensions, get_projection, get_view_matrix};
use crate::graphics::mesh::{RectangularTexturedMesh};
use crate::graphics::orientation::Orientation;
use crate::graphics::texture::{load_texture, Texture};
use crate::shaders::{Position, Projection, static_element_pipeline, Time};
use crate::terrain_shape::terrain_shape_objects::TerrainShapeObjects;

pub mod terrain_shape_objects;

const SHADER_VERT: &[u8] = include_bytes!("../shaders/static_element.v.glsl");
const SHADER_FRAG: &[u8] = include_bytes!("../shaders/static_element.f.glsl");

pub struct TerrainShapeDrawable {
  projection: Projection,
  pub position: Position,
  previous_position: Position,
}

impl TerrainShapeDrawable {
  pub fn new(position: Position) -> TerrainShapeDrawable {
    let view = get_view_matrix(VIEW_DISTANCE);
    let projection = get_projection(view, ASPECT_RATIO);
    TerrainShapeDrawable {
      position,
      previous_position: Position::origin(),
      projection,
    }
  }

  pub fn update(&mut self, world_to_clip: &Projection, ci: &CharacterInputState) {
    self.projection = *world_to_clip;
    self.position = self.position + ci.movement - self.previous_position;
    self.previous_position = ci.movement;
  }
}

impl specs::prelude::Component for TerrainShapeDrawable{
  type Storage = specs::storage::VecStorage<TerrainShapeDrawable>;
}

pub struct TerrainShapeDrawSystem<R: gfx::Resources> {
  bundle: gfx::pso::bundle::Bundle<R, static_element_pipeline::Data<R>>
}

impl<R: gfx::Resources> TerrainShapeDrawSystem<R> {
  pub fn new<F>(factory: &mut F,
                rtv: gfx::handle::RenderTargetView<R, ColorFormat>,
                dsv: gfx::handle::DepthStencilView<R, DepthFormat>,
  ) -> TerrainShapeDrawSystem<R>
    where F: gfx::Factory<R> {
    use gfx::traits::FactoryExt;

    let terrain_shape_bytes = include_bytes!("../../assets/maps/shape.png");
    let terrain_shape_texture = load_texture(factory, terrain_shape_bytes);

    let rect_mesh = RectangularTexturedMesh::new(factory,
                                             Texture::new(terrain_shape_texture, None),
                                             Point2::new(36.0, 36.0),
                                                Some(45.0),
                                             Some(Orientation::Down)
    );

    let pso = factory.create_pipeline_simple(SHADER_VERT, SHADER_FRAG, static_element_pipeline::new())
      .expect("Terrain shape shader loading error");

    let pipeline_data = static_element_pipeline::Data {
      vbuf: rect_mesh.mesh.vertex_buffer,
      position_cb: factory.create_constant_buffer(1),
      time_passed_cb: factory.create_constant_buffer(1),
      projection_cb: factory.create_constant_buffer(1),
      static_element_sheet: (rect_mesh.mesh.texture.raw, factory.create_sampler_linear()),
      out_color: rtv,
      out_depth: dsv,
    };

    TerrainShapeDrawSystem {
      bundle: gfx::Bundle::new(rect_mesh.mesh.slice, pso, pipeline_data),
    }
  }

  pub fn draw<C>(&self,
                 drawable: &TerrainShapeDrawable,
                 time_passed: u64,
                 encoder: &mut gfx::Encoder<R, C>)
    where C: gfx::CommandBuffer<R> {
    encoder.update_constant_buffer(&self.bundle.data.projection_cb, &drawable.projection);
    encoder.update_constant_buffer(&self.bundle.data.position_cb, &drawable.position);
    encoder.update_constant_buffer(&self.bundle.data.time_passed_cb, &Time::new(time_passed));
    self.bundle.encode(encoder);
  }
}

pub struct PreDrawSystem;

impl<'a> specs::prelude::System<'a> for PreDrawSystem {
  type SystemData = (ReadStorage<'a, CameraInputState>,
                     ReadStorage<'a, CharacterInputState>,
                     WriteStorage<'a, TerrainShapeObjects>,
                     Read<'a, Dimensions>);

  fn run(&mut self, (camera_input,character_input, mut terrain_shape_objects, dim): Self::SystemData) {
    use specs::join::Join;

    for (camera, ci, ts_obj) in (&camera_input, &character_input, &mut terrain_shape_objects).join() {
      let world_to_clip = dim.world_to_projection(camera);

      for o in &mut ts_obj.objects {
        o.update(&world_to_clip, ci);
      }
    }
  }
}
