use audio::Effects;
use character::controls::CharacterControl;
use crossbeam_channel as channel;
use gfx_app::mouse_controls::MouseControl;
use graphics::camera::CameraControl;

pub enum Control {
  Plus,
  Negative,
  Released,
}

#[derive(Debug, Clone)]
pub struct TilemapControls {
  audio_control: channel::Sender<Effects>,
  terrain_control: channel::Sender<CameraControl>,
  character_control: channel::Sender<CharacterControl>,
  mouse_control: channel::Sender<(MouseControl, Option<(f64, f64)>)>,
}

impl TilemapControls {
  pub fn new(atc: channel::Sender<Effects>,
             ttc: channel::Sender<CameraControl>,
             ctc: channel::Sender<CharacterControl>,
             mtc: channel::Sender<(MouseControl, Option<(f64, f64)>)>) -> TilemapControls {
    TilemapControls {
      audio_control: atc,
      terrain_control: ttc,
      character_control: ctc,
      mouse_control: mtc,
    }
  }

  fn audio(&mut self, value: Effects) {
    self.audio_control.send(value);
  }

  fn terrain(&mut self, value: CameraControl) {
    self.terrain_control.send(value);
  }
  fn character(&mut self, value: CharacterControl) {
    self.character_control.send(value);
  }
  fn mouse(&mut self, contol_value: MouseControl, value: Option<(f64, f64)>) {
    self.mouse_control.send((contol_value, value));
  }

  pub fn zoom(&mut self, control: Control) {
    match control {
      Control::Plus => self.terrain(CameraControl::ZoomIn),
      Control::Negative => self.terrain(CameraControl::ZoomOut),
      Control::Released => self.terrain(CameraControl::ZoomStop),
    }
  }

  pub fn ctrl_pressed(&mut self, is_ctrl: bool) {
    match is_ctrl {
      true => self.character(CharacterControl::CtrlPressed),
      false => self.character(CharacterControl::CtrlReleased),
    }
  }

  pub fn move_character(&mut self, character_control: CharacterControl) {
    self.character(character_control)
  }

  pub fn reload_weapon(&mut self, is_relodading: bool) {
    match is_relodading {
      true => self.character(CharacterControl::ReloadPressed),
      false => self.character(CharacterControl::ReloadReleased),
    }
  }

  pub fn mouse_left_click(&mut self, mouse_pos: Option<(f64, f64)>) {
    self.mouse(MouseControl::LeftClick, mouse_pos);
    match mouse_pos {
      Some(_) => self.audio(Effects::PistolFire),
      _ => self.audio(Effects::None),
    }
  }
  pub fn mouse_right_click(&mut self, mouse_pos: Option<(f64, f64)>) {
    self.mouse(MouseControl::RightClick, mouse_pos)
  }
}
