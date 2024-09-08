use std::f32::consts::E;
use super::random_number_generator::standard_normal_generator;


pub trait SimulationTrait {
    
    fn simulate(
        &self, 
        s0: f32, 
        dt: f32
    ) -> Vec<f32>;

}

pub struct GBMMonteCarlo {
    pub mu: f32, 
    pub sigma: f32,
    pub n_sims: u32,
}


impl SimulationTrait for GBMMonteCarlo{

    fn simulate(
        &self,
        s0: f32,
        dt: f32,
    ) -> Vec<f32> {

        let st_norm_vec: Vec<f32> = standard_normal_generator(self.n_sims);

        let rate_sim: Vec<f32> = st_norm_vec.iter().map(
            |&dw| s0 * E.powf(
                (self.mu - 0.5 * self.sigma.powf(2.0)) * dt + self.sigma * dw
            )
        ).collect();

        rate_sim
    }
}