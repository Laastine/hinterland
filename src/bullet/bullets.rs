use cgmath;
use specs;

use crate::bullet::{BulletDrawable, collision::Collision};
use crate::shaders::Position;

pub struct Bullets {
  pub bullets: Vec<BulletDrawable>,
}

impl Bullets {
  pub fn new() -> Bullets {
    Bullets {
      bullets: Vec::new()
    }
  }

  pub fn add_bullet(&mut self, position: Position, movement_direction: cgmath::Point2<f32>) {
    self.bullets.push(BulletDrawable::new(position, movement_direction));
  }

  pub fn remove_old_bullets(&mut self) {
    self.bullets.retain(|ref mut b| b.status == Collision::Flying);
  }
}

impl specs::prelude::Component for Bullets {
  type Storage = specs::storage::VecStorage<Bullets>;
}
