
use crate::analytical::black_scholes::BlackScholes;
use crate::simulations::gbm::GBMMonteCarlo;


pub enum OptionType {
    Call,
    Put,
}

pub enum CalculationType {
    AnalyticalMethod(AnalyticalMethod),
    SimulationMethod(SimulationMethod)
}

pub enum AnalyticalMethod { // Fill this out with Fourier methods and PDE solvers
    BlackScholes(BlackScholes)
}

pub enum SimulationMethod {
    GBMMonteCarlo(GBMMonteCarlo),
}