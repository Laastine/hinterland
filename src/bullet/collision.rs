use bullet::bullets::Bullets;
use specs;
use specs::WriteStorage;

#[derive(Debug)]
pub struct CollisionSystem;

impl CollisionSystem {
  pub fn new() -> CollisionSystem {
    CollisionSystem {}
  }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Collision {
  Flying,
  Hit,
  OutOfBounds,
}

impl<'a> specs::System<'a> for CollisionSystem {
  type SystemData = (WriteStorage<'a, Bullets>);

  fn run(&mut self, mut bullets: Self::SystemData) {
    use specs::Join;

    for bs in (&mut bullets).join() {
      Bullets::remove_old_bullets(bs);
    }
  }
}
