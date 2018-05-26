use cgmath::Point2;
use game::get_rand_from_range;
use game::constants::TERRAIN_OBJECTS;
use graphics::coords_to_tile_offset;
use pathfinding::{astar::astar, utils::absdiff};
use shaders::Position;

fn tiles(p: &Point2<i32>) -> Vec<(Point2<i32>, i32)> {
  let e = Point2::new(p.x as i32, p.y as i32);
  let neighbours: Vec<Point2<i32>> = vec![Point2::new(e.x - 1, e.y),
                                          Point2::new(e.x - 1, e.y - 1),
                                          Point2::new(e.x, e.y - 1),
                                          Point2::new(e.x + 1, e.y),
                                          Point2::new(e.x + 1, e.y + 1),
                                          Point2::new(e.x, e.y + 1),
                                          Point2::new(e.x - 1, e.y + 1),
                                          Point2::new(e.x + 1, e.y - 1)
  ];

  neighbours.iter()
            .filter(|ref e| !TERRAIN_OBJECTS.contains(&[e.x as usize, e.y as usize]))
            .map(|p| (Point2::new(p.x, p.y), 1))
            .collect()
}

fn calc_route(start_point: Position, end_point: Position) -> Option<(Vec<Point2<i32>>, i32)> {
  let start = coords_to_tile_offset(start_point);
  let end = coords_to_tile_offset(end_point);

  astar(&start,
        |p: &Point2<i32>| tiles(p),
        |p: &Point2<i32>| absdiff(p.x, end.x) + absdiff(p.y, end.y),
        |p: &Point2<i32>| p.x == end.x && p.y == end.y)
}

pub fn calc_next_movement(start_point: Position, end_point: Position) -> i32 {
  let next_step: Point2<i32> = calc_route(start_point, end_point)
    .map_or_else(|| Point2::new(0 as i32, 0 as i32),
                 |(route, _)| {
                   if route.len() > 1 {
                     Point2::new(route[1].x as i32, route[1].y as i32)
                   } else {
                     Point2::new(route[0].x as i32, route[0].y as i32)
                   }
                 });

  let start = coords_to_tile_offset(start_point);

  let diff: (i32, i32) = (start.x - next_step.x, start.y - next_step.y);
  match diff {
    (1, 0) => 315,
    (1, 1) => 270,
    (0, 1) => 225,
    (-1, 1) => 180,
    (-1, 0) => 135,
    (-1, -1) => 90,
    (0, -1) => 45,
    (1, -1) => 0,
    _ => get_rand_from_range(0, 359),
  }
}
