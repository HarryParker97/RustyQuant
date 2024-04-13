
use super::option_type::{OptionType, SimulationMethod};



pub struct OptionPricing {
    pub option_type: OptionType,
    pub simulation_method: SimulationMethod,
    pub strike: f32,
}

impl OptionPricing {

    pub fn pricing(
        &self, 
    ) -> f32 {
        
        let rate_sim_vec: Vec<f32> = match &self.simulation_method{
            SimulationMethod::GBMMonteCarlo(gbm) =>gbm.simulate(),
        };

        let pay_off_vec: Vec<f32> = match self.option_type {

            OptionType::Call => rate_sim_vec.iter().map(
                |&st| if st - self.strike > 0.0 {st - self.strike} else {0.0}
            ).collect(),

            OptionType::Put => rate_sim_vec.iter().map(
                |&st| if self.strike - st > 0.0 {self.strike - st} else {0.0}
            ).collect(),
        };
        
        pay_off_vec.iter().sum::<f32>() / ( pay_off_vec.len() as f32 )

    }
}