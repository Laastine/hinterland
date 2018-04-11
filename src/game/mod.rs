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

pub fn get_random_edge_point() -> (f32, f32) {
  let mut rnd = rand::thread_rng();
  let multiplier = rnd.gen_range::<f32>(0.7, 1.3);
  if rnd.gen() {
    (rnd.gen_range::<f32>(600.0 * multiplier, 700.0 * multiplier), rnd.gen_range::<f32>(600.0 * multiplier, 700.0 * multiplier))
  } else {
    (rnd.gen_range::<f32>(-700.0 * multiplier, -600.0 * multiplier), rnd.gen_range::<f32>(-700.0 * multiplier, -600.0 * multiplier))
  }
}
