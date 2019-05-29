use crossbeam_channel as channel;

use crate::audio::Effects;
use crate::character::controls::CharacterControl;
use crate::gfx_app::mouse_controls::MouseControl;
use crate::graphics::camera::CameraControl;

pub enum Control {
  Plus,
  Negative,
  Released,
}

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

  pub fn zoom(&mut self, control: &Control) {
    match control {
      Control::Plus => self.terrain_control.send(CameraControl::ZoomIn),
      Control::Negative => self.terrain_control.send(CameraControl::ZoomOut),
      Control::Released => self.terrain_control.send(CameraControl::ZoomStop),
    }.expect("Terrain control update error");
  }

  pub fn ctrl_pressed(&mut self, is_ctrl: bool) {
    if is_ctrl {
      self.character_control.send(CharacterControl::CtrlPressed)
    } else {
      self.character_control.send(CharacterControl::CtrlReleased)
    }.expect("Character Ctrl control update error");
  }

  pub fn move_character(&mut self, character_control: CharacterControl) {
    self.character_control.send(character_control).expect("Character move control update error");
  }

  pub fn reload_weapon(&mut self, is_reloading: bool) {
    if is_reloading {
      self.character_control.send(CharacterControl::ReloadPressed)
    } else {
      self.character_control.send(CharacterControl::ReloadReleased)
    }.expect("Character reload weapon control update error");
  }

  pub fn mouse_left_click(&mut self, mouse_pos: Option<(f64, f64)>) {
    self.mouse_control.send((MouseControl::LeftClick, mouse_pos)).expect("Mouse control shoot update error");
    match mouse_pos {
      Some(_) => self.audio_control.send(Effects::PistolFire),
      _ => self.audio_control.send(Effects::None),
    }.expect("Audio control update error");
  }
}
