use cgmath::Point2;
use game::constants::TERRAIN_OBJECTS;
use graphics::coords_to_tile;
use pathfinding::{astar::astar, utils::absdiff};
use shaders::Position;

fn tiles(p: &Point2<usize>) -> Vec<(Point2<usize>, usize)> {
  let neighbours: Vec<Point2<usize>> = vec![Point2::new(p.x - 1, p.y),
                                            Point2::new(p.x - 1, p.y - 1),
                                            Point2::new(p.x, p.y - 1),
                                            Point2::new(p.x + 1, p.y),
                                            Point2::new(p.x + 1, p.y + 1),
                                            Point2::new(p.x, p.y + 1)];

  neighbours.iter()
            .filter(|ref e| !TERRAIN_OBJECTS.contains(&[e.x, e.y]))
            .map(|p| (Point2::new(p.x, p.y), 1))
            .collect()
}

pub fn calc_route(start_point: Position, end_point: Position) -> Option<(Vec<Point2<usize>>, usize)> {
  let start = coords_to_tile(start_point);
  let end = coords_to_tile(end_point);

  astar(&start,
        |p: &Point2<usize>| tiles(p),
        |p: &Point2<usize>| absdiff(p.x, end.x) + absdiff(p.y, end.y),
        |&p| p.x == end.x && p.y == end.y)
}
