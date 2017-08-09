use specs;

#[derive(Debug)]
pub struct Character;

impl Character {
  pub fn new() -> Character {
    Character {}
  }
}

impl specs::Component for Character {
  type Storage = specs::HashMapStorage<Character>;
}
