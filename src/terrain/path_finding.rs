use cgmath::Point2;
use game::constants::{TERRAIN_OBJECTS, TILES_PCS_H, TILES_PCS_W};
use game::get_rand_from_range;
use graphics::coords_to_tile_offset;
use pathfinding::{directed::astar::astar, utils::absdiff};
use shaders::Position;

fn neighbours<'c>(curr_pos: &'c Point2<i32>, impassable_tiles: &[[usize; 2]], neighbour_tiles: &'c mut Vec<Point2<i32>>) -> Vec<&'c Point2<i32>> {
  neighbour_tiles.push(Point2::new(curr_pos.x - 1, curr_pos.y));
  neighbour_tiles.push(Point2::new(curr_pos.x - 1, curr_pos.y - 1));
  neighbour_tiles.push(Point2::new(curr_pos.x, curr_pos.y - 1));
  neighbour_tiles.push(Point2::new(curr_pos.x + 1, curr_pos.y));
  neighbour_tiles.push(Point2::new(curr_pos.x + 1, curr_pos.y + 1));
  neighbour_tiles.push(Point2::new(curr_pos.x, curr_pos.y + 1));
  neighbour_tiles.push(Point2::new(curr_pos.x - 1, curr_pos.y + 1));
  neighbour_tiles.push(Point2::new(curr_pos.x + 1, curr_pos.y - 1));

  neighbour_tiles
    .iter()
    .filter(|ref e| e.x >= 0 && e.x < TILES_PCS_W as i32 && e.y >= 0 && e.y < TILES_PCS_H as i32)
    .filter(|ref e| !impassable_tiles.contains(&[e.x as usize, e.y as usize]))
    .collect()
}

fn tiles<'c>(p: &'c Point2<i32>, impassable_tiles: &[[usize; 2]]) -> Vec<(Point2<i32>, i32)> {
  neighbours(&p, &impassable_tiles, &mut vec![]).iter()
    .map(|p| (**p, 1))
    .collect()
}

fn find_next_best_endpoint<'c>(end_point: &'c Point2<i32>, impassable_tiles: &[[usize; 2]], neighbour_tiles: &'c mut Vec<Point2<i32>>) -> &'c Point2<i32> {
  if impassable_tiles.iter().any(|e| e[0] == end_point.x as usize && e[1] == end_point.y as usize) {
    neighbours(&end_point, &impassable_tiles, neighbour_tiles)[0]
  } else {
    &end_point
  }
}

pub fn calc_route(start_point: Position, end_point: Position, offset: (i32, i32), impassable_tiles: &[[usize; 2]]) -> Option<(Vec<Point2<i32>>, i32)> {
  let mut neighbour_tiles = vec![];
  let end_point_with_offset = coords_to_tile_offset(end_point);

  let start_tile = coords_to_tile_offset(start_point);
  let start = Point2::new(start_tile.x + offset.0, start_tile.y + offset.1);
  let end = find_next_best_endpoint(&end_point_with_offset, &impassable_tiles, &mut neighbour_tiles);

  astar(&start,
        |p: &Point2<i32>| tiles(p, &impassable_tiles),
        |p: &Point2<i32>| absdiff(p.x, end.x) + absdiff(p.y, end.y),
        |p: &Point2<i32>| p.x == end.x && p.y == end.y)
}

pub fn calc_next_movement(start_point: Position, end_point: Position, offset: (i32, i32)) -> i32 {
  let next_step: Point2<i32> = calc_route(start_point, end_point, offset, &TERRAIN_OBJECTS)
    .map_or_else(|| Point2::new(0, 0),
                 |(route, _)| {
                   if route.len() > 1 {
                     Point2::new(route[1].x, route[1].y)
                   } else {
                     Point2::new(route[0].x, route[0].y)
                   }
                 });

  let start = coords_to_tile_offset(start_point);
  let diff: (i32, i32) = (next_step.x - start.x, next_step.y - start.y);

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
