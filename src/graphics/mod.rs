use shaders::Projection;
use cgmath;
use cgmath::{Matrix4, Point3, Vector3, Point2, Rad};
use game::constants::{RESOLUTION_X, RESOLUTION_Y, VIEW_DISTANCE};
use graphics::orientation::Orientation;
use gfx_app::mouse_controls::MouseInputState;
use std::f32::consts::PI;

pub mod camera;
pub mod orientation;

#[derive(Debug)]
pub struct DeltaTime(pub f64);

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

  pub fn get_view_matrix() -> Matrix4<f32> {
    Matrix4::look_at(
      Point3::new(0.0, 0.0, VIEW_DISTANCE),
      Point3::new(0.0, 0.0, 0.0),
      Vector3::unit_y(),
    )
  }

  pub fn world_to_projection(&self, input: &mut camera::CameraInputState) -> Projection {
    let view: Matrix4<f32> = Matrix4::look_at(
      Point3::new(input.x_pos, -input.y_pos, input.distance),
      Point3::new(input.x_pos, -input.y_pos, 0.0),
      Vector3::unit_y(),
    );
    let aspect_ratio = self.width as f32 / self.height as f32;
    Projection {
      model: view.into(),
      view: view.into(),
      proj: cgmath::perspective(cgmath::Deg(60.0f32), aspect_ratio, 0.1, 4000.0).into(),
    }
  }
}

fn flip_y_axel(point: Point2<f32>) -> Point2<f32> {
  Point2 {
    x: point.x,
    y: RESOLUTION_Y as f32 - point.y
  }
}

fn direction(start_point: Point2<f32>, end_point: Point2<f32>) -> i32 {

  let theta= cgmath::Angle::atan2(end_point.y - start_point.y, (end_point.x - start_point.x));
  let angle = match theta {
    Rad(i) => i
  };
  let rad_to_deg = 180.0 / PI;
  let anglei = if angle < 0.0 {
    (angle + 2.0 * PI) * rad_to_deg
  } else {
    angle * rad_to_deg
  };
  anglei.floor() as i32
}

pub fn get_orientation(mouse_input: &mut MouseInputState) -> Orientation {
  if let Some(end_point_gl) = mouse_input.left_click_point {
    let start_point = Point2 {
      x: (RESOLUTION_X / 2) as f32,
      y: (RESOLUTION_Y / 2) as f32
    };
    let angle_in_degrees = direction(start_point, flip_y_axel(end_point_gl));

    match angle_in_degrees {
      345 ... 360 | 0 ... 22 => Orientation::Right,
      23 ... 68 => Orientation::UpRight,
      69 ... 114 => Orientation::Up,
      115 ... 160 => Orientation::UpLeft,
      161 ... 206 => Orientation::Left,
      207 ... 252 => Orientation::DownLeft,
      253 ... 298 => Orientation::Down,
      299 ... 344 => Orientation::DownRight,
      _ => unreachable!("Invalid orientation")
    }
  } else {
    Orientation::Right
  }
}

fn is_out_of_map_borders(point: Point2<f64>) -> bool {
  point.x > 0.0 && point.x <= 64.0 && point.y > 0.0 && point.y <= 64.0
}

pub fn is_collision(screen_pos: [f32; 2]) -> bool {
  let x_coord = screen_pos[0] as f64;
  let y_coord = screen_pos[1] as f64;
  let point = Point2 {
    x: (x_coord / (60.0) + y_coord / (60.0)).round() + 32.0,
    y: (y_coord / (60.0) - (x_coord / 60.0)).round() + 32.0
  };
  is_out_of_map_borders(point)
}
