use std::fmt::{Display, Formatter, Result};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Orientation {
  Right,
  UpRight,
  Up,
  UpLeft,
  Left,
  DownLeft,
  Down,
  DownRight,
  Still,
}

impl Display for Orientation {
  fn fmt(&self, f: &mut Formatter) -> Result {
    match *self {
      Orientation::Right => write!(f, "Right"),
      Orientation::UpRight => write!(f, "UpRight"),
      Orientation::Up => write!(f, "Up"),
      Orientation::UpLeft => write!(f, "UpLeft"),
      Orientation::Left => write!(f, "Left"),
      Orientation::DownLeft => write!(f, "DownLeft"),
      Orientation::Down => write!(f, "Down"),
      Orientation::DownRight => write!(f, "DownRight"),
      Orientation::Still => write!(f, "Still"),
    }
  }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Stance {
  Walking = 0,
  Firing = 1,
  Still = 2,
}

impl Display for Stance {
  fn fmt(&self, f: &mut Formatter) -> Result {
    match *self {
      Stance::Walking => write!(f, "0"),
      Stance::Firing => write!(f, "1"),
      Stance::Still => write!(f, "2"),
    }
  }
}

