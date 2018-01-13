use rand;
use rand::Rng;

pub mod constants;

pub fn get_random_bool() -> bool {
  let mut rnd = rand::thread_rng();
  rnd.gen()
}


