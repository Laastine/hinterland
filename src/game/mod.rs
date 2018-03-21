use rand;
use rand::Rng;

pub mod constants;

pub fn get_random_bool() -> bool {
  let mut rnd = rand::thread_rng();
  rnd.gen()
}

pub fn get_tenth_bool() -> bool {
  let mut rnd = rand::thread_rng();
  rnd.gen::<f32>() < 0.05
}


