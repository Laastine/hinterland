use gfx;
use game::gfx_macros::{pipe, TileMapData};


#[derive(Clone, Debug)]
pub struct Point {
  pub x: f64,
  pub y: f64,
}

#[derive(Clone, Copy, Debug)]
pub enum Orientation {
  Right = 0,
  UpRight = 1,
  Up = 2,
  UpLeft = 3,
  Left = 4,
  DownLeft = 5,
  Down = 6,
  DownRight = 7,
}

impl Display for Orientation {
  fn fmt(&self, f: &mut Formatter) -> Result {
    use views::Orientation::*;
    match *self {
      Right => write!(f, "0"),
      UpRight => write!(f, "1"),
      Up => write!(f, "2"),
      UpLeft => write!(f, "3"),
      Left => write!(f, "4"),
      DownLeft => write!(f, "5"),
      Down => write!(f, "6"),
      DownRight => write!(f, "7"),
    }
  }
}

pub struct Character<R> where R: gfx::Resource {
  pub params: pipe::Data<R>,
  pub slice: gfx::Slice<R>,
  projection: Projection,
  is_projection_dirty: bool,
  tilemap_settings: TilemapSettings,
  is_tilemap_dirty: bool,
  pub data: Vec<TileMapData>,
}
