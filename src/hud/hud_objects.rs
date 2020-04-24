
use crate::game::constants::{CURRENT_AMMO_TEXT, CURRENT_MAGAZINE_TEXT, GAME_VERSION};
use crate::hud::TextDrawable;
use crate::shaders::Position;

pub struct HudObjects {
  pub objects: Vec<TextDrawable>,
}

impl HudObjects {
  pub fn new() -> HudObjects {
    HudObjects {
      objects: vec![
        TextDrawable::new(GAME_VERSION, Position::origin()),
        TextDrawable::new(CURRENT_AMMO_TEXT, Position::new(1.9, -1.9)),
        TextDrawable::new(CURRENT_MAGAZINE_TEXT, Position::new(1.9, -1.94)),
      ]
    }
  }
}

impl specs::prelude::Component for HudObjects {
  type Storage = specs::storage::VecStorage<HudObjects>;
}
