use std::sync::mpsc;
use specs;

#[derive(Clone, Debug)]
pub struct MouseInputState {
  pub mouse_left: Option<(f64, f64)>,
  pub mouse_right: Option<(f64, f64)>,
}

impl MouseInputState {
  pub fn new() -> MouseInputState {
    MouseInputState {
      mouse_left: None,
      mouse_right: None,
    }
  }
}

impl specs::Component for MouseInputState {
  type Storage = specs::HashMapStorage<MouseInputState>;
}

#[derive(Debug)]
pub enum MouseControl {
  LeftClick,
  RightClick,
}

#[derive(Debug)]
pub struct MouseControlSystem {
  queue: mpsc::Receiver<(MouseControl, Option<(f64, f64)>)>,
  left_click_pos: Option<(f64, f64)>,
  right_click_pos: Option<(f64, f64)>,
}

impl MouseControlSystem {
  pub fn new() -> (MouseControlSystem, mpsc::Sender<(MouseControl, Option<(f64, f64)>)>) {
    let (tx, rx) = mpsc::channel();
    (MouseControlSystem {
      queue: rx,
      left_click_pos: None,
      right_click_pos: None,
    }, tx)
  }
}

impl<C> specs::System<C> for MouseControlSystem {
  fn run(&mut self, arg: specs::RunArg, _: C) {
    let _mouse_input = arg.fetch(|w| w.write::<MouseInputState>());

    while let Ok((control_value, value)) = self.queue.try_recv() {
      match control_value {
        MouseControl::LeftClick => println!("left click {:?} {:?}", value, control_value),
        MouseControl::RightClick => println!("right click {:?} {:?}", value, control_value),
      }
    }
  }
}
