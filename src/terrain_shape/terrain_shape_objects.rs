use specs;

use crate::game::constants::TILE_SIZE;
use crate::graphics::orientation::Orientation;
use crate::shaders::Position;
use crate::terrain_shape::TerrainShapeDrawable;

fn set_position(x: isize, y: isize) -> Position {
  Position::new(
    TILE_SIZE * x as f32,
    TILE_SIZE * 0.9 * y as f32
  )
}

pub struct TerrainShapeObjects {
  pub objects: Vec<TerrainShapeDrawable>,
}

impl TerrainShapeObjects {
  pub fn new() -> TerrainShapeObjects {
    TerrainShapeObjects {
      objects: vec![
        TerrainShapeDrawable::new(set_position(-1, 5), Orientation::DownLeft),
        TerrainShapeDrawable::new(set_position(0, 4), Orientation::DownLeft),
        TerrainShapeDrawable::new(set_position(1, 3), Orientation::DownLeft),
        TerrainShapeDrawable::new(set_position(2, 2), Orientation::DownLeft),
      ]
    }
  }
}

impl specs::prelude::Component for TerrainShapeObjects {
  type Storage = specs::storage::VecStorage<TerrainShapeObjects>;
}
