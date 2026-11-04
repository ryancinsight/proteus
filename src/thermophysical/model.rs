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
    /// For finite `rho > 0`, `c_p > 0`, and `k >= 0`, the result is
    /// non-negative and never NaN. Binary scaling prevents a prematurely
    /// rounded capacity product from destroying a representable quotient.
    /// Native residual comparisons preserve final zero/overflow classification;
    /// finite values have a four-ULP error bound, rather than a universal
    /// correct-rounding guarantee. Zero conductivity retains its sign.
    /// Aequitas proves the result dimension is `L^2 / T`.
    #[must_use]
    pub fn thermal_diffusivity(&self) -> ThermalDiffusivity<T> {
        let density = self.density.into_quantity();
        let heat_capacity = self.specific_heat_capacity.into_quantity();
        let conductivity = self.thermal_conductivity.into_quantity();
        let capacity = density * heat_capacity;
        let scalar = *capacity.as_base();
        let minimum = super::range::minimum_normal::<T>();
        if scalar.is_finite() && scalar >= minimum {
            let result = conductivity / capacity;
            let value = *result.as_base();
            // Preserve the original operation order in the interior. Even a
            // normal capacity can round across a final zero/overflow midpoint,
            // so those extremes require the original-input comparison too.
            if (value > minimum * T::EPSILON && value < T::INFINITY.bitxor(minimum))
                || *conductivity.as_base() == T::ZERO
            {
                return result;
            }
        }
        super::range::diffusivity(density, heat_capacity, conductivity)
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
