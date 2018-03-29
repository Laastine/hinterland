use game::constants::{HOUSE_POSITIONS, TILE_WIDTH};
use shaders::Position;
use specs;
use terrain_object::TerrainObjectDrawable;

pub fn tile_coords_to_screen_coords(coords: [f32; 2]) -> [f32; 2] {
  [coords[0] * TILE_WIDTH, coords[1] * TILE_WIDTH]
}

//fn element_tiles(x_val: usize, y_val: usize, start_pos: [f32; 2]) -> Vec<Point2<usize>> {
//  let mut res = Vec::with_capacity(x_val*y_val+2);
//  for x in 0..x_val {
//    for y in 0..y_val {
//      res.push(Point2::new(start_pos[0] as usize + x, start_pos[1] as usize + y));
//    }
//  }
//  res
//}

#[derive(Debug, Clone)]
pub struct TerrainObjects {
  pub objects: Vec<TerrainObjectDrawable>,
}

impl TerrainObjects {
  pub fn new() -> TerrainObjects {
    TerrainObjects {
      objects: vec![
        TerrainObjectDrawable::new(Position::new(tile_coords_to_screen_coords(HOUSE_POSITIONS[0]))),
      ]
    }
  }
}

impl specs::Component for TerrainObjects {
  type Storage = specs::VecStorage<TerrainObjects>;
}
