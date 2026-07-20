//! Temperature-dependent thermophysical response laws.

mod error;
mod law;
mod response;

pub use error::{
    CoefficientOrder, InvalidTemperatureCoefficient, TemperatureLawError, TemperatureRole,
};
pub use law::{ResponseSet, TemperatureLaw, ThermophysicalResponseSet};
pub use response::{ConstantResponse, LinearResponse, QuadraticResponse, TemperatureResponse};
