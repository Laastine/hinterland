use std::sync::mpsc;
use specs;

#[derive(Clone, Debug)]
pub struct InputState {
  pub distance: f32,
}

impl InputState {
  pub fn new() -> InputState {
    InputState {
      distance: 1000.0,
    }
  }
}

impl specs::Component for InputState {
  type Storage = specs::HashMapStorage<InputState>;
}

#[derive(Debug)]
pub enum TerrainControl {
  ZoomOut,
  ZoomIn,
  ZoomStop,
}

#[derive(Debug)]
pub struct TerrainControlSystem {
  queue: mpsc::Receiver<TerrainControl>,
  zoom_level: Option<f32>,
}

impl TerrainControlSystem {
  pub fn new() -> (TerrainControlSystem, mpsc::Sender<TerrainControl>) {
    let (tx, rx) = mpsc::channel();
    (TerrainControlSystem {
      queue: rx,
      zoom_level: None
    }, tx)
  }
}

impl<C> specs::System<C> for TerrainControlSystem {
  fn run(&mut self, arg: specs::RunArg, _: C) {
    use specs::Join;

    let mut map_input = arg.fetch(|w| w.write::<InputState>());
    while let Ok(control) = self.queue.try_recv() {
      match control {
        TerrainControl::ZoomIn => self.zoom_level = Some(-2.0),
        TerrainControl::ZoomOut => self.zoom_level = Some(2.0),
        TerrainControl::ZoomStop => self.zoom_level = None,
      }
    }
    if let Some(zoom) = self.zoom_level {
      for m in (&mut map_input).join() {
        m.distance += zoom;
      }
    }
  }
}
