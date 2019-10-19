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
    let mut objects = Vec::new();

//    for idx in -4..4 {
//      objects.push(TerrainShapeDrawable::new(set_position(idx, -idx+4), Orientation::DownLeft))
//    }
    objects.push(TerrainShapeDrawable::new(set_position(2,6), Orientation::UpRight));
    objects.push(TerrainShapeDrawable::new(set_position(0,6), Orientation::UpLeft));
    objects.push(TerrainShapeDrawable::new(set_position(1,5), Orientation::Normal));
    objects.push(TerrainShapeDrawable::new(set_position(0,5), Orientation::Left));
    objects.push(TerrainShapeDrawable::new(set_position(0,4), Orientation::DownLeft));
    objects.push(TerrainShapeDrawable::new(set_position(2,4), Orientation::DownRight));

    TerrainShapeObjects {
      objects
    }
  }
}

impl specs::prelude::Component for TerrainShapeObjects {
  type Storage = specs::storage::VecStorage<TerrainShapeObjects>;
}
