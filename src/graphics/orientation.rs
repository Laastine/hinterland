use std::fmt::{Display, Formatter, Result};

#[derive(Clone, Copy, PartialEq)]
pub enum Orientation {
  Right,
  UpRight,
  Up,
  UpLeft,
  Left,
  DownLeft,
  Down,
  DownRight,
  Normal,
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
      Orientation::Normal => write!(f, "Normal"),
    }
  }
}

#[derive(Clone, PartialEq)]
pub enum Stance {
  Walking,
  Running,
  Firing,
  Still,
  NormalDeath,
  CriticalDeath,
}

impl Display for Stance {
  fn fmt(&self, f: &mut Formatter) -> Result {
    match *self {
      Stance::Walking => write!(f, "Walking"),
      Stance::Running => write!(f, "Running"),
      Stance::Firing => write!(f, "Firing"),
      Stance::Still => write!(f, "Still"),
      Stance::NormalDeath => write!(f, "NormalDeath"),
      Stance::CriticalDeath => write!(f, "CriticalDeath"),
    }
  }
}

