// use rusty_quant::simulations::gbm::GBMMonteCarlo;
use rusty_quant::pricing_engine::option_type::{
    CalculationType, 
    AnalyticalMethod, 
    // SimulationMethod, 
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

     println!("{}", bs_option.calculate());



    // println!("{}", bs_option.calculate(OptionType::Call));


    // let sim_method: GBMMonteCarlo = GBMMonteCarlo{
    //     s0: 100.0,
    //     mu: 0.05,
    //     sigma: 0.2,
    //     t: 1.0,
    //     n_sims: 1000,
    // };

    // let calculation_type = CalculationType::SimulationMethod(
    //     SimulationMethod::GBMMonteCarlo(
    //         sim_method
    //     )
    // );

    // match calculation_type {
    //     CalculationType::SimulationMethod(sim) => {
    //         println!("SIMULATION");
    //     }
    //     CalculationType::AnalyticalMethod(anal) => {
    //         println!("ANALYTICAL")
    //     }
    // };

    
    // let option_pricer: VanillaOptionPricing = VanillaOptionPricing {
    //     calculation_type: calculation_type,
    //     option_type: OptionType::Call,
    //     simulation_method: SimulationMethod::GBMMonteCarlo(sim_method),
    //     strike: 90.0,
    // };

    // println!("Option Price: {}", option_pricer.pricing());

    // let price: f32 = gbm_struct.option_pricing(80.0, OptionType::Put);

    // println!("Option_Price: {}", price)

}