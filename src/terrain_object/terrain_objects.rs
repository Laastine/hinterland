use game::constants::{HOUSE_POSITIONS, TREE_POSITIONS};
use shaders::Position;
use specs;
use terrain_object::TerrainObjectDrawable;

#[derive(Debug, Clone)]
pub struct TerrainObjects {
  pub objects: Vec<TerrainObjectDrawable>,
}

impl TerrainObjects {
  pub fn new() -> TerrainObjects {
    TerrainObjects {
      objects: vec![
        TerrainObjectDrawable::new(Position::new(HOUSE_POSITIONS[0])),
        TerrainObjectDrawable::new(Position::new(HOUSE_POSITIONS[1])),
        TerrainObjectDrawable::new(Position::new(TREE_POSITIONS[0])),
        TerrainObjectDrawable::new(Position::new(TREE_POSITIONS[1])),
      ]
    }
  }
}

impl specs::Component for TerrainObjects {
  type Storage = specs::VecStorage<TerrainObjects>;
}
