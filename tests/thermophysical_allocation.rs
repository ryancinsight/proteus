//! Allocation contracts for ordinary and recovered public-law evaluation.

use eunomia::RealField;
use mnemosyne::counting::{AllocationDelta, CountingAllocator};

mod common;

#[global_allocator]
static ALLOCATOR: CountingAllocator<std::alloc::System> =
    CountingAllocator::new(std::alloc::System);

fn verify<T: RealField>(exponent: i32) {
    for scale in [-exponent, 0, exponent] {
        let property = T::ONE.scale_binary(scale);
        let material = std::hint::black_box(common::properties(property, property, property));
        let (result, allocations) = mnemosyne::counting::measure(|| {
            std::hint::black_box(material.thermal_diffusivity().into_base())
        });
        assert_eq!(result, T::ONE.scale_binary(-scale));
        assert_eq!(allocations, AllocationDelta::default());
    }
    assert!(common::reference::<T>().thermal_diffusivity().into_base() > T::ZERO);
}

#[test]
fn all_native_range_paths_allocate_nothing() {
    verify::<f32>(100);
    verify::<f64>(600);
    verify::<eunomia::F32>(100);
    verify::<eunomia::F64>(600);
}
