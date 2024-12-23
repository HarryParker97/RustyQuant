use std::f32::consts::E;
use super::random_number_generator::standard_normal_generator;


pub trait SimulationTrait {
    
    fn simulate(
        &self, 
        s0: f32, 
        dt: f32,
    ) -> Vec<Vec<f32>>;

}

pub struct GBMMonteCarlo {
    pub mu: f32, 
    pub sigma: f32,
    pub n_sims: u32,
}


impl SimulationTrait for GBMMonteCarlo{

    fn simulate(&self, s0: f32, dt: f32) -> Vec<Vec<f32>> {
        let num_steps: u32 = (dt * 365.0) as u32; // Total number of steps
        let timestep: f32 = 1.0 / 365.0;          // Daily timestep
    
        let mut all_simulations: Vec<Vec<f32>> = Vec::with_capacity(self.n_sims as usize);
    
        for _ in 0..self.n_sims {
            let st_norm_vec: Vec<f32> = standard_normal_generator(num_steps, 0.0, 1.0);
            let mut time_series: Vec<f32> = Vec::with_capacity(num_steps as usize);
    
            let mut log_s_t: f32 = s0.ln();
            
            for &dw in st_norm_vec.iter() {
                log_s_t += (self.mu - 0.5 * self.sigma.powf(2.0)) * timestep + self.sigma * timestep.sqrt() * dw;
                
                let next_price = log_s_t.exp();
                time_series.push(next_price);
            }
    
            all_simulations.push(time_series);
        }
    
        all_simulations
    }
}