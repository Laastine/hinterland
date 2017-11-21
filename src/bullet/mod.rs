use cgmath;
use cgmath::Matrix4;
use game::constants::ASPECT_RATIO;
use gfx;
use gfx_app::{ColorFormat, DepthFormat};
use graphics::orientation::Orientation;
use shaders::{bullet_pipeline, VertexData, Position, Projection};
use specs;

const SHADER_VERT: &'static [u8] = include_bytes!("../shaders/bullet.v.glsl");
const SHADER_FRAG: &'static [u8] = include_bytes!("../shaders/bullet.f.glsl");

#[derive(Debug)]
pub struct BulletDrawable {
  projection: Projection,
  position: Position,
  orientation: Orientation,
}

impl BulletDrawable {
  pub fn new(view: Matrix4<f32>, position: Position, orientation: Orientation) -> BulletDrawable {
    BulletDrawable {
      projection: Projection {
        model: view.into(),
        view: view.into(),
        proj: cgmath::perspective(cgmath::Deg(60.0f32), ASPECT_RATIO, 0.1, 4000.0).into(),
      },
      position,
      orientation,
    }
  }

  pub fn update(&mut self, world_to_clip: &Projection) {
    self.projection = *world_to_clip;
  }
}

impl specs::Component for BulletDrawable {
  type Storage = specs::VecStorage<BulletDrawable>;
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

    let vertex_data: Vec<VertexData> =
      vec![
        VertexData::new([-2.0, -2.0, 0.0], [0.0, 1.0]),
        VertexData::new([2.0, -2.0, 0.0], [1.0, 1.0]),
        VertexData::new([2.0, 2.0, 0.0], [1.0, 0.0]),
        VertexData::new([-2.0, -2.0, 0.0], [0.0, 1.0]),
        VertexData::new([2.0, 2.0, 0.0], [1.0, 0.0]),
        VertexData::new([-2.0, 2.0, 0.0], [0.0, 0.0]),
      ];

    let (vertex_buf, slice) = factory.create_vertex_buffer_with_slice(&vertex_data, ());
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
}
