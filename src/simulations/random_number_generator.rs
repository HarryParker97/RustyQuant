use rand::prelude::*;
use rand_distr::Normal;

pub fn standard_normal_generator(n: usize, mean: f32, std_dev: f32) -> Vec<f32> {
    let mut rng = thread_rng();
    let normal = Normal::new(mean, std_dev).unwrap();
    (0..n).map(|_| rng.sample(&normal)).collect()
}