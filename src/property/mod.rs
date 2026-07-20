//! Validated material-property boundaries.

mod conductivity;
mod density;
mod error;
mod heat_capacity;
mod validation;

pub use conductivity::ThermalConductivity;
pub use density::MassDensity;
pub use error::{InvalidProperty, PropertyConstraint, PropertyKind};
pub use heat_capacity::SpecificHeatCapacity;
