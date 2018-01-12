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
  Walking,
  Firing,
  Still,
  NormalDeath,
  CriticalDeath,
}

impl Display for Stance {
  fn fmt(&self, f: &mut Formatter) -> Result {
    match *self {
      Stance::Walking => write!(f, "Walking"),
      Stance::Firing => write!(f, "Firing"),
      Stance::Still => write!(f, "Still"),
      Stance::NormalDeath => write!(f, "NormalDeath"),
      Stance::CriticalDeath => write!(f, "CriticalDeath"),
    }
  }
}

