use terrain::gfx_macros::Projection;
use cgmath;
use cgmath::{Matrix4, Point3, Vector3, Point2, Deg};
use specs;
use terrain;

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
      width: 1280,
      height: 720,
    }
  }

  pub fn world_to_clip(&self, input: &mut terrain::controls::InputState) -> Projection {
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

#[derive(Debug, Clone)]
pub struct Position {
  pub position: Point2<f32>,
  pub orientation: Deg<f32>,
  pub scale: f32,
}

impl Position {
  pub fn new(x: f32, y: f32, angle: Deg<f32>, scale: f32) -> Position {
    Position {
      position: Point2::new(x, y),
      orientation: angle,
      scale: scale,
    }
  }

  pub fn model_to_world(&self) -> Matrix4<f32> {
    Matrix4::from_translation(Vector3::new(self.position.x, self.position.y, 0.0)) *
      Matrix4::from_nonuniform_scale(self.scale, self.scale, 1.0) *
      Matrix4::from_angle_z(-self.orientation)
  }
}

impl specs::Component for Position {
  type Storage = specs::VecStorage<Position>;
}
