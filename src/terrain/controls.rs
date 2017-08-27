use std::sync::mpsc;
use specs;
use game::constants::{VIEW_DISTANCE};


#[derive(Clone, Debug)]
pub struct TerrainInputState {
  pub distance: f32,
  pub x_pos: f32,
  pub y_pos: f32,
}

impl TerrainInputState {
  pub fn new() -> TerrainInputState {
    TerrainInputState {
      distance: VIEW_DISTANCE,
      x_pos: 0.0,
      y_pos: 0.0,
    }
  }
}

impl specs::Component for TerrainInputState {
  type Storage = specs::HashMapStorage<TerrainInputState>;
}

#[derive(Debug)]
pub enum TerrainControl {
  ZoomOut,
  ZoomIn,
  ZoomStop,
  Left,
  Right,
  Up,
  Down,
  XMoveStop,
  YMoveStop,
}

#[derive(Debug)]
pub struct TerrainControlSystem {
  queue: mpsc::Receiver<TerrainControl>,
  zoom_level: Option<f32>,
  x_move: Option<f32>,
  y_move: Option<f32>,
}

impl TerrainControlSystem {
  pub fn new() -> (TerrainControlSystem, mpsc::Sender<TerrainControl>) {
    let (tx, rx) = mpsc::channel();
    (TerrainControlSystem {
      queue: rx,
      zoom_level: None,
      x_move: None,
      y_move: None,
    }, tx)
  }
}

impl<C> specs::System<C> for TerrainControlSystem {
  fn run(&mut self, arg: specs::RunArg, _: C) {
    use specs::Join;

    let mut map_input = arg.fetch(|w| w.write::<TerrainInputState>());
    while let Ok(control) = self.queue.try_recv() {
      match control {
        TerrainControl::ZoomIn => self.zoom_level = Some(2.0),
        TerrainControl::ZoomOut => self.zoom_level = Some(-2.0),
        TerrainControl::ZoomStop => self.zoom_level = None,
        TerrainControl::Up => self.y_move = Some(-1.0),
        TerrainControl::Down => self.y_move = Some(1.0),
        TerrainControl::YMoveStop => self.y_move = None,
        TerrainControl::Right => self.x_move = Some(1.0),
        TerrainControl::Left => self.x_move = Some(-1.0),
        TerrainControl::XMoveStop => self.x_move = None,
      }
    }
    if let Some(zoom) = self.zoom_level {
      for m in (&mut map_input).join() {
        if m.distance > 200.0 && zoom < 0.0 || m.distance < 1000.0 && zoom > 0.0 {
          m.distance += zoom;
        }
      }
    }
    if let Some(x) = self.x_move {
      for m in (&mut map_input).join() {
        m.x_pos += x;
      }
    }
    if let Some(y) = self.y_move {
      for m in (&mut map_input).join() {
        m.y_pos += y;
      }
    }
  }
}
