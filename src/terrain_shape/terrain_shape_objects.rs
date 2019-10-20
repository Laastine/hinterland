use specs;

use crate::game::constants::TILE_SIZE;
use crate::graphics::orientation::Orientation;
use crate::shaders::Position;
use crate::terrain_shape::TerrainShapeDrawable;

fn set_position(x: isize, y: isize) -> Position {
  Position::new(
    TILE_SIZE * x as f32,
    TILE_SIZE * 0.9 * y as f32,
  )
}

pub struct TerrainShapeObjects {
  pub objects: Vec<TerrainShapeDrawable>,
}

impl TerrainShapeObjects {
  pub fn new() -> TerrainShapeObjects {
    TerrainShapeObjects {
      objects: Vec::new(),
    }
  }

  pub fn small_hill(&mut self, x: isize, y: isize) {
    self.objects.push(TerrainShapeDrawable::new(set_position(x, y), Orientation::Normal));
    self.objects.push(TerrainShapeDrawable::new(set_position(x - 1, y + 1), Orientation::UpLeft));
    self.objects.push(TerrainShapeDrawable::new(set_position(x + 1, y + 1), Orientation::UpRight));
    self.objects.push(TerrainShapeDrawable::new(set_position(x - 1, y), Orientation::Left));
    self.objects.push(TerrainShapeDrawable::new(set_position(x + 1, y), Orientation::Right));
    self.objects.push(TerrainShapeDrawable::new(set_position(x - 1, y - 1), Orientation::DownLeft));
    self.objects.push(TerrainShapeDrawable::new(set_position(x + 1, y - 1), Orientation::DownRight));
    self.objects.push(TerrainShapeDrawable::new(set_position(x, y - 1), Orientation::Down));
    self.objects.push(TerrainShapeDrawable::new(set_position(x, y + 1), Orientation::Up));
  }
}

impl specs::prelude::Component for TerrainShapeObjects {
  type Storage = specs::storage::VecStorage<TerrainShapeObjects>;
}
