//! Construct and evaluate a constant thermophysical material.

use aequitas::systems::si::quantities::{
    MassDensity as DensityQuantity, SpecificHeatCapacity as HeatCapacityQuantity,
    ThermalConductivity as ConductivityQuantity,
};
use proteus::{
    ConstantLaw, MassDensity, Material, NoState, SpecificHeatCapacity, ThermalConductivity,
    ThermophysicalProperties,
};

fn main() {
    let density =
        MassDensity::new(DensityQuantity::from_base(1_000.0_f64)).expect("positive density");
    let heat_capacity = SpecificHeatCapacity::new(HeatCapacityQuantity::from_base(4_000.0_f64))
        .expect("positive heat capacity");
    let conductivity = ThermalConductivity::new(ConductivityQuantity::from_base(0.6_f64))
        .expect("non-negative conductivity");
    let properties =
        ThermophysicalProperties::new(density, heat_capacity, conductivity).expect("positive rho");
    let material = Material::borrowed("reference liquid", ConstantLaw::new(properties));

    let evaluated = material
        .properties(NoState)
        .expect("constant law is infallible");

    assert_eq!(material.name(), "reference liquid");
    assert_eq!(
        evaluated.thermal_diffusivity().into_base().to_bits(),
        1.5e-7_f64.to_bits()
    );
}
