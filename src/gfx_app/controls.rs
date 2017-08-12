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
  pub fn move_map_left(&mut self) {
    self.tc(TerrainControl::Left)
  }
  pub fn move_map_right(&mut self) {
    self.tc(TerrainControl::Right)
  }
  pub fn move_map_up(&mut self) {
    self.tc(TerrainControl::Up)
  }
  pub fn move_map_down(&mut self) {
    self.tc(TerrainControl::Down)
  }
  pub fn stop_map_x(&mut self) {
    self.tc(TerrainControl::XMoveStop)
  }
  pub fn stop_map_y(&mut self) {
    self.tc(TerrainControl::YMoveStop)
  }
}
