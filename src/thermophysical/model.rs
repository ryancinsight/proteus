use aequitas::systems::si::quantities::{
    MassDensity as DensityQuantity, SpecificHeatCapacity as HeatCapacityQuantity,
    ThermalConductivity as ConductivityQuantity, ThermalDiffusivity,
};
use eunomia::{NumericElement, RealField};

use crate::property::{
    InvalidProperty, MassDensity, PropertyConstraint, PropertyKind, SpecificHeatCapacity,
    ThermalConductivity,
};

/// Cohesive isotropic thermophysical material properties.
///
/// Construction requires positive density and specific heat, and non-negative
/// conductivity. The thermal-diffusivity law is therefore total.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ThermophysicalProperties<T> {
    density: MassDensity<T>,
    specific_heat_capacity: SpecificHeatCapacity<T>,
    thermal_conductivity: ThermalConductivity<T>,
}

impl<T: RealField> ThermophysicalProperties<T> {
    /// Validate canonical dimensional quantities and compose their property
    /// bundle in one boundary operation.
    ///
    /// # Errors
    ///
    /// Returns [`InvalidProperty`] naming the first invalid component.
    pub fn try_from_quantities(
        density: DensityQuantity<T>,
        specific_heat_capacity: HeatCapacityQuantity<T>,
        thermal_conductivity: ConductivityQuantity<T>,
    ) -> Result<Self, InvalidProperty<T>> {
        Self::new(
            MassDensity::new(density)?,
            SpecificHeatCapacity::new(specific_heat_capacity)?,
            ThermalConductivity::new(thermal_conductivity)?,
        )
    }

    /// Construct a thermophysical property bundle.
    ///
    /// # Errors
    ///
    /// Returns [`InvalidProperty`] when density is zero. The component
    /// newtypes have already rejected every other invalid state.
    pub fn new(
        density: MassDensity<T>,
        specific_heat_capacity: SpecificHeatCapacity<T>,
        thermal_conductivity: ThermalConductivity<T>,
    ) -> Result<Self, InvalidProperty<T>> {
        if *density.quantity().as_base() == <T as NumericElement>::ZERO {
            return Err(InvalidProperty::new(
                PropertyKind::MassDensity,
                <T as NumericElement>::ZERO,
                PropertyConstraint::FinitePositive,
            ));
        }
        Ok(Self {
            density,
            specific_heat_capacity,
            thermal_conductivity,
        })
    }

    /// Derive thermal diffusivity `alpha = k / (rho c_p)`.
    ///
    /// # Theorem
    ///
    /// For `rho > 0`, `c_p > 0`, and `k >= 0`, `alpha` is finite and
    /// non-negative whenever the native scalar operations do not overflow.
    /// Positivity follows because the denominator is positive and the numerator
    /// is non-negative. Aequitas proves the result dimension is `L^2 / T`.
    #[must_use]
    pub fn thermal_diffusivity(&self) -> ThermalDiffusivity<T> {
        self.thermal_conductivity.into_quantity()
            / (self.density.into_quantity() * self.specific_heat_capacity.into_quantity())
    }
}

impl<T> ThermophysicalProperties<T> {
    /// Borrow mass density.
    #[must_use]
    pub const fn density(&self) -> &MassDensity<T> {
        &self.density
    }

    /// Borrow specific heat capacity.
    #[must_use]
    pub const fn specific_heat_capacity(&self) -> &SpecificHeatCapacity<T> {
        &self.specific_heat_capacity
    }

    /// Borrow thermal conductivity.
    #[must_use]
    pub const fn thermal_conductivity(&self) -> &ThermalConductivity<T> {
        &self.thermal_conductivity
    }
}
