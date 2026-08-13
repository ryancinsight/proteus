//! Temperature-dependent thermophysical response laws.

mod error;
mod law;
mod response;

pub use error::{
    CoefficientOrder, InvalidTemperatureCoefficient, InvalidTemperatureValidity,
    TemperatureLawError, TemperatureRole,
};
pub use law::{ResponseSet, TemperatureLaw, TemperatureValidity, ThermophysicalResponseSet};
pub use response::{ConstantResponse, LinearResponse, QuadraticResponse, TemperatureResponse};
