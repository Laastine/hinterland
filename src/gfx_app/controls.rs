use std::sync::mpsc;
use terrain::controls::TerrainControl;
use character::controls::CharacterControl;

#[derive(Debug, Clone)]
pub struct TilemapControls {
  terrain_control: mpsc::Sender<TerrainControl>,
  character_control: mpsc::Sender<CharacterControl>,
}

impl TilemapControls {
  pub fn new(ttc: mpsc::Sender<TerrainControl>, ctc: mpsc::Sender<CharacterControl>) -> TilemapControls {
    TilemapControls {
      terrain_control: ttc,
      character_control: ctc,
    }
  }

  fn tc(&mut self, value: TerrainControl) {
    if self.terrain_control.send(value).is_err() {
      println!("Controls disconnected");
    }
  }
  fn cc(&mut self, value: CharacterControl) {
    if self.character_control.send(value).is_err() {
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

  pub fn move_character_left(&mut self) {
    self.cc(CharacterControl::Left)
  }
  pub fn move_character_right(&mut self) {
    self.cc(CharacterControl::Right)
  }
  pub fn stop_character_x(&mut self) { self.cc(CharacterControl::XMoveStop)}
  pub fn move_character_up(&mut self) {
    self.cc(CharacterControl::Up)
  }
  pub fn move_character_down(&mut self) {
    self.cc(CharacterControl::Down)
  }
  pub fn stop_character_y(&mut self) { self.cc(CharacterControl::YMoveStop)}
}
