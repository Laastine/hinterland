use std::sync::mpsc;
use terrain::controls::{TerrainControl};

#[derive(Debug,Clone)]
pub struct TilemapControls {
  terrain_control: mpsc::Sender<TerrainControl>
}

impl TilemapControls {
  pub fn new(tc: mpsc::Sender<TerrainControl>) -> TilemapControls {
    TilemapControls {
      terrain_control: tc
    }
  }

  fn tc(&mut self, value: TerrainControl) {
    if self.terrain_control.send(value).is_err() {
      println!("Controls disconnected");
    }
  }
  pub fn zoom_in(&mut self) {
    self.tc(TerrainControl::ZoomIn)
  }
  pub fn zoom_out(&mut self) {
    self.tc(TerrainControl::ZoomOut)
  }
  pub fn zoom_stop(&mut self) {
    self.tc(TerrainControl::ZoomStop)
  }
}
