use game::constants::{AMMO_POSITIONS, HOUSE_POSITIONS, TREE_POSITIONS};
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
        TerrainObjectDrawable::new(Position::new(AMMO_POSITIONS[0][0], AMMO_POSITIONS[0][1])),
        TerrainObjectDrawable::new(Position::new(HOUSE_POSITIONS[0][0], HOUSE_POSITIONS[0][1])),
        TerrainObjectDrawable::new(Position::new(HOUSE_POSITIONS[1][0], HOUSE_POSITIONS[1][1])),
        TerrainObjectDrawable::new(Position::new(TREE_POSITIONS[0][0], TREE_POSITIONS[0][1])),
        TerrainObjectDrawable::new(Position::new(TREE_POSITIONS[1][0], TREE_POSITIONS[1][1])),
        TerrainObjectDrawable::new(Position::new(TREE_POSITIONS[2][0], TREE_POSITIONS[2][1])),
        TerrainObjectDrawable::new(Position::new(TREE_POSITIONS[3][0], TREE_POSITIONS[3][1])),
        TerrainObjectDrawable::new(Position::new(TREE_POSITIONS[4][0], TREE_POSITIONS[4][1])),
      ]
    }
  }
}

impl specs::prelude::Component for TerrainObjects {
  type Storage = specs::storage::VecStorage<TerrainObjects>;
}
