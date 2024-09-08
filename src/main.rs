use rusty_quant::simulations::gbm::GBMMonteCarlo;
use rusty_quant::pricing_engine::option_type::{
    CalculationType, 
    AnalyticalMethod, 
    SimulationMethod, 
    OptionType
};
use rusty_quant::pricing_engine::pricing::VanillaOptionPricing;
use rusty_quant::analytical::black_scholes::BlackScholes;
use rusty_quant::pricing_engine::pricing::OptionPricingTrait;

fn main() {
    
    let s0: f32 = 100.0;
    let k: f32 = 100.0;
    let mu: f32 =  0.05;
    let sigma: f32 = 0.2;
    let t: f32 = 1.0;
    let n_sims: u32 = 1000;

    let calculation_type = CalculationType::AnalyticalMethod(
        AnalyticalMethod::BlackScholes(
            BlackScholes{
                r: mu,
                vol: sigma,
            }
        )
    );

    let bs_option: VanillaOptionPricing = VanillaOptionPricing{
        calculation_type: calculation_type,
        option_type: OptionType::Call,
        spot: s0,
        strike: k,
        dt: t
    };


     println!("{:?}", bs_option.calculate().unwrap());


    let calculation_type = CalculationType::SimulationMethod(
        SimulationMethod::GBMMonteCarlo(
            GBMMonteCarlo{
                mu: mu,
                sigma: sigma,
                n_sims: n_sims
            }
        )
    );

    let bs_option: VanillaOptionPricing = VanillaOptionPricing{
        calculation_type: calculation_type,
        option_type: OptionType::Call,
        spot: s0,
        strike: k,
        dt: t
    };

     println!("{:#?}", bs_option.calculate().unwrap());

}