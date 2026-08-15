//! Evaluate a material with independent temperature responses.

use aequitas::systems::si::quantities::{
    MassDensity as DensityQuantity, ReciprocalTemperature, ReciprocalTemperatureSquared,
    SpecificHeatCapacity as HeatCapacityQuantity, ThermalConductivity as ConductivityQuantity,
    ThermodynamicTemperature,
};
use proteus::{
    ConstantResponse, LinearResponse, MassDensity, Material, QuadraticResponse, ResponseSet,
    SpecificHeatCapacity, TemperatureLaw, ThermalConductivity, ThermophysicalProperties,
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
    let law = TemperatureLaw::new(
        reference,
        ThermodynamicTemperature::from_base(310.15),
        responses,
    )?;
    let tissue = Material::borrowed("generic tissue", law);
    let current = ThermodynamicTemperature::from_base(318.15);
    let properties = tissue.properties(&current)?;

    println!(
        "{}: k={} W/(m·K), cp={} J/(kg·K)",
        tissue.name(),
        properties.thermal_conductivity().quantity().into_base(),
        properties.specific_heat_capacity().quantity().into_base(),
    );
    Ok(())
}
