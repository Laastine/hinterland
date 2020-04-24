pub struct CharacterSprite {
  pub character_idx: usize,
  pub character_fire_idx: usize,
}

impl CharacterSprite {
  pub fn new() -> CharacterSprite {
    CharacterSprite {
      character_idx: 0,
      character_fire_idx: 0,
    }
  }

  pub fn update_run(&mut self) {
    if self.character_idx < 12 {
      self.character_idx += 1;
    } else {
      self.character_idx = 0;
    }
    self.character_fire_idx = 0;
  }

  pub fn update_fire(&mut self) {
    if self.character_fire_idx < 3 {
      self.character_fire_idx += 1;
    } else {
      self.character_fire_idx = 0;
    }
  }
}

impl specs::prelude::Component for CharacterSprite {
  type Storage = specs::storage::VecStorage<CharacterSprite>;
}

pub struct CritterData {
  pub data: [f32; 4]
}

impl CritterData {
  pub fn new(data: [f32; 4]) -> CritterData {
    CritterData { data }
  }
}
