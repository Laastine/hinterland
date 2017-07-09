use terrain::gfx_macros::{Projection};
use cgmath;
use cgmath::{Matrix4, Matrix3, Point3, Vector3, Vector4, Quaternion, Decomposed, Transform};

#[derive(Debug,Clone)]
pub struct Dimensions {
  width: u32,
  height: u32,
}

impl Dimensions {
  pub fn new(_window_width: u32, _window_height: u32) -> Dimensions {
    Dimensions {
      width: 1280,
      height: 720,
    }
  }

  pub fn world_to_clip(&self) -> Projection {
    let view: Matrix4<f32> = Matrix4::look_at(
      Point3::new(0.0, 0.0, 2000.0),
      Point3::new(0.0, 0.0, 0.0),
      Vector3::unit_y(),
    );
    let aspect_ratio = self.width as f32 / self.height as f32;
    Projection {
      model: Matrix4::from(view).into(),
      view: view.into(),
      proj: cgmath::perspective(cgmath::Deg(60.0f32), aspect_ratio, 0.1, 4000.0).into(),
    }
  }
}
