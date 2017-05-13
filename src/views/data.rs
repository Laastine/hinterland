#[derive(Clone)]
pub struct InputState {
  pub distance: f32,
  pub x_pos: f32,
  pub y_pos: f32,
  pub move_amt: f32,
}

pub enum Event {
  EventUp(bool),
  EventDown(bool),
  EventLeft(bool),
  EventRight(bool),
}

pub struct MapControls {
  pub map_up: bool,
  pub map_down: bool,
  pub map_left: bool,
  pub map_right: bool,
}

impl MapControls {
  pub fn new() -> MapControls {
    MapControls {
      map_up: false,
      map_down: false,
      map_left: false,
      map_right: false,
    }
  }
}
