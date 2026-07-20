//! Constitutive-law seams.

mod constant;
mod contract;
mod temperature;

pub use constant::{ConstantLaw, NoState};
pub use contract::ConstitutiveLaw;
pub use temperature::{
    CoefficientOrder, ConstantResponse, InvalidTemperatureCoefficient, LinearResponse,
    QuadraticResponse, ResponseSet, TemperatureLaw, TemperatureLawError, TemperatureResponse,
    TemperatureRole, ThermophysicalResponseSet,
};
