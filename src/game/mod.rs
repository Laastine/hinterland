use rand;
use rand::Rng;

pub mod constants;

pub fn get_random_bool() -> bool {
  let mut rnd = rand::thread_rng();
  rnd.gen()
}

#[allow(dead_code)]
pub fn get_weighted_random(weight: f32) -> bool {
  let mut rnd = rand::thread_rng();
  rnd.gen::<f32>() < weight
}
