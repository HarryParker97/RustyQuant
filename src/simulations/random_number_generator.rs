use rand::prelude::*;
use rand_distr::Normal;

pub fn standard_normal_generator(n: u32, mean: f32, std_dev: f32) -> Vec<f32> {

    let mut rng = thread_rng(); // Initialize the random number generator
    let normal: Normal<f32> = Normal::new(mean, std_dev).unwrap(); // Create a normal distribution

    let mut random_numbers_vec = Vec::with_capacity(n as usize);
    for _ in 0..n {
        let rand_num: f32 = rng.sample(normal); // Sample from the distribution
        // println!("Random Number: {}", rand_num);

        random_numbers_vec.push(rand_num);

    }

    random_numbers_vec
}