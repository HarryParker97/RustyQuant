
use crate::simulations::gbm::GBMMonteCarlo;


pub enum OptionType {
    Call,
    Put,
}

pub enum SimulationMethod {
    GBMMonteCarlo(GBMMonteCarlo),
}