//! Property validity-boundary regressions.

use aequitas::systems::si::quantities::{
    MassDensity as DensityQuantity, SpecificHeatCapacity as HeatCapacityQuantity,
    ThermalConductivity as ConductivityQuantity,
};
use proteus::{
    MassDensity, PropertyConstraint, PropertyKind, SpecificHeatCapacity, ThermalConductivity,
    ThermophysicalProperties,
};

#[test]
fn validity_boundaries_reject_nonphysical_values() {
    let density = MassDensity::new(DensityQuantity::from_base(-1.0_f64))
        .expect_err("negative density is invalid");
    assert_eq!(density.kind(), PropertyKind::MassDensity);
    assert_eq!(density.constraint(), PropertyConstraint::FiniteNonNegative);
    assert_eq!(density.value().to_bits(), (-1.0_f64).to_bits());

    assert!(MassDensity::new(DensityQuantity::from_base(f64::INFINITY)).is_err());
    assert!(SpecificHeatCapacity::new(HeatCapacityQuantity::from_base(0.0_f64)).is_err());
    assert!(ThermalConductivity::new(ConductivityQuantity::from_base(-f64::EPSILON)).is_err());
    assert!(ThermalConductivity::new(ConductivityQuantity::from_base(f64::NAN)).is_err());
}

#[test]
fn zero_density_cannot_enter_continuum_bundle() {
    let density = MassDensity::new(DensityQuantity::from_base(0.0_f64)).expect("vacuum is valid");
    let heat_capacity =
        SpecificHeatCapacity::new(HeatCapacityQuantity::from_base(1.0_f64)).expect("positive");
    let conductivity =
        ThermalConductivity::new(ConductivityQuantity::from_base(0.0_f64)).expect("adiabatic");

    let error = ThermophysicalProperties::new(density, heat_capacity, conductivity)
        .expect_err("continuum density must be positive");

    assert_eq!(error.kind(), PropertyKind::MassDensity);
    assert_eq!(error.constraint(), PropertyConstraint::FinitePositive);
}
