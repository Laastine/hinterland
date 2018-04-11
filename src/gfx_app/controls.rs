use audio::Effects;
use character::controls::CharacterControl;
use gfx_app::mouse_controls::MouseControl;
use graphics::camera::CameraControl;
use std::sync::mpsc;

#[derive(Debug, Clone)]
pub struct TilemapControls {
  audio_control: mpsc::Sender<Effects>,
  terrain_control: mpsc::Sender<CameraControl>,
  character_control: mpsc::Sender<CharacterControl>,
  mouse_control: mpsc::Sender<(MouseControl, Option<(f64, f64)>)>,
}

impl TilemapControls {
  pub fn new(atc: mpsc::Sender<Effects>,
             ttc: mpsc::Sender<CameraControl>,
             ctc: mpsc::Sender<CharacterControl>,
             mtc: mpsc::Sender<(MouseControl, Option<(f64, f64)>)>) -> TilemapControls {
    TilemapControls {
      audio_control: atc,
      terrain_control: ttc,
      character_control: ctc,
      mouse_control: mtc,
    }
  }

  fn ac(&mut self, value: Effects) {
    if self.audio_control.send(value).is_err() {
      panic!("Audio controls disconnected");
    }
  }

  fn tc(&mut self, value: CameraControl) {
    if self.terrain_control.send(value).is_err() {
      panic!("Terrain controls disconnected");
    }
  }
  fn cc(&mut self, value: CharacterControl) {
    if self.character_control.send(value).is_err() {
      panic!("Character controls disconnected");
    }
  }
  fn mc(&mut self, contol_value: MouseControl, value: Option<(f64, f64)>) {
    if self.mouse_control.send((contol_value, value)).is_err() {
      panic!("Mouse controls disconnected")
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
  pub fn ctrl_pressed(&mut self) {
    self.cc(CharacterControl::CtrlPressed)
  }
  pub fn ctrl_released(&mut self) {
    self.cc(CharacterControl::CtrlReleased)
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

  pub fn mouse_left_click(&mut self, mouse_pos: Option<(f64, f64)>) {
    self.mc(MouseControl::LeftClick, mouse_pos);
    match mouse_pos {
      Some(_) => self.ac(Effects::PistolFire),
      _ => self.ac(Effects::None),
    }
  }
  pub fn mouse_right_click(&mut self, mouse_pos: Option<(f64, f64)>) { self.mc(MouseControl::RightClick, mouse_pos) }
}
