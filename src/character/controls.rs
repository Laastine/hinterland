use std::sync::mpsc;
use specs;

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

impl<C> specs::System<C> for CharacterControlSystem {
  fn run(&mut self, arg: specs::RunArg, _: C) {
    use specs::Join;

    let mut character_input = arg.fetch(|w| w.write::<CharacterInputState>());
    while let Ok(control) = self.queue.try_recv() {
      match control {
        CharacterControl::Up => self.y_move = Some(0.8),
        CharacterControl::Down => self.y_move = Some(-0.8),
        CharacterControl::YMoveStop => self.y_move = None,
        CharacterControl::Right => self.x_move = Some(0.8),
        CharacterControl::Left => self.x_move = Some(-0.8),
        CharacterControl::XMoveStop => self.x_move = None,
      }
    }
    if let Some(x) = self.x_move {
      if let Some(y) = self.y_move {
        for m in (&mut character_input).join() {
          m.x_movement += x / 1.5;
          m.y_movement += y / 1.5;
        }
      }
    }
    if let Some(x) = self.x_move {
      if self.y_move == None {
        for m in (&mut character_input).join() {
          m.x_movement += x;
        }
      }
    }
    if let Some(y) = self.y_move {
      if self.x_move == None {
        for m in (&mut character_input).join() {
          m.y_movement += y;
        }
      }
    }
  }
}
