use cgmath::{Matrix4, Vector3};

#[derive(Debug,Clone)]
pub struct Dimensions {
  width: u32,
  height: u32,
}

impl Dimensions {
  pub fn new(_window_width: u32, _window_height: u32) -> Dimensions {
    Dimensions {
      width: 1000,
      height: 1000,
    }
  }

  pub fn game_width(&self) -> u32 {
    self.width
  }

  pub fn game_height(&self) -> u32 {
    self.height
  }

  pub fn world_to_clip(&self) -> Matrix4<f32> {
    Matrix4::from_translation(Vector3::new(-1.0, -1.0, 0.0)) *
      Matrix4::from_nonuniform_scale(2.0 / (self.width as f32), 2.0 / (self.height as f32), 1.0)
  }
}
