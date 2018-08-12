use cgmath::Point2;
use character::controls::CharacterInputState;
use game::constants::{ASPECT_RATIO, VIEW_DISTANCE};
use gfx;
use gfx_app::{ColorFormat, DepthFormat};
use graphics::{camera::CameraInputState, dimensions::{Dimensions, get_projection, get_view_matrix}, texture::load_texture};
use graphics::mesh::RectangularMesh;
use graphics::texture::Texture;
use shaders::{Position, Projection, static_element_pipeline};
use specs;
use specs::prelude::{Read, ReadStorage, WriteStorage};
use terrain_object::terrain_objects::TerrainObjects;

pub mod terrain_objects;

const SHADER_VERT: &[u8] = include_bytes!("../shaders/static_element.v.glsl");
const SHADER_FRAG: &[u8] = include_bytes!("../shaders/static_element.f.glsl");

#[derive(Debug, Clone)]
pub struct TerrainObjectDrawable {
  projection: Projection,
  pub position: Position,
  previous_position: Position,
  offset_delta: Position,
}

impl TerrainObjectDrawable {
  pub fn new(position: Position) -> TerrainObjectDrawable {
    let view = get_view_matrix(VIEW_DISTANCE);
    let projection = get_projection(view, ASPECT_RATIO);
    TerrainObjectDrawable {
      projection,
      position,
      previous_position: Position::new(0.0, 0.0),
      offset_delta: Position::new(0.0, 0.0),
    }
  }

  pub fn update(&mut self, world_to_clip: &Projection, ci: &CharacterInputState) {
    self.projection = *world_to_clip;
    self.position = self.position + ci.movement - self.previous_position;
    self.previous_position = ci.movement;
  }
}

impl specs::prelude::Component for TerrainObjectDrawable {
  type Storage = specs::storage::VecStorage<TerrainObjectDrawable>;
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TerrainTexture {
  House,
  Tree,
}

pub struct TerrainObjectDrawSystem<R: gfx::Resources> {
  bundle: gfx::pso::bundle::Bundle<R, static_element_pipeline::Data<R>>,
}

impl<R: gfx::Resources> TerrainObjectDrawSystem<R> {
  pub fn new<F>(factory: &mut F,
                rtv: gfx::handle::RenderTargetView<R, ColorFormat>,
                dsv: gfx::handle::DepthStencilView<R, DepthFormat>,
                texture: TerrainTexture) -> TerrainObjectDrawSystem<R>
                where F: gfx::Factory<R> {
    use gfx::traits::FactoryExt;

    let texture_bytes = match texture {
      TerrainTexture::House => &include_bytes!("../../assets/maps/house.png")[..],
      TerrainTexture::Tree => &include_bytes!("../../assets/maps/tree.png")[..],
    };

    let terrain_object_texture = load_texture(factory, texture_bytes);

    let mesh = RectangularMesh::new(factory, Texture::new(terrain_object_texture, None), Point2::new(120.0, 120.0));

    let pso =
      match factory.create_pipeline_simple(SHADER_VERT, SHADER_FRAG, static_element_pipeline::new()) {
        Ok(val) => val,
        Err(err) => panic!("Terrain object shader loading error {:?}", err)
      };

    let pipeline_data = static_element_pipeline::Data {
      vbuf: mesh.mesh.vertex_buffer,
      position_cb: factory.create_constant_buffer(1),
      projection_cb: factory.create_constant_buffer(1),
      static_element_sheet: (mesh.mesh.texture.raw, factory.create_sampler_linear()),
      out_color: rtv,
      out_depth: dsv,
    };

    TerrainObjectDrawSystem {
      bundle: gfx::Bundle::new(mesh.mesh.slice, pso, pipeline_data),
    }
  }

  pub fn draw<C>(&self,
                 drawable: &TerrainObjectDrawable,
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
                     ReadStorage<'a, CharacterInputState>,
                     WriteStorage<'a, TerrainObjects>,
                     Read<'a, Dimensions>);

  fn run(&mut self, (camera_input, character_input, mut terrain_objects, dim): Self::SystemData) {
    use specs::join::Join;

    for (camera, ci, obj) in (&camera_input, &character_input, &mut terrain_objects).join() {
      let world_to_clip = dim.world_to_projection(camera);

      for o in &mut obj.objects {
        o.update(&world_to_clip, ci);
      }
    }
  }
}
