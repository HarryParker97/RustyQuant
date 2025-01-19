use super::random_number_generator::standard_normal_generator;


pub trait SimulationTrait {
    
    fn simulate(
        &self, 
        s0: f32, 
        dt: f32,
    ) -> Vec<f32>;

}

pub struct GBMMonteCarlo {
    pub mu: f32, 
    pub sigma: f32,
    pub n_sims: u32,
}


impl SimulationTrait for GBMMonteCarlo{

    fn simulate(&self, s0: f32, dt: f32) -> Vec<f32> {
    
        let st_norm_vec: Vec<f32> = standard_normal_generator(self.n_sims as usize, 0.0, 1.0);
    
        let s_t: Vec<f32> = st_norm_vec.iter().map(|&dw| s0 * (
            (self.mu - 0.5 * self.sigma.powi(2)) * dt + self.sigma * dt.sqrt() * dw
        ).exp()).collect();
        
        s_t
    }
}