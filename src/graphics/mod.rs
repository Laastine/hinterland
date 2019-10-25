use std::f32;

use cgmath;
use cgmath::{Angle, Deg, Point2};
use num::{Num, NumCast};

use crate::bullet::BulletDrawable;
use crate::character::CharacterDrawable;
use crate::game::{constants::{RESOLUTION_Y, TERRAIN_OBJECTS, TILE_SIZE, TILES_PCS_H, TILES_PCS_W, Y_OFFSET}, get_rand_from_range};
use crate::gfx_app::{mouse_controls::MouseInputState};
use crate::graphics::{dimensions::Dimensions, orientation::Orientation};
use crate::shaders::Position;
use crate::terrain_object::TerrainObjectDrawable;
use crate::zombie::ZombieDrawable;

pub mod camera;
pub mod dimensions;
mod graphics_test;
pub mod mesh;
pub mod orientation;
pub mod texture;

#[derive(Default)]
pub struct DeltaTime(pub f64);

#[derive(Default)]
pub struct GameTime(pub u64);

pub fn flip_y_axel(point: Point2<f32>) -> Point2<f32> {
  Point2::new(point.x, RESOLUTION_Y as f32 - point.y)
}

pub fn direction(start_point: Point2<f32>, end_point: Point2<f32>) -> f32 {
  let theta = Angle::atan2(end_point.y - start_point.y, end_point.x - start_point.x);
  let Deg(angle) = theta;
  if angle < 0.0 { (360.0 + angle) } else { angle }
}

pub fn direction_movement(direction: f32) -> Point2<f32> {
  let angle = Deg(direction);
  Point2::new(Angle::cos(angle), Angle::sin(angle))
}

pub fn direction_movement_180(movement_direction: Point2<f32>) -> Point2<f32> {
  let angle = Deg(direction(Point2::new(0.0, 0.0), movement_direction) + 180.0);
  Point2::new(Angle::cos(angle), Angle::sin(angle))
}

pub fn orientation_to_direction(angle_in_degrees: f32) -> Orientation {
  match angle_in_degrees as u32 {
    345..=360 | 0..=22 => Orientation::Right,
    23..=68 => Orientation::UpRight,
    69..=114 => Orientation::Up,
    115..=160 => Orientation::UpLeft,
    161..=206 => Orientation::Left,
    207..=252 => Orientation::DownLeft,
    253..=298 => Orientation::Down,
    299..=344 => Orientation::DownRight,
    _ => unreachable!("Invalid orientation")
  }
}

pub fn get_orientation_from_center(mouse_input: &MouseInputState, dim: &Dimensions) -> Orientation {
  if let Some(end_point_gl) = mouse_input.left_click_point {
    let start_point = Point2::new(dim.window_width / 2.0 * dim.hidpi_factor, dim.window_height / 2.0 * dim.hidpi_factor);
    let dir = direction(start_point, flip_y_axel(end_point_gl));
    orientation_to_direction(dir)
  } else {
    Orientation::Right
  }
}

pub fn overlaps(area: Position, el: Position, width: f32, height: f32) -> bool {
  area.x() - width < el.x() &&
    area.x() + width > el.x() &&
    area.y() - height < el.y() &&
    area.y() + height > el.y()
}

pub fn is_within_map_borders(point: Point2<f32>) -> bool {
  point.x >= 0.0 && point.x < (TILES_PCS_W as f32 - 1f32) && point.y >= 0.0 && point.y < (TILES_PCS_H as f32 - 1f32)
}

pub fn can_move(screen_pos: Position) -> bool {
  let point = coords_to_tile(screen_pos);
  is_within_map_borders(Point2::new(point.x as f32, point.y as f32))
}

fn is_not_terrain_object<T>(pos: Point2<T>) -> bool
  where T: NumCast + Num, i32: std::cmp::PartialEq<T> {
  !TERRAIN_OBJECTS.iter().any(|e| (e[0] == pos.x) && (e[1] == pos.y))
}

fn is_map_tile(pos: Point2<i32>) -> bool {
  pos.x > 0 && pos.y > 0 && pos.x < (TILES_PCS_W - 1) as i32 && pos.y < (TILES_PCS_H - 1) as i32
}

pub fn can_move_to_tile(screen_pos: Position) -> bool {
  let tile_pos = coords_to_tile(screen_pos);
  is_not_terrain_object(tile_pos) && is_map_tile(tile_pos)
}

pub fn check_terrain_elevation(critter_pos: Position, objects: &[[i32; 2]]) -> f32 {
  let nearest_hill = objects.iter()
    .map(|x|
      position_distance(
        critter_pos,
        Position::new(-x[0] as f32, -x[1] as f32).tile_center(TILE_SIZE * 1.5, TILE_SIZE * 1.0)
      )
    )
    .fold(100_000_000f32, |mut min, val| {
      if val < min {
        min = val;
      }
      min
    });

  if nearest_hill < TILE_SIZE*2.0 {
    (nearest_hill - TILE_SIZE*2.0).abs()
  } else {
    0.0
  }
}

pub fn set_position(x: i32, y: i32) -> Position {
  let x_val = x as f32;
  let y_val = y as f32;
  Position::new(
    TILE_SIZE * x_val,
    TILE_SIZE * 0.9 * y_val,
  )
}

pub fn coords_to_tile(position: Position) -> Point2<i32> {
  let pos = Point2::new(-position.x(), position.y() + Y_OFFSET);
  Point2::new(((pos.x + pos.y) / TILE_SIZE) as i32, ((pos.y - pos.x) / TILE_SIZE) as i32)
}

pub fn tile_to_coords(tile: Point2<i32>) -> Position {
  let new_tile = Point2::new(tile.x as f32, tile.y as f32);
  let x = round(new_tile.x * TILE_SIZE - new_tile.y / TILE_SIZE, 3);
  let y = round(new_tile.y * TILE_SIZE - new_tile.x / TILE_SIZE, 3);
  Position::new(-x, y - Y_OFFSET)
}

fn round(number: f32, precision: usize) -> f32 {
  let ten: f32 = 10.0;
  let divider = ten.powf(precision as f32);
  (number * divider).round() / divider
}

pub fn get_nearest_random_tile_position(pos: Position) -> Position {
  fn iter(pos: Position) -> Position {
    let offset = Position::new(get_rand_from_range(-2, 2) as f32, get_rand_from_range(-2, 2) as f32);
    let offset_point = Position::new(
      round(offset.x() * TILE_SIZE - offset.y() / TILE_SIZE, 3),
      round(offset.y() * TILE_SIZE - offset.y() / TILE_SIZE, 3),
    );
    pos + offset_point
  }
  loop {
    let res = iter(pos);
    if can_move_to_tile(res) {
      return res;
    }
  }
}

pub fn distance(a: f32, b: f32) -> f32 {
  (a.powf(2.0) + b.powf(2.0)).sqrt()
}

fn position_distance(a: Position, b: Position) -> f32 {
  let d = a - b;
  distance(d.x(), d.y())
}

pub enum Drawables<'b> {
  Bullet(&'b BulletDrawable),
  Character(&'b mut CharacterDrawable),
  TerrainAmmo(&'b TerrainObjectDrawable),
  TerrainHouse(&'b TerrainObjectDrawable),
  TerrainTree(&'b TerrainObjectDrawable),
  Zombie(&'b mut ZombieDrawable),
}

impl<'b> Drawables<'b> {
  pub fn get_vertical_pos(drawable: &Drawables) -> f32 {
    match drawable {
      Drawables::Bullet(e) => e.position.y(),
      Drawables::Zombie(e) => e.position.y(),
      Drawables::TerrainAmmo(e) => e.position.y(),
      Drawables::TerrainHouse(e) => e.position.y(),
      Drawables::TerrainTree(e) => e.position.y(),
      Drawables::Character(e) => e.position.y(),
    }
  }
}
