use specs;

#[derive(Debug)]
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
    if self.character_idx < 13 {
      self.character_idx += 1;
    } else {
      self.character_idx = 0;
    }
    self.character_fire_idx = 0;
  }

  pub fn update_fire(&mut self) {
    if self.character_fire_idx < 4 {
      self.character_fire_idx += 1;
    } else {
      self.character_fire_idx = 0;
    }
  }
}

impl specs::Component for CharacterSprite {
  type Storage = specs::VecStorage<CharacterSprite>;
}

#[derive(Debug)]
pub struct ZombieSprite {
  pub zombie_idx: usize,
}

impl ZombieSprite {
  pub fn new() -> ZombieSprite {
    ZombieSprite {
      zombie_idx: 0,
    }
  }

  pub fn update_walk(&mut self) {
    if self.zombie_idx < 8 {
      self.zombie_idx += 1;
    } else {
      self.zombie_idx = 0;
    }
  }

  pub fn update_still(&mut self) {
    if self.zombie_idx < 3 {
      self.zombie_idx += 1;
    } else {
      self.zombie_idx = 0;
    }
  }
}

impl specs::Component for ZombieSprite {
  type Storage = specs::VecStorage<ZombieSprite>;
}

#[derive(Debug)]
pub struct CritterData {
  pub data: [f32; 4]
}

impl CritterData {
  pub fn new(data: [f32; 4]) -> CritterData {
    CritterData { data }
  }
}
