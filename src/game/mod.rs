use num::Integer;
use rand::distributions::uniform::SampleUniform;
use rand::Rng;

pub mod constants;

pub fn get_random_bool() -> bool {
  let mut rnd = rand::thread_rng();
  rnd.gen()
}

pub fn get_rand_from_range<T>(min: T, max: T) -> T
  where T: Integer + SampleUniform {
  let mut rnd = rand::thread_rng();
  rnd.gen_range(min, max)
}

#[allow(dead_code)]
pub fn get_weighted_random(weight: f32) -> bool {
  let mut rnd = rand::thread_rng();
  rnd.gen::<f32>() < weight
}
