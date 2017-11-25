use cgmath;
use game::constants::{ASPECT_RATIO, RESOLUTION_X, RESOLUTION_Y};
use gfx;
use gfx_app::{ColorFormat, DepthFormat};
use gfx_app::mouse_controls::MouseInputState;
use graphics::{Dimensions, direction, flip_y_axel};
use graphics::camera::CameraInputState;
use shaders::{bullet_pipeline, VertexData, Position, Projection};
use specs;
use specs::{Fetch, ReadStorage, WriteStorage};

const SHADER_VERT: &'static [u8] = include_bytes!("../shaders/bullet.v.glsl");
const SHADER_FRAG: &'static [u8] = include_bytes!("../shaders/bullet.f.glsl");

#[derive(Debug, Clone)]
pub struct Bullet {
  pub position: Position,
  pub direction: u32,
}

impl Bullet {
  pub fn new(position: cgmath::Point2<f32>, direction: i32) -> Bullet {
    Bullet {
      position: Position {
        position: [position.x, position.y],
      },
      direction: direction as u32
    }
  }
}

#[derive(Debug, Clone)]
pub struct BulletDrawable {
  projection: Projection,
  pub bullets: Vec<Bullet>
}

impl BulletDrawable {
  pub fn new() -> BulletDrawable {
    let view = Dimensions::get_view_matrix();

    BulletDrawable {
      projection: Projection {
        model: view.into(),
        view: view.into(),
        proj: cgmath::perspective(cgmath::Deg(75.0f32), ASPECT_RATIO, 0.1, 4000.0).into(),
      },
      bullets: vec![],
    }
  }

  pub fn update(&mut self, world_to_clip: &Projection, mouse_input: &MouseInputState) {
    self.projection = *world_to_clip;
    if let Some(end_point_gl) = mouse_input.left_click_point {
      let start_point = cgmath::Point2 {
        x: (RESOLUTION_X / 2) as f32,
        y: (RESOLUTION_Y / 2) as f32
      };
      let angle_in_degrees = direction(start_point, flip_y_axel(end_point_gl));
      self.bullets.push(Bullet::new(cgmath::Point2 {
        x: 0.0,
        y: 0.0
      }, angle_in_degrees));
    }
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
        VertexData::new([-0.5, -0.5, 0.0], [0.0, 1.0]),
        VertexData::new([0.5, -0.5, 0.0], [1.0, 1.0]),
        VertexData::new([0.5, 0.5, 0.0], [1.0, 0.0]),
        VertexData::new([-0.5, -0.5, 0.0], [0.0, 1.0]),
        VertexData::new([0.5, 0.5, 0.0], [1.0, 0.0]),
        VertexData::new([-0.5, 0.5, 0.0], [0.0, 0.0]),
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

  pub fn draw<C>(&mut self,
                 drawable: &mut BulletDrawable,
                 encoder: &mut gfx::Encoder<R, C>)
    where C: gfx::CommandBuffer<R> {
    encoder.update_constant_buffer(&self.bundle.data.projection_cb, &drawable.projection);
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
                      WriteStorage<'a, BulletDrawable>,
                     ReadStorage<'a, MouseInputState>,
                     Fetch<'a, Dimensions>);

  fn run(&mut self, (camera_input, mut bullet, mouse_input, dim): Self::SystemData) {
    use specs::Join;

    for (camera, b, mi) in (&camera_input, &mut bullet, &mouse_input).join() {
      let world_to_clip = dim.world_to_projection(camera);
      b.update(&world_to_clip, mi);
    }
  }
}
