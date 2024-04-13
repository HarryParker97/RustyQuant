use std::f32::consts::E;
use super::random_number_generator::standard_normal_generator;


pub trait Simulation {
    fn simulate(&self) -> Vec<f32>;
}


pub struct GBMMonteCarlo {
    pub s0: f32,
    pub mu: f32, 
    pub sigma: f32,
    pub t: f32,
    pub n_sims: u32,
}

impl GBMMonteCarlo{

    pub fn simulate(&self) -> Vec<f32> {

        let st_norm_vec: Vec<f32> = standard_normal_generator(self.n_sims);

        let mut rate_sim: Vec<f32> = st_norm_vec.iter().map(
            |&dw| self.s0 * E.powf(
                (self.mu - 0.5 * self.sigma.powf(2.0)) * self.t + self.sigma * dw
            )
        ).collect();

        for st_norm in rate_sim.iter_mut() {
            println!("{}", st_norm);
        }

        rate_sim
    }
}