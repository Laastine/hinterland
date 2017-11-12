use std::sync::mpsc;
use specs;
use specs::{ReadStorage, WriteStorage};
use gfx_app::mouse_controls::MouseInputState;
use graphics::camera::CameraInputState;

#[derive(Clone, Debug)]
pub struct CharacterInputState {
  pub x_movement: f32,
  pub y_movement: f32,
}

impl CharacterInputState {
  pub fn new() -> CharacterInputState {
    CharacterInputState {
      x_movement: 0.0,
      y_movement: 0.0,
    }
  }
}

impl specs::Component for CharacterInputState {
  type Storage = specs::HashMapStorage<CharacterInputState>;
}

#[derive(Debug)]
pub enum CharacterControl {
  Left,
  Right,
  Up,
  Down,
  XMoveStop,
  YMoveStop,
}

#[derive(Debug)]
pub struct CharacterControlSystem {
  queue: mpsc::Receiver<CharacterControl>,
  x_move: Option<f32>,
  y_move: Option<f32>,
}

impl CharacterControlSystem {
  pub fn new() -> (CharacterControlSystem, mpsc::Sender<CharacterControl>) {
    let (tx, rx) = mpsc::channel();
    (CharacterControlSystem {
      queue: rx,
      x_move: None,
      y_move: None,
    }, tx)
  }
}

impl<'a> specs::System<'a> for CharacterControlSystem {
  type SystemData = (WriteStorage<'a, CharacterInputState>, ReadStorage<'a, MouseInputState>, WriteStorage<'a, CameraInputState>);

  fn run(&mut self, (mut character_input, mouse_input, mut camera_input): Self::SystemData) {
    use specs::Join;

    while let Ok(control) = self.queue.try_recv() {
      match control {
        CharacterControl::Up => self.y_move = Some(0.9),
        CharacterControl::Down => self.y_move = Some(-0.9),
        CharacterControl::YMoveStop => self.y_move = None,
        CharacterControl::Right => self.x_move = Some(1.0),
        CharacterControl::Left => self.x_move = Some(-1.0),
        CharacterControl::XMoveStop => self.x_move = None,
      }
    }
    if let Some(x) = self.x_move {
      if let Some(y) = self.y_move {
        for (ci, mi, camera) in (&mut character_input, &mouse_input, &mut camera_input).join() {
          if mi.left_click_point.is_none() {
            ci.x_movement += x / 1.5;
            ci.y_movement += y / 1.5;
            camera.x_pos += x / 1.5 / 1.9;
            camera.y_pos -= y / 1.5 / 3.471;
          }
        }
      }
    }
    if let Some(x) = self.x_move {
      if self.y_move == None {
        for (ci, mi, camera) in (&mut character_input, &mouse_input, &mut camera_input).join() {
          if mi.left_click_point.is_none() {
            ci.x_movement += x;
            camera.x_pos += x / 1.95;
          }
        }
      }
    }
    if let Some(y) = self.y_move {
      if self.x_move == None {
        for (ci, mi, camera) in (&mut character_input, &mouse_input, &mut camera_input).join() {
          if mi.left_click_point.is_none() {
            ci.y_movement += y;
            camera.y_pos -= y / 3.471;
          }
        }
      }
    }
  }
}
