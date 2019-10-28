use specs;

use crate::graphics::orientation::Orientation;
use crate::graphics::set_position;
use crate::terrain_shape::TerrainShapeDrawable;

pub struct TerrainShapeObjects {
  pub objects: Vec<TerrainShapeDrawable>,
}

impl TerrainShapeObjects {
  pub fn new() -> TerrainShapeObjects {
    TerrainShapeObjects {
      objects: Vec::new(),
    }
  }

  pub fn small_hill(&mut self, x: i32, y: i32) {
    self.objects.push(TerrainShapeDrawable::new(set_position(x, y), Orientation::Normal));
    self.objects.push(TerrainShapeDrawable::new(set_position(x - 1, y - 1), Orientation::DownLeft));
    self.objects.push(TerrainShapeDrawable::new(set_position(x + 1, y - 1), Orientation::DownRight));
    self.objects.push(TerrainShapeDrawable::new(set_position(x - 1, y + 1), Orientation::UpLeft));
    self.objects.push(TerrainShapeDrawable::new(set_position(x + 1, y + 1), Orientation::UpRight));
    self.objects.push(TerrainShapeDrawable::new(set_position(x - 2, y), Orientation::Left));
    self.objects.push(TerrainShapeDrawable::new(set_position(x + 2, y), Orientation::Right));
    self.objects.push(TerrainShapeDrawable::new(set_position(x, y - 1), Orientation::Down));
    self.objects.push(TerrainShapeDrawable::new(set_position(x, y + 1), Orientation::Up));
  }
}

impl specs::prelude::Component for TerrainShapeObjects {
  type Storage = specs::storage::VecStorage<TerrainShapeObjects>;
}
