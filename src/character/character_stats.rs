#[derive(Clone, Debug)]
pub struct CharacterStats {
  pub ammunition: usize,
  pub shots_fired: usize,
  pub magazines: usize,
  pub health: usize,
}

impl CharacterStats {
  pub fn new() -> CharacterStats {
    CharacterStats {
      ammunition: 10,
      shots_fired: 0,
      magazines: 2,
      health: 100,
    }
  }
}
