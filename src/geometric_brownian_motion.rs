use std::f32::consts::E;
use crate::random_number_generation::standard_normal_generator;


pub enum OptionType {
    Call,
    Put,
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

    pub fn option_pricing(
        &self, 
        strike: f32, 
        option_type: OptionType
    ) -> f32 {

        let rate_sim_vec: Vec<f32> = self.simulate();

        let pay_off_vec: Vec<f32> = match option_type {

            OptionType::Call => rate_sim_vec.iter().map(
                |&st| if st - strike > 0.0 {st - strike} else {0.0}
            ).collect(),

            OptionType::Put => rate_sim_vec.iter().map(
                |&st| if strike - st > 0.0 {strike - st} else {0.0}
            ).collect(),
        };
        
        pay_off_vec.iter().sum::<f32>() / ( pay_off_vec.len() as f32 )

    }
}