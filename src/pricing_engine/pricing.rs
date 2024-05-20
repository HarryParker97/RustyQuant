
use super::option_type::{CalculationType, OptionType, AnalyticalMethod, SimulationMethod};
use crate::simulations::gbm::Simulation;
use crate::analytical::black_scholes::AnalyticalTrait;

pub trait OptionPricingTrait {

    fn calculate(&self) -> f32;

    fn analytical_pricing(&self) -> f32;
}



pub struct VanillaOptionPricing {

    pub calculation_type: CalculationType,
    pub option_type: OptionType,
    pub spot: f32,
    pub strike: f32,
    pub dt: f32
}

impl OptionPricingTrait for VanillaOptionPricing {

    fn calculate(
        &self, 
    ) -> f32 {

        let option_price: f32 = match &self.calculation_type {
            CalculationType::AnalyticalMethod(anal) => self.analytical_pricing(),
            CalculationType::SimulationMethod(sim) => 0.99
        };

        option_price
    }

    fn analytical_pricing(
        &self,
    ) -> f32 {
        
        if let CalculationType::AnalyticalMethod(anal) = &self.calculation_type {
            match anal {
                AnalyticalMethod::BlackScholes(bs) => {
                    let price: f32 = bs.calculate(
                        &self.option_type,
                        self.spot,
                        self.strike,
                        self.dt,
                    );
                    return price 
                }
            }
        }
        0.0
    }
        
    //     let rate_sim_vec: Vec<f32> = match &self.simulation_method{
    //         SimulationMethod::GBMMonteCarlo(gbm) => gbm.simulate(),
    //     };

    //     let pay_off_vec: Vec<f32> = match self.option_type {

    //         OptionType::Call => rate_sim_vec.iter().map(
    //             |&st| if st - self.strike > 0.0 {st - self.strike} else {0.0}
    //         ).collect(),

    //         OptionType::Put => rate_sim_vec.iter().map(
    //             |&st| if self.strike - st > 0.0 {self.strike - st} else {0.0}
    //         ).collect(),
    //     };
        
    //     pay_off_vec.iter().sum::<f32>() / ( pay_off_vec.len() as f32 )

    // }
}