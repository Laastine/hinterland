use game::constants::{HOUSE_POSITIONS, TREE_POSITIONS};
use shaders::Position;
use specs;
use terrain_object::TerrainObjectDrawable;

#[derive(Debug, Clone)]
pub struct TerrainObjects {
  pub objects: [TerrainObjectDrawable; 7],
}

impl TerrainObjects {
  pub fn new() -> TerrainObjects {
    TerrainObjects {
      objects: [
        TerrainObjectDrawable::new(Position::new(HOUSE_POSITIONS[0])),
        TerrainObjectDrawable::new(Position::new(HOUSE_POSITIONS[1])),
        TerrainObjectDrawable::new(Position::new(TREE_POSITIONS[0])),
        TerrainObjectDrawable::new(Position::new(TREE_POSITIONS[1])),
        TerrainObjectDrawable::new(Position::new(TREE_POSITIONS[2])),
        TerrainObjectDrawable::new(Position::new(TREE_POSITIONS[3])),
        TerrainObjectDrawable::new(Position::new(TREE_POSITIONS[4])),
      ]
    }
  }
}

impl specs::Component for TerrainObjects {
  type Storage = specs::VecStorage<TerrainObjects>;
}
