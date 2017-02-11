use game::gfx::{Sprite};
use game::data::Rectangle;
use game::constants::{CHARACTER_POS_W, CHARACTER_POS_H, CHARACTER_W, CHARACTER_H};

#[derive(Clone, Copy)]
pub enum CharacterFrame {
  Right = 0,
  UpRight = 1,
  Up = 2,
  UpLeft = 3,
  Left = 4,
  DownLeft = 5,
  Down = 6,
  DownRight = 7,
}

pub struct Character {
  pub rect: Rectangle,
  pub sprites: Vec<Sprite>,
  pub frame_delay: f64,
  pub curr_time: f64,
  pub current: CharacterFrame,
  pub heading: CharacterFrame,
  pub move_anim_index: u32,
  pub fire_anim_index: u32
}

impl Character {
  pub fn new(sprites: Vec<Sprite>) -> Character {
    Character {
      rect: Rectangle {
        x: CHARACTER_POS_W,
        y: CHARACTER_POS_H,
        w: CHARACTER_W,
        h: CHARACTER_H,
      },
      sprites: sprites.clone(),
      frame_delay: 0.0,
      curr_time: 0.0,
      current: CharacterFrame::Down,
      heading: CharacterFrame::Down,
      move_anim_index: 0,
      fire_anim_index: 0,
    }
  }
}
