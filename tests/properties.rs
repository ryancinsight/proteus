//! Property validity-boundary regressions.

use aequitas::systems::si::quantities::{
    MassDensity as DensityQuantity, SpecificHeatCapacity as HeatCapacityQuantity,
    ThermalConductivity as ConductivityQuantity,
};
use eunomia::RealField;
use proteus::{
    MassDensity, PropertyConstraint, PropertyKind, SpecificHeatCapacity, ThermalConductivity,
    ThermophysicalProperties,
};

fn assert_validity_boundaries_reject_nonphysical_values<T: RealField>() {
    let density = MassDensity::new(DensityQuantity::from_base(-T::from_f64(1.0)))
        .expect_err("negative density is invalid");
    assert_eq!(density.kind(), PropertyKind::MassDensity);
    assert_eq!(density.constraint(), PropertyConstraint::FiniteNonNegative);
    assert_eq!(*density.value(), -T::from_f64(1.0));

    let density = MassDensity::new(DensityQuantity::from_base(T::infinity()))
        .expect_err("infinite density is invalid");
    assert_eq!(density.kind(), PropertyKind::MassDensity);
    assert_eq!(density.constraint(), PropertyConstraint::FiniteNonNegative);
    assert_eq!(*density.value(), T::infinity());

    let heat_capacity =
        SpecificHeatCapacity::new(HeatCapacityQuantity::from_base(T::from_f64(0.0)))
            .expect_err("zero heat capacity is invalid");
    assert_eq!(heat_capacity.kind(), PropertyKind::SpecificHeatCapacity);
    assert_eq!(
        heat_capacity.constraint(),
        PropertyConstraint::FinitePositive
    );
    assert_eq!(*heat_capacity.value(), T::from_f64(0.0));

    // A small finite negative at the scalar's own precision: minus the gap
    // between 1 and the next representable value.
    let conductivity = ThermalConductivity::new(ConductivityQuantity::from_base(-T::EPSILON))
        .expect_err("negative conductivity is invalid");
    assert_eq!(conductivity.kind(), PropertyKind::ThermalConductivity);
    assert_eq!(
        conductivity.constraint(),
        PropertyConstraint::FiniteNonNegative
    );
    assert_eq!(*conductivity.value(), -T::EPSILON);

    let conductivity = ThermalConductivity::new(ConductivityQuantity::from_base(T::nan()))
        .expect_err("NaN conductivity is invalid");
    assert_eq!(conductivity.kind(), PropertyKind::ThermalConductivity);
    assert_eq!(
        conductivity.constraint(),
        PropertyConstraint::FiniteNonNegative
    );
    assert!(conductivity.value().is_nan());
}

#[test]
fn validity_boundaries_reject_nonphysical_values_for_every_supported_real_scalar() {
    assert_validity_boundaries_reject_nonphysical_values::<f32>();
    assert_validity_boundaries_reject_nonphysical_values::<f64>();
}

fn assert_zero_density_cannot_enter_continuum_bundle<T: RealField>() {
    let density =
        MassDensity::new(DensityQuantity::from_base(T::from_f64(0.0))).expect("vacuum is valid");
    let heat_capacity =
        SpecificHeatCapacity::new(HeatCapacityQuantity::from_base(T::from_f64(1.0)))
            .expect("positive");
    let conductivity = ThermalConductivity::new(ConductivityQuantity::from_base(T::from_f64(0.0)))
        .expect("adiabatic");

    let error = ThermophysicalProperties::new(density, heat_capacity, conductivity)
        .expect_err("continuum density must be positive");

    assert_eq!(error.kind(), PropertyKind::MassDensity);
    assert_eq!(error.constraint(), PropertyConstraint::FinitePositive);
    assert_eq!(*error.value(), T::from_f64(0.0));
}

#[test]
fn zero_density_cannot_enter_continuum_bundle_for_every_supported_real_scalar() {
    assert_zero_density_cannot_enter_continuum_bundle::<f32>();
    assert_zero_density_cannot_enter_continuum_bundle::<f64>();
}
