use monte_carlo::simulations::gbm::GBMMonteCarlo;
use monte_carlo::pricing_engine::option_type::{OptionType, SimulationMethod};
use monte_carlo::pricing_engine::pricing::OptionPricing;

fn main() {


    let sim_method: GBMMonteCarlo = GBMMonteCarlo{
        s0: 100.0,
        mu: 0.05,
        sigma: 0.2,
        t: 1.0,
        n_sims: 1000,
    };
    
    let option_pricer: OptionPricing = OptionPricing {
        option_type: OptionType::Call,
        simulation_method: SimulationMethod::GBMMonteCarlo(sim_method),
        strike: 90.0,
    };

    println!("Option Price: {}", option_pricer.pricing());

    // let price: f32 = gbm_struct.option_pricing(80.0, OptionType::Put);

    // println!("Option_Price: {}", price)

}