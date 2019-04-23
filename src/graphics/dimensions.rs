use cgmath;
use cgmath::{BaseFloat, Matrix4, Point3, Vector3};
use nalgebra;

use crate::graphics::camera::CameraInputState;
use crate::graphics::shaders::Projection;

pub fn get_projection(view: nalgebra::base::Matrix4<f32>, aspect_ratio: f32) -> Projection {
  Projection {
    model: view.into(),
    view: view.into(),
    proj: nalgebra::Perspective3::new(aspect_ratio, 75.0f32, 0.1, 4000.0).into_inner().into(),
  }
}

pub fn get_view_matrix(view: f32) -> nalgebra::Matrix4<f32> {
  nalgebra::base::Matrix4::look_at_rh(
    &nalgebra::Point3::new(0.0, 0.0, view),
    &nalgebra::Point3::new(0.0, 0.0, 0.0),
    &nalgebra::Vector3::new(0.0, 1.0, 0.0),
  )
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

  pub fn world_to_projection(&self, input: &CameraInputState) -> Projection {
    let view = get_view_matrix(input.distance);
    let aspect_ratio = self.window_width / self.window_height;
    get_projection(view, aspect_ratio)
  }
}
