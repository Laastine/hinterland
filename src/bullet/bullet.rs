use gfx_app::mouse_controls::MouseInputState;
use specs;
use specs::{Entities, ReadStorage, WriteStorage};
use std::sync::mpsc;

#[derive(Debug,Clone)]

pub struct Bullet;

impl Bullet {
  pub fn new() -> Bullet { Bullet {} }
}

impl specs::Component for Bullet {
  type Storage = specs::HashMapStorage<Bullet>;
}

#[derive(Debug)]
pub struct BulletControlSystem {
  queue: mpsc::Receiver<Bullet>,
}

impl BulletControlSystem {
  pub fn new() -> (BulletControlSystem, mpsc::Sender<Bullet>) {
    let (tx, rx) = mpsc::channel();
    (BulletControlSystem {
      queue: rx
    }, tx)
  }
}
