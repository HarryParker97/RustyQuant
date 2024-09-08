use statrs::distribution::{Normal};
use statrs::distribution::ContinuousCDF;
use crate::pricing_engine::option_type::OptionType;

pub trait AnalyticalTrait {
    fn calculate(
        &self, 
        option_type: &OptionType,
        spot: f32,
        strike: f32,
        dt: f32,
    ) -> f32;
}

pub struct BlackScholes {
    pub r: f32,
    pub vol: f32,
}

impl AnalyticalTrait for BlackScholes {
    
    fn calculate(
        &self, 
        option_type: &OptionType,
        spot: f32,
        strike: f32,
        dt: f32,
    ) -> f32 {
        let d1: f32 = (
            (spot / strike).ln() + (self.r + (self.vol.powi(2) / 2.0)) * dt
        ) / (self.vol * dt.sqrt());
        let d2: f32 = d1 - self.vol * dt.sqrt();

        let price: f32 = match option_type {
            OptionType::Call => {
                let normal = Normal::new(0.0, 1.0).unwrap();
                let d1_cdf: f32 = normal.cdf(d1.into()) as f32;
                let d2_cdf: f32 = normal.cdf(d2.into()) as f32;
                spot * d1_cdf - strike * (-self.r * dt).exp() * d2_cdf
            }
            OptionType::Put => {
                let normal = Normal::new(0.0, 1.0).unwrap();
                let d1_cdf: f32 = normal.cdf((-d1).into()) as f32;
                let d2_cdf: f32 = normal.cdf((-d2).into()) as f32;
                - spot * d1_cdf + strike * (-self.r * dt).exp() * d2_cdf
            }
        };

        price
    }
}