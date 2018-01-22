use bullet::BulletDrawable;
use cgmath;
use specs;

#[derive(Debug, Clone)]
pub struct Bullets {
  pub bullets: Vec<BulletDrawable>,
}

impl Bullets {
  pub fn new() -> Bullets {
    Bullets {
      bullets: Vec::new()
    }
  }

  pub fn add_bullet(&mut self, position: cgmath::Point2<f32>, movement_direction: cgmath::Point2<f32>) {
    self.bullets.push(BulletDrawable::new(position, movement_direction));
  }
}

impl specs::Component for Bullets {
  type Storage = specs::VecStorage<Bullets>;
}
