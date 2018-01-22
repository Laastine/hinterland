use cgmath::Point2;
use graphics::{Dimensions, direction, direction_movement};
use graphics::camera::CameraInputState;
use std::sync::mpsc;
use specs;
use specs::Fetch;
use specs::{ReadStorage, WriteStorage};
use bullet::bullets::Bullets;

type MouseEvent = mpsc::Sender<(MouseControl, Option<(f64, f64)>)>;

#[derive(Clone, Debug)]
pub struct MouseInputState {
  pub mouse_left: Option<Point2<f32>>,
  pub mouse_right: Option<Point2<f32>>,
  pub left_click_point: Option<Point2<f32>>,
  pub right_click_point: Option<Point2<f32>>,
}

#[cfg_attr(feature = "cargo-clippy", allow(new_without_default_derive))]
impl MouseInputState {
  pub fn new() -> MouseInputState {
    MouseInputState {
      mouse_left: None,
      mouse_right: None,
      left_click_point: None,
      right_click_point: None,
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
  pub fn new() -> (MouseControlSystem, MouseEvent) {
    let (tx, rx) = mpsc::channel();
    (MouseControlSystem {
      queue: rx,
      left_click_pos: None,
      right_click_pos: None,
    }, tx)
  }
}

impl<'a> specs::System<'a> for MouseControlSystem {
  type SystemData = (WriteStorage<'a, MouseInputState>,
                     ReadStorage<'a, CameraInputState>,
                     WriteStorage<'a, Bullets>,
                     Fetch<'a, Dimensions>);

  fn run(&mut self, (mut mouse_input, camera, mut bullets, dim): Self::SystemData) {
    use specs::Join;

    while let Ok((control_value, value)) = self.queue.try_recv() {
      match control_value {
        MouseControl::LeftClick => {
          for (mi, bs, ca) in (&mut mouse_input, &mut bullets, &camera).join() {
            if let Some(val) = value {
              let start_point =  Point2::new(dim.width as f32 / 2.0, dim.height as f32 / 2.0);
              let end_point = Point2::new(val.0 as f32, val.1 as f32);
              mi.left_click_point = Some(end_point);
              let movement_direction = direction_movement(direction(start_point, end_point));
              Bullets::add_bullet(bs,Point2 {x: -ca.x_pos, y: ca.y_pos}, movement_direction);
            } else {
              mi.left_click_point = None;
            }
          }
        }
        MouseControl::RightClick => {
          for mi in (&mut mouse_input).join() {
            if let Some(val) = value {
              mi.right_click_point = Some(Point2::new(val.0 as f32, val.1 as f32));
            } else {
              mi.right_click_point = None
            }
          }
        }
      }
    }
  }
}
