use game::constants::{AMMO_POSITIONS, HOUSE_POSITIONS, TREE_POSITIONS};
use shaders::Position;
use specs;
use terrain_object::{TerrainObjectDrawable, TerrainTexture};

pub struct TerrainObjects {
  pub objects: Vec<TerrainObjectDrawable>,
}

impl TerrainObjects {
  pub fn new() -> TerrainObjects {
    TerrainObjects {
      objects: vec![
        TerrainObjectDrawable::new(Position::new(AMMO_POSITIONS[0][0], AMMO_POSITIONS[0][1]), TerrainTexture::Ammo),
        TerrainObjectDrawable::new(Position::new(AMMO_POSITIONS[1][0], AMMO_POSITIONS[1][1]), TerrainTexture::Ammo),
        TerrainObjectDrawable::new(Position::new(AMMO_POSITIONS[2][0], AMMO_POSITIONS[2][1]), TerrainTexture::Ammo),
        TerrainObjectDrawable::new(Position::new(AMMO_POSITIONS[3][0], AMMO_POSITIONS[3][1]), TerrainTexture::Ammo),
        TerrainObjectDrawable::new(Position::new(HOUSE_POSITIONS[0][0], HOUSE_POSITIONS[0][1]), TerrainTexture::House),
        TerrainObjectDrawable::new(Position::new(HOUSE_POSITIONS[1][0], HOUSE_POSITIONS[1][1]), TerrainTexture::House),
        TerrainObjectDrawable::new(Position::new(TREE_POSITIONS[0][0], TREE_POSITIONS[0][1]), TerrainTexture::Tree),
        TerrainObjectDrawable::new(Position::new(TREE_POSITIONS[1][0], TREE_POSITIONS[1][1]), TerrainTexture::Tree),
        TerrainObjectDrawable::new(Position::new(TREE_POSITIONS[2][0], TREE_POSITIONS[2][1]), TerrainTexture::Tree),
        TerrainObjectDrawable::new(Position::new(TREE_POSITIONS[3][0], TREE_POSITIONS[3][1]), TerrainTexture::Tree),
        TerrainObjectDrawable::new(Position::new(TREE_POSITIONS[4][0], TREE_POSITIONS[4][1]), TerrainTexture::Tree),
      ]
    }
  }
}

impl specs::prelude::Component for TerrainObjects {
  type Storage = specs::storage::VecStorage<TerrainObjects>;
}
