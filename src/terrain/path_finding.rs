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

pub fn calc_next_movement(start_point: Position, end_point: Position) -> i32 {
  let next_step: Point2<i32> = calc_route(start_point, end_point)
    .map_or_else(|| Point2::new(0 as i32, 0 as i32),
                 |(route, _)| Point2::new(route[1].x as i32, route[1].y as i32));

  let start = coords_to_tile(start_point);

  let diff: (i32, i32) = (start.x as i32 - next_step.x, start.y as i32 - next_step.y);
  match diff {
    (1, 0) => 0,
    (1, 1) => 45,
    (0, 1) => 180,
    (-1, 1) => 315,
    (-1, 0) => 270,
    (-1, -1) => 225,
    (0, -1) => 90,
    (1, -1) => 135,
    _ => 0,
  }
}
