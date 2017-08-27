use game::gfx_macros::Projection;
use cgmath;
use cgmath::{Matrix4, Point3, Vector3};
use specs;
use terrain;
use game::constants::{RESOLUTION_X, RESOLUTION_Y};

pub type Delta = f32;
pub type Planner = specs::Planner<Delta>;

#[derive(Debug, Clone)]
pub struct Dimensions {
  width: u32,
  height: u32,
}

impl Dimensions {
  pub fn new(_window_width: u32, _window_height: u32) -> Dimensions {
    Dimensions {
      width: RESOLUTION_X,
      height: RESOLUTION_Y,
    }
  }

  pub fn world_to_projection(&self, input: &mut terrain::controls::TerrainInputState) -> Projection {
    let view: Matrix4<f32> = Matrix4::look_at(
      Point3::new(input.x_pos, -input.y_pos, input.distance),
      Point3::new(input.x_pos, -input.y_pos, 0.0),
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
