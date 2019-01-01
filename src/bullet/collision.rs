use crate::bullet::bullets::Bullets;
use specs;
use specs::prelude::WriteStorage;

pub struct CollisionSystem;

impl CollisionSystem {
  pub fn new() -> CollisionSystem {
    CollisionSystem {}
  }
}

#[derive(Clone, PartialEq)]
pub enum Collision {
  Flying,
  Hit,
  OutOfBounds,
}

impl<'a> specs::prelude::System<'a> for CollisionSystem {
  type SystemData = (WriteStorage<'a, Bullets>);

  fn run(&mut self, mut bullets: Self::SystemData) {
    use specs::join::Join;

    for bs in (&mut bullets).join() {
      Bullets::remove_old_bullets(bs);
    }
  }
}
