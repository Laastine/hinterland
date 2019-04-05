use crossbeam_channel as channel;

use crate::graphics::camera::CameraControl;

pub enum Control {
  Plus,
  Negative,
  Released,
}

pub struct TilemapControls {
  terrain_control: channel::Sender<CameraControl>,
}

impl TilemapControls {
  pub fn new(
             ttc: channel::Sender<CameraControl>) -> TilemapControls {
    TilemapControls {
      terrain_control: ttc,
    }
  }

  pub fn zoom(&mut self, control: &Control) {
    let _ = match control {
      Control::Plus => self.terrain_control.send(CameraControl::ZoomIn),
      Control::Negative => self.terrain_control.send(CameraControl::ZoomOut),
      Control::Released => self.terrain_control.send(CameraControl::ZoomStop),
    };
  }
}
