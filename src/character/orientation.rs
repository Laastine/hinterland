use std::fmt::{Display, Formatter, Result};

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
