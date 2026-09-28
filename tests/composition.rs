//! Material identity and static-law composition regressions.

use std::string::String;

use proteus::{ConstantLaw, Material, NoState};

mod common;

#[test]
fn borrowed_material_name_preserves_input_storage() {
    let name = "reference liquid";
    let material = Material::borrowed(name, ConstantLaw::new(common::reference::<f64>()));

    assert_eq!(material.name().as_ptr(), name.as_ptr());
    assert_eq!(
        material.properties(NoState).expect("infallible"),
        common::reference::<f64>()
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
        ConstantLaw::new(common::reference::<f64>()),
    );

    assert_eq!(material.name(), "patient-specific");
    assert_eq!(
        material.properties(NoState).expect("infallible"),
        common::reference::<f64>()
    );
}
