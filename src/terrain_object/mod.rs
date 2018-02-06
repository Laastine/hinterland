use cgmath;
use character::controls::CharacterInputState;
use game::constants::ASPECT_RATIO;
use gfx;
use gfx_app::{ColorFormat, DepthFormat};
use graphics::{Dimensions, load_texture};
use graphics::camera::CameraInputState;
use shaders::{Position, Projection, static_element_pipeline, VertexData};
use specs;
use specs::{Fetch, ReadStorage, WriteStorage};
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
  pub fn new(position: cgmath::Point2<f32>) -> TerrainObjectDrawable {
    let view = Dimensions::get_view_matrix();
    TerrainObjectDrawable {
      projection: Projection {
        model: view.into(),
        view: view.into(),
        proj: cgmath::perspective(cgmath::Deg(75.0f32), ASPECT_RATIO, 0.1, 4000.0).into(),
      },
      position: Position::new([position.x, position.y]),
      previous_position: Position::new([0.0, 0.0]),
      offset_delta: Position::new([0.0, 0.0]),
    }
  }

  pub fn update(&mut self, world_to_clip: &Projection, ci: &CharacterInputState) {
    self.projection = *world_to_clip;

    self.offset_delta =
      Position::new([ci.x_movement - self.previous_position.position[0], ci.y_movement - self.previous_position.position[1]]);

    self.previous_position = Position::new([
      ci.x_movement,
      ci.y_movement
    ]);

    self.position = Position::new([
      self.position.position[0] + self.offset_delta.position[0],
      self.position.position[1] + self.offset_delta.position[1],
    ]);
  }
}

impl specs::Component for TerrainObjectDrawable {
  type Storage = specs::VecStorage<TerrainObjectDrawable>;
}

pub struct TerrainObjectDrawSystem<R: gfx::Resources> {
  bundle: gfx::pso::bundle::Bundle<R, static_element_pipeline::Data<R>>,
}

impl<R: gfx::Resources> TerrainObjectDrawSystem<R> {
  pub fn new<F>(factory: &mut F,
                rtv: gfx::handle::RenderTargetView<R, ColorFormat>,
                dsv: gfx::handle::DepthStencilView<R, DepthFormat>) -> TerrainObjectDrawSystem<R>
                where F: gfx::Factory<R> {
    use gfx::traits::FactoryExt;

    let vertex_data: Vec<VertexData> =
      vec![
        VertexData::new([-120.0, -120.0, 0.0], [0.0, 1.0]),
        VertexData::new([120.0, -120.0, 0.0], [1.0, 1.0]),
        VertexData::new([120.0, 120.0, 0.0], [1.0, 0.0]),
        VertexData::new([-120.0, -120.0, 0.0], [0.0, 1.0]),
        VertexData::new([120.0, 120.0, 0.0], [1.0, 0.0]),
        VertexData::new([-120.0, 120.0, 0.0], [0.0, 0.0]),
      ];

    let house_bytes = &include_bytes!("../../assets/maps/house.png")[..];

    let (vertex_buf, slice) = factory.create_vertex_buffer_with_slice(&vertex_data, ());

    let terrain_object_texture = load_texture(factory, house_bytes).unwrap();

    let pso = factory
      .create_pipeline_simple(SHADER_VERT,
                              SHADER_FRAG,
                              static_element_pipeline::new())
      .unwrap();

    let pipeline_data = static_element_pipeline::Data {
      vbuf: vertex_buf,
      position_cb: factory.create_constant_buffer(1),
      projection_cb: factory.create_constant_buffer(1),
      static_element_sheet: (terrain_object_texture, factory.create_sampler_linear()),
      out_color: rtv,
      out_depth: dsv,
    };

    TerrainObjectDrawSystem {
      bundle: gfx::Bundle::new(slice, pso, pipeline_data),
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

impl<'a> specs::System<'a> for PreDrawSystem {
  type SystemData = (ReadStorage<'a, CameraInputState>,
                     ReadStorage<'a, CharacterInputState>,
                     WriteStorage<'a, TerrainObjects>,
                     Fetch<'a, Dimensions>);

  fn run(&mut self, (camera_input, character_input, mut terrain_objects, dim): Self::SystemData) {
    use specs::Join;

    for (camera, ci, obj) in (&camera_input, &character_input, &mut terrain_objects).join() {
      let world_to_clip = dim.world_to_projection(camera);

      for o in &mut obj.objects {
        o.update(&world_to_clip, ci);
      }
    }
  }
}
