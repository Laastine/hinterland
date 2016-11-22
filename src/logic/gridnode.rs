struct GridNode {
  x: i32,
  y: i32,
  weight: i32,
  f: i32,
  g: i32,
  h: i32,
  visited: bool,
  closed: bool,
  parent: GridNode,
}

impl ToString for GridNode {
  fn to_string(&self) -> String {
    format!("[{} {}]", &self.x, &self.y)
  }
}

trait Cost {
  fn get_cost(&self, from_neighbor: GridNode) -> f64;
}

impl Cost for GridNode {
  fn get_cost(&self, from_neighbor: GridNode) -> f64 {
    if from_neighbor.x != self.x && from_neighbor.y != self.y { self.weight * 1.41421 } else { self.weight }
  }
}

trait Wall {
  fn is_wall(&self) -> bool;
}

impl Wall for GridNode {
  fn is_wall(&self) -> bool {
    self.weight == 0
  }
}
