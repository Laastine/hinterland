use std::mem::size_of;
use std::slice::from_raw_parts;

use cgmath;
use cgmath::{BaseFloat, Matrix4, Point3, Vector3};
use crate::graphics::camera::CameraInputState;

#[derive(Debug, Default)]
pub struct Position {
  position: [f32; 2],
}

impl Position {
  pub fn new<T: BaseFloat>(x: T, y: T) -> Position where f32: std::convert::From<T> {
    Position { position: [f32::from(x), f32::from(y)] }
  }

  pub fn new_from_array(pos: [f32; 2]) -> Position {
    Position { position: pos }
  }

  pub fn origin() -> Position {
    Position { position: [0.0, 0.0] }
  }

  pub fn x(self) -> f32 {
    self.position[0]
  }

  pub fn y(self) -> f32 {
    self.position[1]
  }
}

#[derive(Clone, Default)]
pub struct Projection {
  model: [[f32; 4]; 4],
  view: [[f32; 4]; 4],
  proj: [[f32; 4]; 4],
}

impl Projection {
  pub fn as_raw(&self) -> &[u8] {
    let all = [self.model, self.view, self.proj];
    unsafe {
      from_raw_parts(all.as_ptr() as *const u8, all.len() * size_of::<Projection>())
    }
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
    let view: Matrix4<f32> = get_view_matrix(input.distance);
    let aspect_ratio = self.window_width / self.window_height;
    get_projection(view, aspect_ratio)
  }
}
