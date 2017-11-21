use specs;

#[derive(Debug)]
pub struct Bullet {
  direction: u32
}

impl Bullet {
  pub fn new(direction: u32) -> Bullet {
    Bullet {
      direction
    }
  }
}

impl specs::Component for Bullet {
  type Storage = specs::BTreeStorage<Bullet>;
}
