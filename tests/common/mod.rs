//! Shared fixtures for the integration tests.
//!
//! The reference continuum fixture (`rho = 1000`, `c_p = 4000`, `k = 0.6`) is
//! used by several test binaries; centralizing it here keeps the values and
//! their validity expectations from drifting apart across copies.

use aequitas::systems::si::quantities::{
    MassDensity as DensityQuantity, SpecificHeatCapacity as HeatCapacityQuantity,
    ThermalConductivity as ConductivityQuantity,
};
use eunomia::RealField;
use proteus::{MassDensity, SpecificHeatCapacity, ThermalConductivity, ThermophysicalProperties};

/// Compose a validated thermophysical bundle from canonical-SI base scalars.
///
/// # Panics
///
/// Panics when `density` is not positive, or when `heat_capacity` or
/// `conductivity` is not finite and non-negative.
#[must_use]
pub fn properties<T: RealField>(
    density: T,
    heat_capacity: T,
    conductivity: T,
) -> ThermophysicalProperties<T> {
    ThermophysicalProperties::new(
        MassDensity::new(DensityQuantity::from_base(density))
            .expect("fixture density is finite and non-negative"),
        SpecificHeatCapacity::new(HeatCapacityQuantity::from_base(heat_capacity))
            .expect("fixture heat capacity is finite and positive"),
        ThermalConductivity::new(ConductivityQuantity::from_base(conductivity))
            .expect("fixture conductivity is finite and non-negative"),
    )
    .expect("fixture density is finite and positive")
}

/// The canonical reference continuum fixture in canonical-SI base units:
/// `rho = 1000`, `c_p = 4000`, `k = 0.6`.
#[must_use]
pub fn reference<T: RealField>() -> ThermophysicalProperties<T> {
    properties(T::from_f64(1_000.0), T::from_f64(4_000.0), T::from_f64(0.6))
}
