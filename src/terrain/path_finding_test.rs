use game::constants::Y_OFFSET_256;

#[allow(dead_code)]
const IMPASSABLE_TILES: [[usize; 2]; 12] = [[1, 1], [2, 2], [2, 3], [2, 4], [2, 5], [3, 0], [3, 5], [4, 0], [4, 2], [4, 3], [4, 4], [4, 5]];

#[test]
fn path_finding_test_1() {
  use cgmath::Point2;
  use shaders::Position;
  use terrain::path_finding::calc_route;

  let expected_result_1: Vec<Point2<i32>> = vec![[0, 0], [1, 0], [2, 1], [3, 2], [4, 1], [5, 2], [5, 3], [5, 4], [5, 5]]
    .iter()
    .map(|e| Point2::new(e[0], e[1]))
    .collect();

  assert_eq!(expected_result_1,
             calc_route(Position::new(0.0, -Y_OFFSET_256), Position::new(0.0, -Y_OFFSET_256 + 5.0 * 46.0), &IMPASSABLE_TILES.to_vec())
               .map_or_else(|| vec![],
                            |(f, _)| f),
             "path_finding_test_1");
}

#[test]
fn path_finding_test_2() {
  use cgmath::Point2;
  use shaders::Position;
  use terrain::path_finding::calc_route;

  let expected_result: Vec<Point2<i32>> = vec![[3, 3], [3, 2], [4, 1], [5, 2], [5, 3], [5, 4], [5, 5]]
    .iter()
    .map(|e| Point2::new(e[0], e[1]))
    .collect();

  assert_eq!(expected_result,
             calc_route(Position::new(0.0, -Y_OFFSET_256 + 3.0 * 46.0), Position::new(0.0, -Y_OFFSET_256 + 5.0 * 46.0), &IMPASSABLE_TILES.to_vec())
               .map_or_else(|| vec![],
                            |(f, _)| f),
             "path_finding_test_2");
}
