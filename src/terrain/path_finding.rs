use cgmath::Point2;
use pathfinding::{directed::astar::astar, utils::absdiff};

use crate::game::constants::{TERRAIN_OBJECTS, TILES_PCS_H, TILES_PCS_W};
use crate::game::get_rand_from_range;
use crate::graphics::coords_to_tile;
use crate::graphics::shaders::Position;

fn neighbours<'c>(curr_pos: Point2<i32>, impassable_tiles: &[[i32; 2]], neighbour_tiles: &'c mut Vec<Point2<i32>>) -> Vec<&'c Point2<i32>> {
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
    .filter(|ref e| !impassable_tiles.contains(&[e.x, e.y]))
    .collect()
}

fn tiles(p: Point2<i32>, impassable_tiles: &[[i32; 2]]) -> Vec<(Point2<i32>, i32)> {
  neighbours(p, &impassable_tiles, &mut vec![])
    .iter()
    .map(|p| (**p, 1))
    .collect()
}

fn find_next_best_endpoint<'c>(end_point: &'c Point2<i32>, impassable_tiles: &[[i32; 2]], neighbour_tiles: &'c mut Vec<Point2<i32>>) -> &'c Point2<i32> {
  if impassable_tiles.iter().any(|e| e[0] == end_point.x && e[1] == end_point.y) {
    neighbours(*end_point, &impassable_tiles, neighbour_tiles)[0]
  } else {
    &end_point
  }
}

pub fn calc_route(start_point: Position, end_point: Position, impassable_tiles: &[[i32; 2]]) -> Option<(Vec<Point2<i32>>, i32)> {
  let mut neighbour_tiles = vec![];
  let end_point_with_offset = coords_to_tile(end_point);

  let start = coords_to_tile(start_point);
  let end = find_next_best_endpoint(&end_point_with_offset, &impassable_tiles, &mut neighbour_tiles);

  astar(&start,
        |p: &Point2<i32>| tiles(*p, &impassable_tiles),
        |p: &Point2<i32>| absdiff(p.x, end.x) + absdiff(p.y, end.y),
        |p: &Point2<i32>| p.x == end.x && p.y == end.y)
}

pub fn calc_next_movement(start_point: Position, end_point: Position) -> i32 {
  let next_step: Point2<i32> = calc_route(start_point, end_point, &TERRAIN_OBJECTS.to_vec())
    .map_or_else(|| Point2::new(0, 0),
                 |(route, ..)| {
                   if route.len() > 1 {
                     route[1]
                   } else {
                     route[0]
                   }
                 });

  let start = coords_to_tile(start_point);
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
