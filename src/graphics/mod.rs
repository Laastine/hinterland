use game::gfx_macros::Projection;
use cgmath;
use cgmath::{Matrix4, Point3, Vector3, Point2, Rad};
use specs;
use terrain;
use game::constants::{RESOLUTION_X, RESOLUTION_Y};
use character::orientation::Orientation;
use character::gfx_macros::Position;
use gfx_app::mouse_controls::MouseInputState;

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

  pub fn world_to_projection(&self, input: &mut terrain::controls::CameraInputState) -> Projection {
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

fn direction(start_point: Point2<f32>, end_point: Point2<f32>) -> Rad<f32> {
  cgmath::Angle::atan2(end_point.y - start_point.y, (end_point.x - start_point.x))
}

#[allow(dead_code)]
pub fn get_orientation(character: &Position, mouse_input: &mut MouseInputState) -> Orientation {
  if let Some(val) = mouse_input.left_click_point {
    let start_point = Point2::new(character.position[0], character.position[1]);
    print!("direction {:?}", direction(start_point, val))
  }
  Orientation::Right
}
