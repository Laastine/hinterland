use cgmath;
use cgmath::{Matrix4, Point3, Vector3};

//use crate::graphics::camera::CameraInputState;

pub struct Projection {
  model: [[f32; 4]; 4],
  view: [[f32; 4]; 4],
  proj: [[f32; 4]; 4],
}

#[derive(Clone, Default)]
pub struct Dimensions {
  pub window_width: f32,
  pub window_height: f32,
  pub hidpi_factor: f32,
}

impl Dimensions {
  pub fn new(window_width: f32, window_height: f32, hidpi_val: f32) -> Dimensions {
    let hidpi_factor = if cfg!(feature = "windowed") { 1.0 } else { hidpi_val };
    Dimensions {
      window_width,
      window_height,
      hidpi_factor,
    }
  }

  pub fn world_to_projection(&self/*, input: &CameraInputState*/) -> Projection {
    let view: Matrix4<f32> = get_view_matrix(1500.0);
    let aspect_ratio = self.window_width / self.window_height;
    get_projection(view, aspect_ratio)
  }
}

pub fn get_projection(view: Matrix4<f32>, aspect_ratio: f32) -> Projection {
  Projection {
    model: view.into(),
    view: view.into(),
    proj: cgmath::perspective(cgmath::Deg(75.0f32), aspect_ratio, 0.1, 4000.0).into(),
  }
}

pub fn get_view_matrix(view: f32) -> Matrix4<f32> {
  Matrix4::look_at(
    Point3::new(0.0, 0.0, view),
    Point3::new(0.0, 0.0, 0.0),
    Vector3::unit_y(),
  )
}
