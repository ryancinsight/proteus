//! Concrete-reference fixture for thermophysical code generation.

use aequitas::systems::si::quantities::{
    MassDensity as DensityQuantity, ReciprocalTemperature,
    SpecificHeatCapacity as HeatCapacityQuantity, TemperatureDifference,
    ThermalConductivity as ConductivityQuantity,
};
use proteus::{
    LinearResponse, MassDensity, SpecificHeatCapacity, TemperatureResponse, ThermalConductivity,
    ThermophysicalProperties,
};

/// Raw scalar representation used only as the code-generation reference.
#[derive(Clone, Copy)]
#[repr(C)]
pub struct RawProperties {
    density: f64,
    heat_capacity: f64,
    conductivity: f64,
}

#[inline(never)]
#[must_use]
/// Evaluate the raw scalar thermal-diffusivity expression.
pub fn raw_diffusivity(properties: RawProperties) -> f64 {
    properties.conductivity / (properties.density * properties.heat_capacity)
}

#[inline(never)]
#[must_use]
/// Evaluate the typed Proteus thermal-diffusivity expression.
pub fn typed_diffusivity(properties: ThermophysicalProperties<f64>) -> f64 {
    properties.thermal_diffusivity().into_base()
}

#[inline(never)]
#[must_use]
/// Evaluate a raw scalar relative-linear response.
pub fn raw_linear_response(base: f64, coefficient: f64, delta: f64) -> f64 {
    base * coefficient.mul_add(delta, 1.0)
}

#[inline(never)]
#[must_use]
/// Evaluate the typed Proteus relative-linear response.
pub fn typed_linear_response(
    base: f64,
    response: LinearResponse<f64>,
    delta: TemperatureDifference<f64>,
) -> f64 {
    base * response.factor(delta).into_base()
}

#[test]
fn typed_and_raw_diffusivity_have_identical_value_semantics() {
    let density = std::hint::black_box(1_000.0);
    let heat_capacity = std::hint::black_box(4_000.0);
    let conductivity = std::hint::black_box(0.6);
    let typed_density =
        MassDensity::new(DensityQuantity::from_base(density)).expect("fixture density is positive");
    let typed_heat_capacity =
        SpecificHeatCapacity::new(HeatCapacityQuantity::from_base(heat_capacity))
            .expect("fixture heat capacity is positive");
    let typed_conductivity =
        ThermalConductivity::new(ConductivityQuantity::from_base(conductivity))
            .expect("fixture conductivity is non-negative");
    let properties =
        ThermophysicalProperties::new(typed_density, typed_heat_capacity, typed_conductivity)
            .expect("fixture density is positive");
    let raw = RawProperties {
        density,
        heat_capacity,
        conductivity,
    };

    assert_eq!(
        typed_diffusivity(properties).to_bits(),
        raw_diffusivity(raw).to_bits()
    );
}

#[test]
fn typed_and_raw_linear_responses_have_identical_value_semantics() {
    let base = std::hint::black_box(4_000.0);
    let coefficient = std::hint::black_box(0.001);
    let delta = std::hint::black_box(8.0);
    let response = LinearResponse::new(ReciprocalTemperature::from_base(coefficient))
        .expect("fixture coefficient is finite");

    assert_eq!(
        typed_linear_response(base, response, TemperatureDifference::from_base(delta)).to_bits(),
        raw_linear_response(base, coefficient, delta).to_bits()
    );
}
