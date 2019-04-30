use crossbeam_channel as channel;

use crate::character::controls::CharacterControl;
use crate::graphics::camera::CameraControl;

pub enum Control {
  Plus,
  Negative,
  Released,
}

pub struct TilemapControls {
  camera_control: channel::Sender<CameraControl>,
  character_control: channel::Sender<CharacterControl>,
}

impl TilemapControls {
  pub fn new(ttc: channel::Sender<CameraControl>,
             ctc: channel::Sender<CharacterControl>) -> TilemapControls {
    TilemapControls {
      camera_control: ttc,
      character_control: ctc,
    }
  }

  pub fn zoom(&mut self, control: &Control) {
    let _ = match control {
      Control::Plus => self.camera_control.send(CameraControl::ZoomIn),
      Control::Negative => self.camera_control.send(CameraControl::ZoomOut),
      Control::Released => self.camera_control.send(CameraControl::ZoomStop),
    };
  }

  pub fn ctrl_pressed(&mut self, is_ctrl: bool) {
    let _ = if is_ctrl {
      self.character_control.send(CharacterControl::CtrlPressed)
    } else {
      self.character_control.send(CharacterControl::CtrlReleased)
    };
  }

  pub fn move_character(&mut self, character_control: CharacterControl) {
    let _ = self.character_control.send(character_control);
  }

  pub fn reload_weapon(&mut self, is_reloading: bool) {
    let _ = if is_reloading {
      self.character_control.send(CharacterControl::ReloadPressed)
    } else {
      self.character_control.send(CharacterControl::ReloadReleased)
    };
  }
}
