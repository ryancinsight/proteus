//! Material identity and static-law composition regressions.

use std::string::String;

use eunomia::RealField;
use proteus::{ConstantLaw, Material, NoState};

mod common;

fn assert_borrowed_material_name_preserves_input_storage<T: RealField>() {
    let name = "reference liquid";
    let material = Material::borrowed(name, ConstantLaw::new(common::reference::<T>()));

    assert_eq!(material.name().as_ptr(), name.as_ptr());
    assert_eq!(
        material.properties(NoState).expect("infallible"),
        common::reference::<T>()
    );
}

#[test]
fn borrowed_material_name_preserves_input_storage_for_every_supported_real_scalar() {
    assert_borrowed_material_name_preserves_input_storage::<f32>();
    assert_borrowed_material_name_preserves_input_storage::<f64>();
}

#[test]
fn static_routing_types_have_no_runtime_footprint() {
    assert_eq!(core::mem::size_of::<NoState>(), 0);
}

fn assert_runtime_material_owns_its_name_through_the_same_contract<T: RealField>() {
    let material = Material::owned(
        String::from("patient-specific"),
        ConstantLaw::new(common::reference::<T>()),
    );

    assert_eq!(material.name(), "patient-specific");
    assert_eq!(
        material.properties(NoState).expect("infallible"),
        common::reference::<T>()
    );
}

#[test]
fn runtime_material_owns_its_name_through_the_same_contract_for_every_supported_real_scalar() {
    assert_runtime_material_owns_its_name_through_the_same_contract::<f32>();
    assert_runtime_material_owns_its_name_through_the_same_contract::<f64>();
}
