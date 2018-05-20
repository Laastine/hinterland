use cgmath::Point2;
use game::constants::TERRAIN_OBJECTS;
use graphics::{can_move_to_tile, direction};
use pathfinding::utils::absdiff;
use shaders::Position;

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
struct Pos(f32, f32);

impl Pos {
  pub fn new(x: f32, y: f32) -> Pos {
    Pos(x,y)
  }

  fn distance(&self, other: &Pos) -> f32 {
    (absdiff(self.0, other.0) + absdiff(self.1, other.1)) as f32
  }

  fn neighbours(&self) -> Vec<(Pos, f32)> {
    let &Pos(x, y) = self;
    let mut empty_tiles = [Pos::new(0.0, 0.0); 64 * 64].to_vec();

    for x in 0..64 {
      for y in 0..64 {
        empty_tiles[y + x * 64] = Pos::new(x as f32, y as f32);
      }
    }

    empty_tiles.iter()
               .filter(|ref e| !TERRAIN_OBJECTS.contains(&[e.0 as usize, e.1 as usize]))
               .map(|p| (Pos::new(p.0 as f32, p.1 as f32), 1.0))
               .collect()
  }

  pub fn calc_route(start_point: Pos, end_point: Pos) -> u32 {
    if can_move_to_tile(Position::new([start_point.0, start_point.1])) {
      let a = Point2::new(start_point.0, start_point.1);
      let b = Point2::new(end_point.0, end_point.1);
      direction(a, b)
    } else {
      0
    }
  }
}
