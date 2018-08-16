#[derive(Clone, Debug, Default)]
pub struct CharacterStats {
  pub ammunition: usize,
  pub magazines: usize,
}

impl CharacterStats {
  pub fn new() -> CharacterStats {
    CharacterStats {
      ammunition: 10,
      magazines: 1,
    }
  }
}
