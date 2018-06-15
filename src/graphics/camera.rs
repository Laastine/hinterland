use crossbeam_channel as channel;
use game::constants::VIEW_DISTANCE;
use specs;
use specs::prelude::WriteStorage;

#[derive(Clone, Debug)]
pub struct CameraInputState {
  pub distance: f32,
  pub x_pos: f32,
  pub y_pos: f32,
}

#[cfg_attr(feature = "cargo-clippy", allow(new_without_default_derive))]
impl CameraInputState {
  pub fn new() -> CameraInputState {
    CameraInputState {
      distance: VIEW_DISTANCE,
      x_pos: 0.0,
      y_pos: 0.0,
    }
  }
}

impl specs::prelude::Component for CameraInputState {
  type Storage = specs::storage::HashMapStorage<CameraInputState>;
}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct CameraControlSystem {
  queue: channel::Receiver<CameraControl>,
  zoom_level: Option<f32>,
  x_move: Option<f32>,
  y_move: Option<f32>,
}

impl CameraControlSystem {
  pub fn new() -> (CameraControlSystem, channel::Sender<CameraControl>) {
    let (tx, rx) = channel::unbounded();
    (CameraControlSystem {
      queue: rx,
      zoom_level: None,
      x_move: None,
      y_move: None,
    }, tx)
  }
}

impl<'a> specs::prelude::System<'a> for CameraControlSystem {
  type SystemData = (WriteStorage<'a, CameraInputState>);
  fn run(&mut self, mut map_input: Self::SystemData) {
    use specs::join::Join;

    while let Some(control) = self.queue.try_recv() {
      match control {
        CameraControl::ZoomIn => self.zoom_level = Some(2.0),
        CameraControl::ZoomOut => self.zoom_level = Some(-2.0),
        CameraControl::ZoomStop => self.zoom_level = None,
        _ => (),
      }
    }
    if let Some(zoom) = self.zoom_level {
      for m in (&mut map_input).join() {
        if m.distance > 200.0 && zoom < 0.0 || m.distance < 1000.0 && zoom > 0.0 {
          m.distance += zoom;
        }
      }
    }
  }
}
