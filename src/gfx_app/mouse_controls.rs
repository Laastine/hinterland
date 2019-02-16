use cgmath::Point2;
use crossbeam_channel as channel;
use specs;
use specs::prelude::{Read, ReadStorage, WriteStorage};

use crate::bullet::bullets::Bullets;
use crate::character::{CharacterDrawable, controls::CharacterInputState};
use crate::graphics::{camera::CameraInputState, dimensions::Dimensions, direction, direction_movement};
use crate::shaders::Position;

type MouseEvent = channel::Sender<(MouseControl, Option<(f64, f64)>)>;

#[derive(Clone)]
pub struct MouseInputState {
  pub mouse_left: Option<Point2<f32>>,
  pub mouse_right: Option<Point2<f32>>,
  pub left_click_point: Option<Point2<f32>>,
  pub right_click_point: Option<Point2<f32>>,
}

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

impl Default for MouseInputState {
  fn default() -> MouseInputState {
    MouseInputState::new()
  }
}

impl specs::prelude::Component for MouseInputState {
  type Storage = specs::storage::VecStorage<MouseInputState>;
}

pub enum MouseControl {
  LeftClick,
  RightClick,
}

pub struct MouseControlSystem {
  queue: channel::Receiver<(MouseControl, Option<(f64, f64)>)>,
}

impl MouseControlSystem {
  pub fn new() -> (MouseControlSystem, MouseEvent) {
    let (tx, rx) = channel::unbounded();
    (MouseControlSystem {
      queue: rx,
    }, tx)
  }
}

impl<'a> specs::prelude::System<'a> for MouseControlSystem {
  type SystemData = (WriteStorage<'a, MouseInputState>,
                     WriteStorage<'a, CharacterDrawable>,
                     ReadStorage<'a, CameraInputState>,
                     ReadStorage<'a, CharacterInputState>,
                     WriteStorage<'a, Bullets>,
                     Read<'a, Dimensions>);

  fn run(&mut self, (mut mouse_input, mut character_drawable, camera, character_input, mut bullets, dim): Self::SystemData) {
    use specs::join::Join;

    while let Ok((control_value, value)) = self.queue.try_recv() {
      match control_value {
        MouseControl::LeftClick => {
          for (mut mi, cd, bs, ca, ci) in (&mut mouse_input, &mut character_drawable, &mut bullets, &camera, &character_input).join() {
            if let Some(val) = value {
              if ci.is_shooting && cd.stats.ammunition > 0 {
                cd.stats.ammunition -= 1;
                let start_point = Point2::new(dim.window_width / 2.0 * dim.hidpi_factor, dim.window_height / 2.0 * dim.hidpi_factor);
                let end_point = Point2::new(val.0 as f32 * dim.hidpi_factor, val.1 as f32 * dim.hidpi_factor);
                mi.left_click_point = Some(end_point);
                let movement_direction = direction_movement(direction(start_point, end_point));
                Bullets::add_bullet(bs, Position::new(-ca.movement.x(), ca.movement.y()), movement_direction);
              }
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
