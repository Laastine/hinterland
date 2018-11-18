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
        TerrainObjectDrawable::new(Position::new_from_array(AMMO_POSITIONS[0]), TerrainTexture::Ammo),
        TerrainObjectDrawable::new(Position::new_from_array(AMMO_POSITIONS[1]), TerrainTexture::Ammo),
        TerrainObjectDrawable::new(Position::new_from_array(AMMO_POSITIONS[2]), TerrainTexture::Ammo),
        TerrainObjectDrawable::new(Position::new_from_array(AMMO_POSITIONS[3]), TerrainTexture::Ammo),
        TerrainObjectDrawable::new(Position::new_from_array(HOUSE_POSITIONS[0]), TerrainTexture::House),
        TerrainObjectDrawable::new(Position::new_from_array(HOUSE_POSITIONS[1]), TerrainTexture::House),
        TerrainObjectDrawable::new(Position::new_from_array(TREE_POSITIONS[0]), TerrainTexture::Tree),
        TerrainObjectDrawable::new(Position::new_from_array(TREE_POSITIONS[1]), TerrainTexture::Tree),
        TerrainObjectDrawable::new(Position::new_from_array(TREE_POSITIONS[2]), TerrainTexture::Tree),
        TerrainObjectDrawable::new(Position::new_from_array(TREE_POSITIONS[3]), TerrainTexture::Tree),
        TerrainObjectDrawable::new(Position::new_from_array(TREE_POSITIONS[4]), TerrainTexture::Tree),
      ]
    }
  }
}

impl specs::prelude::Component for TerrainObjects {
  type Storage = specs::storage::VecStorage<TerrainObjects>;
}
