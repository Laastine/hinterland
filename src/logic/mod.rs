use std::cmp;
use logic::gridnode::GridNode;

mod gridnode;

#[derive(Clone)]
pub struct Node {
  pub x: i32,
  pub y: i32,
}

impl PartialEq for Node {
  fn eq(&self, other: &Node) -> bool {
    self.x == other.x && self.y == other.y
  }

  fn ne(&self, other: &Node) -> bool {
    self.x != other.x || self.y != other.y
  }
}

pub struct Edge {
  pub node: Node,
  pub cost: i32,
}

fn manhattan(a: Node, b: Node) -> i32 {
  let x = a.x - b.x;
  let y = a.y - b.y;
  x.abs() + y.abs()
}

fn diagonal(a: Node, b: Node) -> i32 {
  let D = 1;
  let D2 = (2 as f64).sqrt() as i32;
  let d1 = b.x - a.x;
  let d2 = b.x - a.y;
  let d1_abs = d1.abs();
  let d2_abs = d2.abs();
  (D * (d1_abs + d2_abs)) + (D2 - 2 * D) * cmp::min(d1_abs, d2_abs)
}

pub fn search(adj_list: &Vec<Vec<Edge>>, start: Node, goal: Node) -> Option<i32> {
  None
}

pub fn clean_node(node: GridNode) -> GridNode {
  GridNode { f: 0, g: 0, h: 0, visited: false, closed: false, parent: Box::new(None), weight: 0.0, x: node.x, y: node.y }
}
