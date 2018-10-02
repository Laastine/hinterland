use game::constants::{CURRENT_AMMO_TEXT, CURRENT_MAGAZINE_TEXT, VERSION_NUMBER_TEXT};
use hud::TextDrawable;
use shaders::Position;
use specs;

#[derive(Debug, Clone)]
pub struct HudObjects {
  pub objects: Vec<TextDrawable>,
}

impl HudObjects {
  pub fn new() -> HudObjects {
    HudObjects {
      objects: vec![
        TextDrawable::new(VERSION_NUMBER_TEXT, Position::origin()),
        TextDrawable::new(CURRENT_AMMO_TEXT, Position::new(1.9, -1.9)),
        TextDrawable::new(CURRENT_MAGAZINE_TEXT, Position::new(1.9, -1.94)),
      ]
    }
  }
}

impl specs::prelude::Component for HudObjects {
  type Storage = specs::storage::VecStorage<HudObjects>;
}
