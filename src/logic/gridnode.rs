pub struct GridNode {
  pub x: i32,
  pub y: i32,
  pub weight: f64,
  pub f: i32,
  pub g: i32,
  pub h: i32,
  pub visited: bool,
  pub closed: bool,
  pub parent: Box<Option<GridNode>>,
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
    if from_neighbor.x != self.x && from_neighbor.y != self.y { (self.weight * 1.41421) } else { self.weight }
  }
}

trait Wall {
  fn is_wall(&self) -> bool;
}

impl Wall for GridNode {
  fn is_wall(&self) -> bool {
    self.weight == 0.0
  }
}
