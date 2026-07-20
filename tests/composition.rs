//! Material identity and static-law composition regressions.

use std::string::String;

use aequitas::systems::si::quantities::{
    MassDensity as DensityQuantity, SpecificHeatCapacity as HeatCapacityQuantity,
    ThermalConductivity as ConductivityQuantity,
};
use proteus::{
    ConstantLaw, MassDensity, Material, NoState, SpecificHeatCapacity, ThermalConductivity,
    ThermophysicalProperties,
};

fn reference() -> ThermophysicalProperties<f64> {
    ThermophysicalProperties::new(
        MassDensity::new(DensityQuantity::from_base(1_000.0)).expect("positive"),
        SpecificHeatCapacity::new(HeatCapacityQuantity::from_base(4_000.0)).expect("positive"),
        ThermalConductivity::new(ConductivityQuantity::from_base(0.6)).expect("non-negative"),
    )
    .expect("positive density")
}

#[test]
fn borrowed_material_name_preserves_input_storage() {
    let name = "reference liquid";
    let material = Material::borrowed(name, ConstantLaw::new(reference()));

    assert_eq!(material.name().as_ptr(), name.as_ptr());
    assert_eq!(
        material.properties(NoState).expect("infallible"),
        reference()
    );
}

#[test]
fn static_routing_types_have_no_runtime_footprint() {
    assert_eq!(core::mem::size_of::<NoState>(), 0);
}

#[test]
fn runtime_material_owns_its_name_through_the_same_contract() {
    let material = Material::owned(
        String::from("patient-specific"),
        ConstantLaw::new(reference()),
    );

    assert_eq!(material.name(), "patient-specific");
    assert_eq!(
        material.properties(NoState).expect("infallible"),
        reference()
    );
}
