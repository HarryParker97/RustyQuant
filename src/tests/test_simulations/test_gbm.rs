use crate::simulations::gbm::GBMMonteCarlo;
use crate::pricing_engine::option_type::{
    CalculationType, 
    AnalyticalMethod, 
    SimulationMethod, 
    OptionType
};
use crate::pricing_engine::pricing::VanillaOptionPricing;
use crate::analytical::black_scholes::BlackScholes;
use crate::pricing_engine::pricing::OptionPricingTrait;

#[test]
fn test_analytical_equals_simulation() {
    // Parameters for the test
    let s0: f32 = 100.0;   // Spot price
    let k: f32 = 100.0;    // Strike price
    let mu: f32 = 0.05;    // Risk-free rate
    let sigma: f32 = 0.2;  // Volatility
    let t: f32 = 1.0;      // Time to maturity
    let n_sims: u32 = 1_000_000; // Number of simulations
    let tolerance: f32 = 0.01; // Allowed tolerance between methods

    let analytical_calculation = CalculationType::AnalyticalMethod(
        AnalyticalMethod::BlackScholes(
            BlackScholes {
                r: mu,
                vol: sigma,
            }
        )
    );
    let analytical_option = VanillaOptionPricing {
        calculation_type: analytical_calculation,
        option_type: OptionType::Call,
        spot: s0,
        strike: k,
        dt: t,
    };
    let analytical_price = analytical_option.calculate().unwrap();

    println!("{}", analytical_price);

    let simulation_calculation = CalculationType::SimulationMethod(
        SimulationMethod::GBMMonteCarlo(
            GBMMonteCarlo {
                mu: mu,
                sigma: sigma,
                n_sims: n_sims,
            }
        )
    );
    let simulation_option = VanillaOptionPricing {
        calculation_type: simulation_calculation,
        option_type: OptionType::Call,
        spot: s0,
        strike: k,
        dt: t,
    };
    let simulation_price = simulation_option.calculate().unwrap();

    println!("{}", simulation_price);

    assert!(
        (analytical_price - simulation_price).abs() < tolerance,
        "Prices do not match within tolerance: Analytical = {}, Simulation = {}",
        analytical_price,
        simulation_price
    );
}