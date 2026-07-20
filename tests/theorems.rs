//! Executable evidence for thermophysical laws.

use aequitas::systems::si::quantities::{
    MassDensity as DensityQuantity, SpecificHeatCapacity as HeatCapacityQuantity,
    ThermalConductivity as ConductivityQuantity,
};
use eunomia::RealField;
use proteus::{MassDensity, SpecificHeatCapacity, ThermalConductivity, ThermophysicalProperties};

fn properties<T: RealField>(
    density: T,
    heat_capacity: T,
    conductivity: T,
) -> ThermophysicalProperties<T> {
    ThermophysicalProperties::new(
        MassDensity::new(DensityQuantity::from_base(density)).expect("positive density"),
        SpecificHeatCapacity::new(HeatCapacityQuantity::from_base(heat_capacity))
            .expect("positive heat capacity"),
        ThermalConductivity::new(ConductivityQuantity::from_base(conductivity))
            .expect("non-negative conductivity"),
    )
    .expect("positive continuum density")
}

fn assert_diffusivity_law<T: RealField>() {
    let material = properties(T::from_f64(1_000.0), T::from_f64(4_000.0), T::from_f64(0.6));
    let actual = material.thermal_diffusivity().into_base();
    let expected = T::from_f64(0.6) / (T::from_f64(1_000.0) * T::from_f64(4_000.0));
    assert_eq!(actual, expected);
}

#[test]
fn diffusivity_definition_holds_for_every_supported_real_scalar() {
    assert_diffusivity_law::<f32>();
    assert_diffusivity_law::<f64>();
}

proptest::proptest! {
    #[test]
    fn diffusivity_is_non_negative(
        density in 1e-3_f64..2e4,
        heat_capacity in 1e-3_f64..1e5,
        conductivity in 0.0_f64..1e4,
    ) {
        let actual = properties(density, heat_capacity, conductivity)
            .thermal_diffusivity()
            .into_base();
        proptest::prop_assert!(actual.is_finite());
        proptest::prop_assert!(actual >= 0.0);
    }

    #[test]
    fn diffusivity_scales_linearly_with_conductivity(
        density in 1.0_f64..2e4,
        heat_capacity in 1.0_f64..1e5,
        conductivity in 0.0_f64..1e3,
        factor in 0.0_f64..10.0,
    ) {
        let base = properties(density, heat_capacity, conductivity)
            .thermal_diffusivity()
            .into_base();
        let scaled = properties(density, heat_capacity, conductivity * factor)
            .thermal_diffusivity()
            .into_base();
        let expected = base * factor;
        let rounding = 8.0 * f64::EPSILON * expected.abs().max(1.0);
        proptest::prop_assert!((scaled - expected).abs() <= rounding);
    }
}
