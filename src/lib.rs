//! Atlas material-property and constitutive-law foundation.
//!
//! Proteus owns validated material properties, material identity, and static
//! constitutive-law evaluation. Aequitas owns dimensions and units; Eunomia
//! owns scalar representations and arithmetic.

#![doc = include_str!("../README.md")]
#![no_std]
#![forbid(unsafe_code)]
#![deny(missing_docs)]

extern crate alloc;

/// Constitutive-law contracts and static implementations.
pub mod constitutive;
/// Material identity and law composition.
pub mod material;
/// Validated physical material properties.
pub mod property;
/// Cohesive thermophysical property bundles.
pub mod thermophysical;

pub use constitutive::{
    CoefficientOrder, ConstantLaw, ConstantResponse, ConstitutiveLaw,
    InvalidTemperatureCoefficient, LinearResponse, NoState, QuadraticResponse, ResponseSet,
    TemperatureLaw, TemperatureLawError, TemperatureResponse, TemperatureRole,
    ThermophysicalResponseSet,
};
pub use material::Material;
pub use property::{
    InvalidProperty, MassDensity, PropertyConstraint, PropertyKind, SpecificHeatCapacity,
    ThermalConductivity,
};
pub use thermophysical::ThermophysicalProperties;
