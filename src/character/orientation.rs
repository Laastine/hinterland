use std::fmt::{Display, Formatter, Result};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Orientation {
  Right = 0,
  UpRight = 1,
  Up = 2,
  UpLeft = 3,
  Left = 4,
  DownLeft = 5,
  Down = 6,
  DownRight = 7,
  Still = 8,
}

impl Display for Orientation {
  fn fmt(&self, f: &mut Formatter) -> Result {
    match *self {
      Orientation::Right => write!(f, "0"),
      Orientation::UpRight => write!(f, "1"),
      Orientation::Up => write!(f, "2"),
      Orientation::UpLeft => write!(f, "3"),
      Orientation::Left => write!(f, "4"),
      Orientation::DownLeft => write!(f, "5"),
      Orientation::Down => write!(f, "6"),
      Orientation::DownRight => write!(f, "7"),
      Orientation::Still => write!(f, "8"),
    }
  }
}
