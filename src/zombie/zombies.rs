use zombie::ZombieDrawable;
use shaders::Position;
use specs;

#[derive(Debug, Clone)]
pub struct Zombies {
  pub zombies: Vec<ZombieDrawable>,
}

impl Zombies {
  pub fn new() -> Zombies {
    Zombies {
      zombies: vec![
        ZombieDrawable::new(Position::new([200.0, 10.0])),
        ZombieDrawable::new(Position::new([-200.0, 10.0])),
        ZombieDrawable::new(Position::new([10.0, 200.0])),
        ZombieDrawable::new(Position::new([10.0, -200.0])),
      ]
    }
  }
}

impl specs::Component for Zombies {
  type Storage = specs::VecStorage<Zombies>;
}
