use logic::{clean_node};

struct Graph {
  nodes: Vec<i32>,
  grid: Vec<Vec<i32>>,
  dirty_nodes: Vec<GridNode<i32>>
}

trait Init {
  fn init(&self) -> ();
}

impl Init for Graph {
  fn init(&self) -> () {
    let grid: Vec<Vec<i32>> = vec![vec![i32]];
  }
}

trait Neighbors {
  fn get_neighbors(&self) -> Vec<GridNode>;
}

trait Dirty {
  fn mark_dirty(&self, node: GridNode) -> ();
  fn clean_dirty(&self) -> ();
}

impl Dirty for Graph {
  fn mark_dirty(&self, node: GridNode) {
    self.dirty_nodes.push(node);
  }
  fn clean_dirty(&self) -> () {
    self.dirty_nodes.into_iter().foreach(|x| {

    });
    self.dirty_nodes = Vec::with_capacity(256);
  }
}

impl Neighbors for Graph {
  fn get_neighbors(&self, node: GridNode) {
    let ret: Vec<GridNode> = Vec::with_capacity(256);
    let x: i32 = node.x;
    let y: i32 = node.y;
    let grid = self.grid;

    if grid[x - 1] && grid[x - 1][y] {
      // 315
      ret.push(grid[x - 1][y])
    } else if grid[x + 1] && grid[x + 1][y] {
      // 135
      ret.push(grid[x + 1][y])
    } else if grid[x] && grid[x][y - 1] {
      // 225
      ret.push(grid[x][y - 1])
    } else if grid[x] && grid[x][y + 1] {
      // 45
      ret.push(grid[x][y + 1])
    }
    ret
  }
}

impl ToString for Neighbors {
  fn to_string(&self) -> String {
    let graph_string: Vec<String> = Vec::with_capacity(256);
    let nodes = self.grid;
    nodes.iter().foreach(|x| {
      let rowDebug: Vec<i32> = Vec::with_capacity(nodes.len());
      let row = nodes[x];
      row.iter().foreach(|y| {
        rowDebug.push(row[y].weight)
      });
      graph_string.push(rowDebug.join(" "))
    });
    graph_string.join("\n")
  }
}

fn join(&self: String, separator: &str) {
  for x in foo {
    write!(self, "{}{},", x, separator).unwrap();
  }
  self.pop()
}
