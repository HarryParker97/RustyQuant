use super::option_type::{CalculationType, OptionType, AnalyticalMethod, SimulationMethod};
use crate::analytical::black_scholes::AnalyticalTrait;
use crate::simulations::gbm::SimulationTrait;

pub trait OptionPricingTrait {

    fn calculate(&self) -> Result<f32, String>;
    fn analytical_pricing(&self) -> Result<f32, String>;
    fn simulation_pricing(&self) -> Result<f32, String>;
}



pub struct VanillaOptionPricing {

    pub calculation_type: CalculationType,
    pub option_type: OptionType,
    pub spot: f32,
    pub strike: f32,
    pub dt: f32
}

impl OptionPricingTrait for VanillaOptionPricing {

    fn calculate(&self) -> Result<f32, String> {

        match &self.calculation_type {
            CalculationType::AnalyticalMethod(_) => self.analytical_pricing(),
            CalculationType::SimulationMethod(_) => self.simulation_pricing(),
        }
    }

    fn analytical_pricing(
        &self,
    ) -> Result<f32, String> {
        
        if let CalculationType::AnalyticalMethod(anal) = &self.calculation_type {
            match anal {
                AnalyticalMethod::BlackScholes(bs) => {
                    let price: f32 = bs.calculate(
                        &self.option_type,
                        self.spot,
                        self.strike,
                        self.dt,
                    );
                    return Ok(price) 
                }
            }
        }
        Err("Invalid calculation type for analytical pricing.".to_string())
    }

    fn simulation_pricing(&self) -> Result<f32, String> {

        let (simulations, rf): (Vec<f32>, f32) = if let CalculationType::SimulationMethod(sim) = &self.calculation_type {
            match sim {
                SimulationMethod::GBMMonteCarlo(gbm) => {
                    let rf: f32 = gbm.mu;
    
                    (gbm.simulate(self.spot, self.dt), rf)
                }
            }
        } else {
            return Err("Invalid calculation type.".to_string());
        };
    
        if simulations.is_empty() {
            return Err("Simulation returned no results.".to_string());
        }
    
        let mut pay_off_vec: Vec<f32> = Vec::new();
    
        for simulation in simulations.iter() {
            let final_price = simulation;
    
            let payoff = match self.option_type {
                OptionType::Call => f32::max(0.0, final_price - self.strike),
                OptionType::Put => f32::max(0.0, self.strike - final_price),
            };
    
            pay_off_vec.push(payoff);
        }

        let mtm: f32 = (pay_off_vec.iter().sum::<f32>() / pay_off_vec.len() as f32) * (-rf * self.dt).exp();
    
        Ok(mtm)
    }
}