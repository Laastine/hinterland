use specs;

use crate::shaders::Position;
use crate::graphics::orientation::Orientation;
use crate::terrain_shape::TerrainShapeDrawable;

pub struct TerrainShapeObjects {
  pub objects: Vec<TerrainShapeDrawable>,
}

impl TerrainShapeObjects {
  pub fn new() -> TerrainShapeObjects {
    TerrainShapeObjects {
      objects: vec![
        TerrainShapeDrawable::new(Position::new(200.0, -100.0), Orientation::Right),
      ]
    }
  }
}

impl specs::prelude::Component for TerrainShapeObjects {
  type Storage = specs::storage::VecStorage<TerrainShapeObjects>;
}
