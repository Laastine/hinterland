use gfx::handle::ShaderResourceView;
use gfx::{texture, Factory, Resources};
use gfx::format::Rgba8;
use cgmath;
use cgmath::{Angle, Deg, Matrix4, Point3, Vector3, Point2};
use game::constants::{TILE_WIDTH, RESOLUTION_X, RESOLUTION_Y, VIEW_DISTANCE};
use graphics::orientation::Orientation;
use gfx_app::mouse_controls::MouseInputState;
use image;
use shaders::Projection;
use std::io::Cursor;

pub mod camera;
pub mod orientation;

#[derive(Debug)]
pub struct DeltaTime(pub f64);

#[derive(Debug, Clone)]
pub struct Dimensions {
  pub width: u32,
  pub height: u32,
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

  pub fn world_to_projection(&self, input: &camera::CameraInputState) -> Projection {
    let view: Matrix4<f32> = Matrix4::look_at(
      Point3::new(0.0, 0.0, input.distance),
      Point3::new(0.0, 0.0, 0.0),
      Vector3::unit_y(),
    );
    let aspect_ratio = self.width as f32 / self.height as f32;
    Projection {
      model: view.into(),
      view: view.into(),
      proj: cgmath::perspective(cgmath::Deg(75.0f32), aspect_ratio, 0.1, 4000.0).into(),
    }
  }
}

pub fn flip_y_axel(point: Point2<f32>) -> Point2<f32> {
  Point2 {
    x: point.x,
    y: RESOLUTION_Y as f32 - point.y
  }
}

pub fn direction(start_point: Point2<f32>, end_point: Point2<f32>) -> u32 {
  let theta= cgmath::Angle::atan2(end_point.y - start_point.y, end_point.x - start_point.x);
  let angle = match theta {
    Deg(i) => i
  };
  let a = angle.floor() as i32;
  if a < 0 { (360 + a) as u32 } else { a as u32 }
}

pub fn direction_movement(direction: u32) -> Point2<f32> {
  let f_direction = direction as f32;
  Point2 {
    x: (Angle::cos(Deg(f_direction)) * 100.0).round() / 100.0,
    y: (Angle::sin(Deg(f_direction)) * 100.0).round() / 100.0,
  }
}

pub fn get_orientation(mouse_input: &MouseInputState) -> Orientation {
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

fn is_within_map_borders(point: Point2<f64>) -> bool {
  point.x > 0.0 && point.x < 63.0 && point.y > 0.0 && point.y < 63.0
}
pub fn can_move(screen_pos: [f32; 2]) -> bool {
  let x_coord = f64::from(screen_pos[0]);
  let y_coord = f64::from(screen_pos[1]);
  let point = Point2 {
    x: (x_coord / TILE_WIDTH + y_coord / TILE_WIDTH).round() + 31.0,
    y: (y_coord / TILE_WIDTH - x_coord / TILE_WIDTH).round() + 32.0
  };
  is_within_map_borders(point)
}

pub fn load_texture<R, F>(factory: &mut F, data: &[u8]) -> Result<ShaderResourceView<R, [f32; 4]>, String> where R: Resources, F: Factory<R> {
  let img = image::load(Cursor::new(data), image::PNG).unwrap().to_rgba();
  let (width, height) = img.dimensions();
  let kind = texture::Kind::D2(width as texture::Size, height as texture::Size, texture::AaMode::Single);
  let (_, view) = factory.create_texture_immutable_u8::<Rgba8>(kind, &[&img]).unwrap();
  Ok(view)
}
