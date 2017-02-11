use game::gfx::{Sprite};
use game::data::Rectangle;
use game::constants::{CHARACTER_POS_W, CHARACTER_POS_H, CHARACTER_W, CHARACTER_H};
use views::{Orientation};

pub struct Character {
  pub rect: Rectangle,
  pub sprites: Vec<Sprite>,
  pub current: Orientation,
  pub heading: Orientation,
  pub move_anim_index: u32,
  pub fire_anim_index: u32,
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
      current: Orientation::Down,
      heading: Orientation::Down,
      move_anim_index: 0,
      fire_anim_index: 0,
    }
  }
}
