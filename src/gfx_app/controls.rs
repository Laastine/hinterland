use std::sync::mpsc;
use graphics::camera::CameraControl;
use character::controls::CharacterControl;
use gfx_app::mouse_controls::MouseControl;

#[derive(Debug, Clone)]
pub struct TilemapControls {
  terrain_control: mpsc::Sender<CameraControl>,
  character_control: mpsc::Sender<CharacterControl>,
  mouse_control: mpsc::Sender<(MouseControl, Option<(f64, f64)>)>,
}

impl TilemapControls {
  pub fn new(ttc: mpsc::Sender<CameraControl>,
             ctc: mpsc::Sender<CharacterControl>,
             mtc: mpsc::Sender<(MouseControl, Option<(f64, f64)>)>) -> TilemapControls {
    TilemapControls {
      terrain_control: ttc,
      character_control: ctc,
      mouse_control: mtc,
    }
  }

  fn tc(&mut self, value: CameraControl) {
    if self.terrain_control.send(value).is_err() {
      panic!("Controls disconnected");
    }
  }
  fn cc(&mut self, value: CharacterControl) {
    if self.character_control.send(value).is_err() {
      panic!("Controls disconnected");
    }
  }
  fn mc(&mut self, contol_value: MouseControl, value: Option<(f64, f64)>) {
    if self.mouse_control.send((contol_value, value)).is_err() {
      panic!("Controls disconnected")
    }
  }

  pub fn zoom_in(&mut self) {
    self.tc(CameraControl::ZoomIn)
  }
  pub fn zoom_out(&mut self) {
    self.tc(CameraControl::ZoomOut)
  }
  pub fn zoom_stop(&mut self) {
    self.tc(CameraControl::ZoomStop)
  }
  #[allow(dead_code)]
  pub fn move_map_left(&mut self) {
    self.tc(CameraControl::Left)
  }
  #[allow(dead_code)]
  pub fn move_map_right(&mut self) {
    self.tc(CameraControl::Right)
  }
  #[allow(dead_code)]
  pub fn move_map_up(&mut self) {
    self.tc(CameraControl::Up)
  }
  #[allow(dead_code)]
  pub fn move_map_down(&mut self) {
    self.tc(CameraControl::Down)
  }
  #[allow(dead_code)]
  pub fn stop_map_x(&mut self) {
    self.tc(CameraControl::XMoveStop)
  }
  #[allow(dead_code)]
  pub fn stop_map_y(&mut self) {
    self.tc(CameraControl::YMoveStop)
  }

  pub fn move_character_left(&mut self) {
    self.cc(CharacterControl::Left)
  }
  pub fn move_character_right(&mut self) {
    self.cc(CharacterControl::Right)
  }
  pub fn stop_character_x(&mut self) { self.cc(CharacterControl::XMoveStop) }
  pub fn move_character_up(&mut self) {
    self.cc(CharacterControl::Up)
  }
  pub fn move_character_down(&mut self) {
    self.cc(CharacterControl::Down)
  }
  pub fn stop_character_y(&mut self) { self.cc(CharacterControl::YMoveStop) }

  pub fn mouse_left_click(&mut self, mouse_pos: Option<(f64, f64)>) { self.mc(MouseControl::LeftClick, mouse_pos) }
  pub fn mouse_right_click(&mut self, mouse_pos: Option<(f64, f64)>) { self.mc(MouseControl::RightClick, mouse_pos) }
}
