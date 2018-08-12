use shaders::Position;
use specs;
use zombie::ZombieDrawable;

#[derive(Debug, Clone)]
pub struct Zombies {
  pub zombies: Vec<ZombieDrawable>,
}

impl Zombies {
  pub fn new() -> Zombies {
    Zombies {
      zombies: vec![
        // 1
        ZombieDrawable::new(Position::new(200.0, 10.0)),
        ZombieDrawable::new(Position::new(-200.0, 10.0)),
        ZombieDrawable::new(Position::new(10.0, 200.0)),
        ZombieDrawable::new(Position::new(10.0, -200.0)),
        // 2
        ZombieDrawable::new(Position::new(500.0, 40.0)),
        ZombieDrawable::new(Position::new(-500.0, 40.0)),
        ZombieDrawable::new(Position::new(40.0, 500.0)),
        ZombieDrawable::new(Position::new(40.0, -500.0)),
        // 3
        ZombieDrawable::new(Position::new(700.0, 60.0)),
        ZombieDrawable::new(Position::new(-900.0, 60.0)),
        ZombieDrawable::new(Position::new(60.0, 700.0)),
        ZombieDrawable::new(Position::new(60.0, -700.0)),
        // 4
        ZombieDrawable::new(Position::new(800.0, 160.0)),
        ZombieDrawable::new(Position::new(-1000.0, 160.0)),
        ZombieDrawable::new(Position::new(160.0, 800.0)),
        ZombieDrawable::new(Position::new(160.0, -800.0)),
      ]
    }
  }
}

impl specs::prelude::Component for Zombies {
  type Storage = specs::storage::VecStorage<Zombies>;
}
