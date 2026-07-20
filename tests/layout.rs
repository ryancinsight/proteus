//! Representation and allocation invariants.

use core::mem::{align_of, size_of};

use aequitas::systems::si::quantities::{
    MassDensity as DensityQuantity, SpecificHeatCapacity as HeatCapacityQuantity,
    ThermalConductivity as ConductivityQuantity,
};
use proteus::{
    ConstantLaw, MassDensity, Material, NoState, SpecificHeatCapacity, ThermalConductivity,
    ThermophysicalProperties,
};
use stats_alloc::{INSTRUMENTED_SYSTEM, Region, StatsAlloc};

#[global_allocator]
static ALLOCATOR: &StatsAlloc<std::alloc::System> = &INSTRUMENTED_SYSTEM;

#[test]
fn property_newtypes_are_transparent_over_their_quantities() {
    assert_eq!(
        size_of::<MassDensity<f64>>(),
        size_of::<DensityQuantity<f64>>()
    );
    assert_eq!(
        align_of::<MassDensity<f64>>(),
        align_of::<DensityQuantity<f64>>()
    );
    assert_eq!(
        size_of::<SpecificHeatCapacity<f32>>(),
        size_of::<HeatCapacityQuantity<f32>>()
    );
    assert_eq!(
        size_of::<ThermalConductivity<f64>>(),
        size_of::<ConductivityQuantity<f64>>()
    );
}

#[test]
fn borrowed_material_construction_and_evaluation_allocate_nothing() {
    let properties = ThermophysicalProperties::try_from_quantities(
        DensityQuantity::from_base(1_000.0_f64),
        HeatCapacityQuantity::from_base(4_000.0_f64),
        ConductivityQuantity::from_base(0.6_f64),
    )
    .expect("valid fixture");

    let region = Region::new(ALLOCATOR);
    let material = Material::borrowed("reference", ConstantLaw::new(properties));
    let evaluated = material.properties(NoState).expect("infallible");
    let change = region.change();

    assert_eq!(evaluated, properties);
    assert_eq!(change.allocations, 0);
    assert_eq!(change.reallocations, 0);
    assert_eq!(change.deallocations, 0);
}
