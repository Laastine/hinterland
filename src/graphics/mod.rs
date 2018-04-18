use bullet::BulletDrawable;
use cgmath;
use cgmath::{Angle, Deg, Matrix4, Point2, Point3, Vector3};
use character::CharacterDrawable;
use game::constants::{RESOLUTION_Y, TERRAIN_OBJECTS, TILE_WIDTH, TILES_PCS_H, TILES_PCS_W, VIEW_DISTANCE};
use gfx::{Factory, format::Rgba8, handle::ShaderResourceView, Resources, texture};
use gfx_app::mouse_controls::MouseInputState;
use graphics::orientation::Orientation;
use image;
use shaders::{Position, Projection};
use std::io::Cursor;
use terrain_object::TerrainObjectDrawable;
use zombie::ZombieDrawable;
use gfx_app::ColorFormat;

pub mod camera;
pub mod orientation;

#[derive(Debug)]
pub struct DeltaTime(pub f64);

#[derive(Debug, Clone)]
pub struct Dimensions {
  pub window_width: f32,
  pub window_height: f32,
  pub hidpi_factor: f32,
}

impl Dimensions {
  pub fn new(window_width: f32, window_height: f32, hidpi_factor: f32) -> Dimensions {
    Dimensions {
      window_width,
      window_height,
      hidpi_factor,
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
    let aspect_ratio = self.window_width / self.window_height;
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

pub fn get_orientation(angle_in_degrees: u32) -> Orientation {
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

pub fn get_orientation_from_center(mouse_input: &MouseInputState, dim: &Dimensions) -> Orientation {
  if let Some(end_point_gl) = mouse_input.left_click_point {
    let start_point = Point2::new(dim.window_width / 2.0 * dim.hidpi_factor, dim.window_height / 2.0 * dim.hidpi_factor);
    let dir = direction(start_point, flip_y_axel(end_point_gl));
    get_orientation(dir)
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

pub fn is_not_terrain_object(pos: Point2<usize>) -> bool {
  !TERRAIN_OBJECTS.iter().any(|e| (e[0] == pos.x) && (e[1] == pos.y))
}

fn is_map_tile(pos: Point2<usize>) -> bool {
  pos.x >= 1usize && pos.y >= 1usize && pos.x < TILES_PCS_W && pos.y < TILES_PCS_H
}

pub fn can_move_to_tile(screen_pos: Position) -> bool {
  let pos = coords_to_tile(screen_pos);
  is_not_terrain_object(pos) && is_map_tile(pos)
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

pub fn load_texture<R, F>(factory: &mut F, data: &[u8]) -> ShaderResourceView<R, [f32; 4]> where R: Resources, F: Factory<R> {
  let img = image::load(Cursor::new(data), image::PNG).unwrap().to_rgba();
  let (width, height) = img.dimensions();
  let kind = texture::Kind::D2(width as texture::Size, height as texture::Size, texture::AaMode::Single);
  match factory.create_texture_immutable_u8::<Rgba8>(kind, texture::Mipmap::Provided, &[&img]) {
    Ok(val) => val.1,
    Err(e) => panic!("Couldn't load texture {:?}", e)
  }
}

pub fn load_raw_texture<R, F>(factory: &mut F, data: &[u8], size: Point2<i32>) -> ShaderResourceView<R, [f32; 4]>
                      where R: Resources, F: Factory<R> {
  let kind = texture::Kind::D2(size.x as texture::Size, size.y as texture::Size, texture::AaMode::Single);
  let mipmap = texture::Mipmap::Provided;
  match factory
    .create_texture_immutable_u8::<ColorFormat>(kind, mipmap, &[data]) {
    Ok(val) => val.1,
    Err(e) => panic!("Couldn't load texture {:?}", e)
  }
}

pub enum Drawables {
  Bullet(BulletDrawable),
  Character(CharacterDrawable),
  TerrainHouse(TerrainObjectDrawable),
  TerrainTree(TerrainObjectDrawable),
  Zombie(ZombieDrawable),
}

pub trait DrawOrder {
  fn get_zindex(&self) -> (f32, Drawables, &Self);
}
