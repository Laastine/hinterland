use specs;

use crate::shaders::Position;
use crate::terrain_shape::{TerrainShapeDrawable, TerrainShapes};

pub struct TerrainShapeObjects {
  pub objects: Vec<TerrainShapeDrawable>,
}

impl TerrainShapeObjects {
  pub fn new() -> TerrainShapeObjects {
    TerrainShapeObjects {
      objects: vec![
        TerrainShapeDrawable::new(Position::new(-15.0, 234.0), TerrainShapes::DownLeft),
        TerrainShapeDrawable::new(Position::new(45.0, 234.0), TerrainShapes::DownLeft),
      ]
    }
  }
}

impl specs::prelude::Component for TerrainShapeObjects {
  type Storage = specs::storage::VecStorage<TerrainShapeObjects>;
}
