use cgmath;
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
        TerrainObjectDrawable::new(cgmath::Point2::new(0.0, 750.0)),
        TerrainObjectDrawable::new(cgmath::Point2::new(850.0, 250.0)),
      ]
    }
  }
}

impl specs::Component for TerrainObjects {
  type Storage = specs::VecStorage<TerrainObjects>;
}
