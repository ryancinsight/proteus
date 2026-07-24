use aequitas::systems::si::quantities::{
    Dimensionless, TemperatureDifference, ThermodynamicTemperature,
};
use eunomia::{NumericElement, RealField};

use crate::{ConstitutiveLaw, ThermophysicalProperties};

use super::{TemperatureLawError, TemperatureResponse, TemperatureRole};

mod private {
    pub trait Sealed {}
}

/// Static strategy supplying independent thermophysical response factors.
pub trait ThermophysicalResponseSet<T: RealField>: private::Sealed {
    /// Density multiplier.
    fn density_factor(&self, delta: TemperatureDifference<T>) -> Dimensionless<T>;
    /// Specific-heat-capacity multiplier.
    fn specific_heat_factor(&self, delta: TemperatureDifference<T>) -> Dimensionless<T>;
    /// Thermal-conductivity multiplier.
    fn conductivity_factor(&self, delta: TemperatureDifference<T>) -> Dimensionless<T>;
}

/// Independent density, heat-capacity, and conductivity responses.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ResponseSet<Density, HeatCapacity, Conductivity> {
    density: Density,
    heat_capacity: HeatCapacity,
    conductivity: Conductivity,
}

impl<Density, HeatCapacity, Conductivity> ResponseSet<Density, HeatCapacity, Conductivity> {
    /// Compose the three independent property responses.
    #[must_use]
    pub const fn new(
        density: Density,
        heat_capacity: HeatCapacity,
        conductivity: Conductivity,
    ) -> Self {
        Self {
            density,
            heat_capacity,
            conductivity,
        }
    }

    /// Borrow the density response.
    #[must_use]
    pub const fn density(&self) -> &Density {
        &self.density
    }

    /// Borrow the specific-heat response.
    #[must_use]
    pub const fn heat_capacity(&self) -> &HeatCapacity {
        &self.heat_capacity
    }

    /// Borrow the conductivity response.
    #[must_use]
    pub const fn conductivity(&self) -> &Conductivity {
        &self.conductivity
    }
}

impl<Density, HeatCapacity, Conductivity> private::Sealed
    for ResponseSet<Density, HeatCapacity, Conductivity>
{
}

impl<T, Density, HeatCapacity, Conductivity> ThermophysicalResponseSet<T>
    for ResponseSet<Density, HeatCapacity, Conductivity>
where
    T: RealField,
    Density: TemperatureResponse<T>,
    HeatCapacity: TemperatureResponse<T>,
    Conductivity: TemperatureResponse<T>,
{
    #[inline]
    fn density_factor(&self, delta: TemperatureDifference<T>) -> Dimensionless<T> {
        self.density.factor(delta)
    }

    #[inline]
    fn specific_heat_factor(&self, delta: TemperatureDifference<T>) -> Dimensionless<T> {
        self.heat_capacity.factor(delta)
    }

    #[inline]
    fn conductivity_factor(&self, delta: TemperatureDifference<T>) -> Dimensionless<T> {
        self.conductivity.factor(delta)
    }
}

/// Thermophysical law evaluated relative to a reference temperature.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TemperatureLaw<T, Responses> {
    reference_properties: ThermophysicalProperties<T>,
    reference_temperature: ThermodynamicTemperature<T>,
    responses: Responses,
}

impl<T: RealField, Responses> TemperatureLaw<T, Responses> {
    /// Construct a temperature-dependent law from validated reference state.
    ///
    /// # Errors
    ///
    /// Returns [`TemperatureLawError::InvalidTemperature`] when the reference
    /// temperature is non-finite or not strictly positive.
    pub fn new(
        reference_properties: ThermophysicalProperties<T>,
        reference_temperature: ThermodynamicTemperature<T>,
        responses: Responses,
    ) -> Result<Self, TemperatureLawError<T>> {
        validate_temperature(TemperatureRole::Reference, reference_temperature)?;
        Ok(Self {
            reference_properties,
            reference_temperature,
            responses,
        })
    }

    /// Borrow the reference properties.
    #[must_use]
    pub const fn reference_properties(&self) -> &ThermophysicalProperties<T> {
        &self.reference_properties
    }

    /// Borrow the reference temperature.
    #[must_use]
    pub const fn reference_temperature(&self) -> &ThermodynamicTemperature<T> {
        &self.reference_temperature
    }

    /// Borrow the response strategy.
    #[must_use]
    pub const fn responses(&self) -> &Responses {
        &self.responses
    }
}

impl<T, Responses> ConstitutiveLaw<T> for TemperatureLaw<T, Responses>
where
    T: RealField,
    Responses: ThermophysicalResponseSet<T>,
{
    type State<'a>
        = &'a ThermodynamicTemperature<T>
    where
        Self: 'a,
        T: 'a;
    type Error = TemperatureLawError<T>;

    #[inline]
    fn properties<'a>(
        &'a self,
        temperature: Self::State<'a>,
    ) -> Result<ThermophysicalProperties<T>, Self::Error>
    where
        T: 'a,
    {
        validate_temperature(TemperatureRole::Evaluation, *temperature)?;
        let delta = *temperature - self.reference_temperature;
        let density =
            *self.reference_properties.density().quantity() * self.responses.density_factor(delta);
        let heat_capacity = *self
            .reference_properties
            .specific_heat_capacity()
            .quantity()
            * self.responses.specific_heat_factor(delta);
        let conductivity = *self.reference_properties.thermal_conductivity().quantity()
            * self.responses.conductivity_factor(delta);

        ThermophysicalProperties::try_from_quantities(density, heat_capacity, conductivity)
            .map_err(TemperatureLawError::from)
    }
}

fn validate_temperature<T: RealField>(
    role: TemperatureRole,
    temperature: ThermodynamicTemperature<T>,
) -> Result<(), TemperatureLawError<T>> {
    let value = *temperature.as_base();
    if value.is_finite() && value > <T as NumericElement>::ZERO {
        Ok(())
    } else {
        Err(TemperatureLawError::invalid_temperature(role, value))
    }
}
