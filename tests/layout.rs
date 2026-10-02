//! Representation and allocation invariants.
//!
//! Allocation windows count allocations made by the calling thread only. A
//! process-wide counter is invalid here: libtest runs the test body on a
//! spawned thread while its main thread keeps inserting the running test into
//! its bookkeeping collections, so a process-wide window occasionally absorbs
//! those allocations and fails for reasons unrelated to the code under test.

use core::mem::{align_of, size_of};

use aequitas::systems::si::quantities::{
    MassDensity as DensityQuantity, SpecificHeatCapacity as HeatCapacityQuantity,
    ThermalConductivity as ConductivityQuantity,
};
use mnemosyne::counting::{AllocationDelta, CountingAllocator};
use proteus::{
    ConstantLaw, MassDensity, Material, NoState, SpecificHeatCapacity, ThermalConductivity,
};

mod common;

#[global_allocator]
static ALLOCATOR: CountingAllocator<std::alloc::System> =
    CountingAllocator::new(std::alloc::System);

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
    let properties = common::reference::<f64>();

    let (evaluated, change) = mnemosyne::counting::measure(|| {
        let material = Material::borrowed("reference", ConstantLaw::new(properties));
        material.properties(NoState).expect("infallible")
    });

    assert_eq!(evaluated, properties);
    assert_eq!(change, AllocationDelta::default());
}
