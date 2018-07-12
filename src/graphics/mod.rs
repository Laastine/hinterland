use bullet::BulletDrawable;
use cgmath;
use cgmath::{Angle, Deg, Point2};
use character::CharacterDrawable;
use game::constants::{RESOLUTION_Y, TERRAIN_OBJECTS, TILE_WIDTH, TILES_PCS_H, TILES_PCS_W};
use gfx_app::{mouse_controls::MouseInputState};
use graphics::{dimensions::Dimensions, orientation::Orientation};
use shaders::Position;
use terrain_object::TerrainObjectDrawable;
use zombie::ZombieDrawable;

pub mod camera;
pub mod dimensions;
mod graphics_test;
pub mod orientation;
pub mod texture;

#[derive(Debug, Default)]
pub struct DeltaTime(pub f64);

pub fn flip_y_axel(point: Point2<f32>) -> Point2<f32> {
  Point2::new(point.x, RESOLUTION_Y as f32 - point.y)
}

pub fn direction(start_point: Point2<f32>, end_point: Point2<f32>) -> f32 {
  let theta = cgmath::Angle::atan2(end_point.y - start_point.y, end_point.x - start_point.x);
  let Deg(angle) = theta;
  if angle < 0.0 { (360.0 + angle) } else { angle }
}

pub fn direction_movement(direction: f32) -> Point2<f32> {
  let angle = Deg(direction);
  Point2::new(Angle::cos(angle), Angle::sin(angle))
}

pub fn get_orientation(angle_in_degrees: f32) -> Orientation {
  match angle_in_degrees as u32 {
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

pub fn overlaps(area: Position, el: Position, width: f32, height: f32) -> bool {
  area.position[0] - width < el.position[0] &&
    area.position[0] + width > el.position[0] &&
    area.position[1] - height < el.position[1] &&
    area.position[1] + height > el.position[1]
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
  Point2::new((pos.x / tile_width + (pos.y / tile_width)) as usize,
              (pos.y / tile_width - (pos.x / tile_width)) as usize)
}

pub fn coords_to_tile_offset(position: Position) -> Point2<i32> {
  let tile_width = TILE_WIDTH;
  let pos = Point2::new(-position.position[0], position.position[1] + 1500.0);
  Point2::new((pos.x / tile_width + (pos.y / tile_width)) as i32,
              (pos.y / tile_width - (pos.x / tile_width)) as i32)
}

pub fn tile_to_coords(tile: Point2<usize>) -> Position {
  let tile_width = TILE_WIDTH;
  let new_tile = Point2::new(tile.x as f32, tile.y as f32);
  let point = Point2::new(new_tile.x * tile_width - new_tile.y / tile_width,
                          new_tile.y * tile_width - new_tile.x / tile_width);
  Position::new([-point.x, point.y - 1500.0])
}

pub fn calc_hypotenuse(a: f32, b: f32) -> f32 {
  (a.powf(2.0) + b.powf(2.0)).sqrt()
}

pub enum Drawables<'b> {
  Bullet(&'b BulletDrawable),
  Character(&'b mut CharacterDrawable),
  TerrainHouse(&'b TerrainObjectDrawable),
  TerrainTree(&'b TerrainObjectDrawable),
  Zombie(&'b mut ZombieDrawable),
}

pub trait DrawOrder {
  fn get_zindex(&self) -> (f32, Drawables, &Self);
}
