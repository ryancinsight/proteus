//! Evaluate a material with independent temperature responses.
//!
//! The values are round arithmetic-fixture numbers that exercise the API;
//! they are not measured reference data for any physical material.

#![expect(clippy::print_stdout, reason = "example stdout is the deliverable")]

extern crate aequitas;
extern crate proteus;

use aequitas::systems::si::quantities::{
    MassDensity as DensityQuantity, ReciprocalTemperature, ReciprocalTemperatureSquared,
    SpecificHeatCapacity as HeatCapacityQuantity, ThermalConductivity as ConductivityQuantity,
    ThermodynamicTemperature,
};
use proteus::{
    ConstantResponse, LinearResponse, MassDensity, Material, QuadraticResponse, ResponseSet,
    SpecificHeatCapacity, TemperatureLaw, TemperatureLawError, TemperatureValidity,
    ThermalConductivity, ThermophysicalProperties,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let reference = ThermophysicalProperties::new(
        MassDensity::new(DensityQuantity::from_base(1_050.0_f64))?,
        SpecificHeatCapacity::new(HeatCapacityQuantity::from_base(3_600.0))?,
        ThermalConductivity::new(ConductivityQuantity::from_base(0.5))?,
    )?;
    let responses = ResponseSet::new(
        ConstantResponse,
        LinearResponse::new(ReciprocalTemperature::from_base(2.0e-4))?,
        QuadraticResponse::new(
            ReciprocalTemperature::from_base(1.0e-3),
            ReciprocalTemperatureSquared::from_base(0.0),
        )?,
    );
    // The fixture declares its own inclusive calibration domain; evaluation
    // outside it is rejected instead of extrapolated.
    let validity = TemperatureValidity::bounded(
        ThermodynamicTemperature::from_base(280.0),
        ThermodynamicTemperature::from_base(340.0),
    )?;
    let law = TemperatureLaw::with_validity(
        reference,
        ThermodynamicTemperature::from_base(310.15),
        validity,
        responses,
    )?;
    let fixture = Material::borrowed("arithmetic fixture", law);
    let current = ThermodynamicTemperature::from_base(318.15);
    let properties = fixture.properties(&current)?;

    let outside = ThermodynamicTemperature::from_base(400.0);
    if !matches!(
        fixture.properties(&outside),
        Err(TemperatureLawError::OutsideValidityDomain { .. })
    ) {
        return Err("evaluation at 400 K must leave the calibration domain".into());
    }

    println!(
        "{}: k={} W/(m·K), cp={} J/(kg·K)",
        fixture.name(),
        properties.thermal_conductivity().quantity().into_base(),
        properties.specific_heat_capacity().quantity().into_base(),
    );
    Ok(())
}
