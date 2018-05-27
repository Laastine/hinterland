#[test]
fn path_finding_test() {
  use cgmath::Point2;
  use shaders::Position;
  use terrain::path_finding::calc_route;

  let impassable_tiles = vec![[1, 1], [2, 2], [2, 3], [2, 4], [2, 5], [3, 0], [3, 5], [4, 0], [4, 2], [4, 3], [4, 4], [4, 5]];

  let expected_result: Vec<Point2<i32>> = vec![[0, 0], [1, 0], [2, 1], [3, 2], [4, 1], [5, 2], [5, 3], [5, 4], [5, 5]]
    .iter()
    .map(|e| Point2::new(e[0], e[1]))
    .collect();

  assert_eq!(expected_result,
             calc_route(Position::new([0.0, -1500.0]), Position::new([0.0, -1500.0 + 5.0 * 46.0]), &impassable_tiles)
               .map_or_else(|| vec![],
                            |(f, _)| f),
             "path_finding_test");
}
