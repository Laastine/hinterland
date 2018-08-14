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
        TextDrawable::new("v0.3.6", Position::new(0.0, 0.0)),
        TextDrawable::new("Ammo 10", Position::new(1.9, -1.94)),
      ]
    }
  }
}

impl specs::prelude::Component for HudObjects {
  type Storage = specs::storage::VecStorage<HudObjects>;
}
