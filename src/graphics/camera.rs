use crossbeam_channel as channel;
use specs::prelude::WriteStorage;

use crate::game::constants::VIEW_DISTANCE;
use crate::shaders::Position;

#[derive(Clone)]
pub struct CameraInputState {
  pub distance: f32,
  pub movement: Position,
}

impl CameraInputState {
  pub fn new() -> CameraInputState {
    CameraInputState {
      distance: VIEW_DISTANCE,
      movement: Position::origin(),
    }
  }
}

impl Default for CameraInputState {
  fn default() -> CameraInputState {
    CameraInputState::new()
  }
}

impl specs::prelude::Component for CameraInputState {
  type Storage = specs::storage::HashMapStorage<CameraInputState>;
}

pub enum CameraControl {
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

pub struct CameraControlSystem {
  queue: channel::Receiver<CameraControl>,
  zoom_level: Option<f32>,
}

impl CameraControlSystem {
  pub fn new() -> (CameraControlSystem, channel::Sender<CameraControl>) {
    let (tx, rx) = channel::unbounded();
    (CameraControlSystem {
      queue: rx,
      zoom_level: None,
    }, tx)
  }
}

impl<'a> specs::prelude::System<'a> for CameraControlSystem {
  type SystemData = WriteStorage<'a, CameraInputState>;
  fn run(&mut self, mut map_input: Self::SystemData) {
    use specs::join::Join;

    while let Ok(control) = self.queue.try_recv() {
      match control {
        CameraControl::ZoomIn => self.zoom_level = Some(2.0),
        CameraControl::ZoomOut => self.zoom_level = Some(-2.0),
        CameraControl::ZoomStop => self.zoom_level = None,
        _ => (),
      }
    }
    if let Some(zoom) = self.zoom_level {
      for m in (&mut map_input).join() {
        if m.distance > 200.0 && zoom < 0.0 || m.distance < 600.0 && zoom > 0.0 {
          m.distance += zoom;
        }
      }
    }
  }
}
