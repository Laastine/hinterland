use character::CharacterDrawable;
use crossbeam_channel as channel;
use game::constants::{CHARACTER_X_SPEED, CHARACTER_Y_SPEED};
use graphics::{camera::CameraInputState, can_move_to_tile, DeltaTime, orientation::Orientation};
use shaders::Position;
use specs;
use specs::prelude::{Read, ReadStorage, WriteStorage};

#[derive(Clone, Debug)]
pub struct CharacterInputState {
  pub x_movement: f32,
  pub y_movement: f32,
  pub orientation: Orientation,
  pub is_colliding: bool,
  pub is_shooting: bool,
}

impl CharacterInputState {
  pub fn new() -> CharacterInputState {
    CharacterInputState {
      x_movement: 0.0,
      y_movement: 0.0,
      orientation: Orientation::Still,
      is_colliding: false,
      is_shooting: false,
    }
  }

  pub fn update(&mut self, camera: &mut CameraInputState, css: &CharacterControlSystem) {
    if css.y_move.is_none() && css.x_move.is_none() {
        self.orientation = Orientation::Still;
    } else if css.x_move.is_none() {
      if let Some(y) = css.y_move {
        let vert_move = Position::new(self.x_movement, self.y_movement + y);
        if !self.is_colliding || can_move_to_tile(vert_move) {
          self.y_movement += y;
          camera.y_pos -= y;
          self.orientation = match y {
            y if y < 0.0 => Orientation::Up,
            y if y > 0.0 => Orientation::Down,
            _ => Orientation::Still,
          };
        }
      }
    } else if let Some(x) = css.x_move {
      let horizontal_move  = Position::new(self.x_movement + x, self.y_movement);
      if let Some(y) = css.y_move {
        let diag_move = Position::new(self.x_movement + x, self.y_movement + y);
        if !self.is_colliding || can_move_to_tile(diag_move) {
          self.x_movement += x / 1.5;
          self.y_movement += y / 1.5;
          camera.x_pos += x / 1.5;
          camera.y_pos -= y / 1.5;

          self.orientation = match (x, y) {
            (x, y) if x > 0.0 && y > 0.0 => Orientation::DownLeft,
            (x, y) if x > 0.0 && y < 0.0 => Orientation::UpLeft,
            (x, y) if x < 0.0 && y > 0.0 => Orientation::DownRight,
            (x, y) if x < 0.0 && y < 0.0 => Orientation::UpRight,
            _ => Orientation::Still,
          };
        }
      } else if css.y_move.is_none() && !self.is_colliding || can_move_to_tile(horizontal_move) {
        self.x_movement += x;
        camera.x_pos += x;
        self.orientation = match x {
          x if x < 0.0 => Orientation::Right,
          x if x > 0.0 => Orientation::Left,
          _ => Orientation::Still,
        };
      }
    }
    self.is_shooting = css.is_ctrl_pressed;
  }
}

impl Default for CharacterInputState {
  fn default() -> Self {
    CharacterInputState::new()
  }
}

impl specs::prelude::Component for CharacterInputState {
  type Storage = specs::storage::DenseVecStorage<CharacterInputState>;
}

#[derive(Debug)]
pub enum CharacterControl {
  Left,
  Right,
  Up,
  Down,
  XMoveStop,
  YMoveStop,
  CtrlPressed,
  CtrlReleased,
}

#[derive(Debug)]
pub struct CharacterControlSystem {
  queue: channel::Receiver<CharacterControl>,
  x_move: Option<f32>,
  y_move: Option<f32>,
  cool_down: f64,
  is_ctrl_pressed: bool,
}

impl CharacterControlSystem {
  pub fn new() -> (CharacterControlSystem, channel::Sender<CharacterControl>) {
    let (tx, rx) = channel::unbounded();
    (CharacterControlSystem {
      queue: rx,
      x_move: None,
      y_move: None,
      cool_down: 1.0,
      is_ctrl_pressed: false,
    }, tx)
  }
}

impl<'a> specs::prelude::System<'a> for CharacterControlSystem {
  type SystemData = (WriteStorage<'a, CharacterInputState>,
                     ReadStorage<'a, CharacterDrawable>,
                     WriteStorage<'a, CameraInputState>,
                     Read<'a, DeltaTime>);

  fn run(&mut self, (mut character_input, character, mut camera_input, d): Self::SystemData) {
    use specs::join::Join;
    use graphics::orientation::Stance;
    let delta = d.0;

    if self.cool_down == 0.0 {
      self.cool_down += 0.1;
    } else {
      self.cool_down = (self.cool_down - delta).max(0.0);
      while let Some(control) = self.queue.try_recv() {
        match control {
          CharacterControl::Up => self.y_move = Some(-CHARACTER_Y_SPEED),
          CharacterControl::Down => self.y_move = Some(CHARACTER_Y_SPEED),
          CharacterControl::YMoveStop => self.y_move = None,
          CharacterControl::Right => self.x_move = Some(-CHARACTER_X_SPEED),
          CharacterControl::Left => self.x_move = Some(CHARACTER_X_SPEED),
          CharacterControl::XMoveStop => self.x_move = None,
          CharacterControl::CtrlPressed => self.is_ctrl_pressed = true,
          CharacterControl::CtrlReleased => self.is_ctrl_pressed = false,
        }
      }

      for (ci, c, camera) in (&mut character_input, &character, &mut camera_input).join() {
        if c.stance != Stance::NormalDeath {
          ci.update(camera, self);
        }
      }
    }
  }
}
