use cgmath;
use cgmath::{Angle, Deg, Matrix4, Point2, Point3, Vector3};
use game::constants::{RESOLUTION_X, RESOLUTION_Y, TILES_PCS_H, TILES_PCS_W, TILE_WIDTH, VIEW_DISTANCE};
use gfx::{Factory, Resources, texture};
use gfx::format::Rgba8;
use gfx::handle::ShaderResourceView;
use gfx_app::mouse_controls::MouseInputState;
use graphics::orientation::Orientation;
use image;
use shaders::{Position, Projection};
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
  Point2::new(point.x, RESOLUTION_Y as f32 - point.y)
}

pub fn direction(start_point: Point2<f32>, end_point: Point2<f32>) -> u32 {
  let theta = cgmath::Angle::atan2(end_point.y - start_point.y, end_point.x - start_point.x);
  let angle = match theta {
    Deg(i) => i
  };
  let a = angle.floor() as i32;
  if a < 0 { (360 + a) as u32 } else { a as u32 }
}

pub fn direction_movement(direction: u32) -> Point2<f32> {
  let f_direction = direction as f32;
  Point2::new((Angle::cos(Deg(f_direction)) * 100.0).round() / 100.0,
              (Angle::sin(Deg(f_direction)) * 100.0).round() / 100.0)
}

pub fn get_orientation(start_point: Point2<f32>, end_point: Point2<f32>) -> Orientation {
  let angle_in_degrees = direction(start_point, end_point);

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
}

pub fn get_orientation_from_center(mouse_input: &MouseInputState) -> Orientation {
  if let Some(end_point_gl) = mouse_input.left_click_point {
    let start_point = Point2::new((RESOLUTION_X / 2) as f32, (RESOLUTION_Y / 2) as f32);
    get_orientation(start_point, flip_y_axel(end_point_gl))
  } else {
    Orientation::Right
  }
}

pub fn overlaps(a: Position, b: Position, width: f32, height: f32) -> bool {
  a.position[0] < b.position[0] + width &&
    a.position[0] + width > b.position[0] &&
    a.position[1] < b.position[1] + height &&
    a.position[1] + a.position[1] > b.position[1]
}

fn is_within_map_borders(point: Point2<f32>) -> bool {
  point.x > 0.0 && point.x < 63.0 && point.y > 0.0 && point.y < 63.0
}

pub fn can_move(screen_pos: Position) -> bool {
  let tile_width = TILE_WIDTH;
  let x_coord = screen_pos.position[0];
  let y_coord = screen_pos.position[1];
  let point = Point2::new(
    (x_coord / tile_width + y_coord / tile_width).round() + 31.0,
    (y_coord / tile_width - x_coord / tile_width).round() + 32.0);
  is_within_map_borders(point)
}

pub fn can_move_to_tile(screen_pos: Position) -> bool {
  let pos = coords_to_tile(screen_pos);
  pos.x > 0 as usize && pos.y > 0 as usize && pos.x < TILES_PCS_W && pos.y < TILES_PCS_H
}

pub fn coords_to_tile(position: Position) -> Point2<usize> {
  let tile_width = TILE_WIDTH;
  let pos = Point2 {
    x: -position.position[0],
    y: position.position[1] + 1500.0,
  };
  Point2 {
    x : (pos.x / tile_width + (pos.y / tile_width)) as usize,
    y: (pos.y / tile_width - (pos.x / tile_width)) as usize,
  }
}

pub fn load_texture<R, F>(factory: &mut F, data: &[u8]) -> Result<ShaderResourceView<R, [f32; 4]>, String> where R: Resources, F: Factory<R> {
  let img = image::load(Cursor::new(data), image::PNG).unwrap().to_rgba();
  let (width, height) = img.dimensions();
  let kind = texture::Kind::D2(width as texture::Size, height as texture::Size, texture::AaMode::Single);
  let (_, view) = match factory.create_texture_immutable_u8::<Rgba8>(kind, texture::Mipmap::Provided, &[&img]) {
    Ok(val) => val,
    Err(e) => panic!("Couldn't create texture {:?}", e)
  };
  Ok(view)
}
