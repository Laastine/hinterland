use specs;

#[derive(Debug)]
pub struct Character;

impl specs::Component for Character {
  type Storage = specs::HashMapStorage<Character>;
}
