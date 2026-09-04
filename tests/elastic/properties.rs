//! Property tests: the laws that hold for every admissible input, rather
//! than for the enumerated cases beside them.

use super::*;

proptest::proptest! {
    #[test]
    fn engineering_round_trip_holds_across_the_domain(
        young in 1e3_f64..5e12,
        poisson in -0.99_f64..0.49,
    ) {
        let state = IsotropicModuli::from_young_poisson(
            Pressure::from_base(young),
            Dimensionless::from_base(poisson),
        ).expect("inside the domain");

        let condition = round_trip_condition(poisson);
        let bound = f64::EPSILON * RELATIVE_TOLERANCE_ULP * condition;

        let relative = (state.youngs_modulus().into_base() - young).abs() / young;
        proptest::prop_assert!(
            relative <= bound,
            "E relative error {relative:e} exceeds the condition-scaled bound {bound:e} \
             (nu = {poisson}, kappa = {condition})"
        );

        let nu_error = (state.poissons_ratio().into_base() - poisson).abs();
        proptest::prop_assert!(
            nu_error <= bound * poisson.abs().max(1.0),
            "nu absolute error {nu_error:e} exceeds the condition-scaled bound (kappa = {condition})"
        );
    }

    #[test]
    fn accepted_states_are_always_positive_definite(
        young in 1e3_f64..5e12,
        poisson in -0.99_f64..0.49,
    ) {
        let state = IsotropicModuli::from_young_poisson(
            Pressure::from_base(young),
            Dimensionless::from_base(poisson),
        ).expect("inside the domain");

        proptest::prop_assert!(*state.shear_modulus().as_base() > 0.0);
        proptest::prop_assert!(state.bulk_modulus().into_base() > 0.0);
        proptest::prop_assert!(state.p_wave_modulus().into_base() > 0.0);
    }

    #[test]
    fn wave_speed_round_trip_holds_across_the_domain(
        c_s in 1.0_f64..6000.0,
        ratio in 1.16_f64..4.0,
        rho in 1.0_f64..2e4,
    ) {
        let c_p = c_s * ratio;
        let density = DensityQuantity::from_base(rho);
        let state = IsotropicModuli::from_wave_speeds(
            Velocity::from_base(c_p),
            Velocity::from_base(c_s),
            density,
        ).expect("ratio exceeds sqrt(4/3)");

        let recovered_s = state.shear_wave_speed(density).expect("positive").into_base();
        let recovered_p = state.compressional_wave_speed(density).expect("positive").into_base();

        proptest::prop_assert!((recovered_s - c_s).abs() <= f64::EPSILON * RELATIVE_TOLERANCE_ULP * c_s);
        proptest::prop_assert!((recovered_p - c_p).abs() <= f64::EPSILON * RELATIVE_TOLERANCE_ULP * c_p);
    }

    #[test]
    fn density_never_affects_the_moduli(
        young in 1e3_f64..5e12,
        poisson in -0.99_f64..0.49,
        rho_a in 1.0_f64..2e4,
        rho_b in 1.0_f64..2e4,
    ) {
        // Moduli are density-free; only the derived speeds carry rho.
        let state = IsotropicModuli::from_young_poisson(
            Pressure::from_base(young),
            Dimensionless::from_base(poisson),
        ).expect("inside the domain");

        let speed_a = state.shear_wave_speed(DensityQuantity::from_base(rho_a)).expect("positive");
        let speed_b = state.shear_wave_speed(DensityQuantity::from_base(rho_b)).expect("positive");

        // c_s scales as rho^(-1/2): c_s(a) * sqrt(a) == c_s(b) * sqrt(b).
        let invariant_a = speed_a.into_base() * rho_a.sqrt();
        let invariant_b = speed_b.into_base() * rho_b.sqrt();
        let scale = invariant_a.abs().max(invariant_b.abs());
        proptest::prop_assert!(
            (invariant_a - invariant_b).abs() <= f64::EPSILON * RELATIVE_TOLERANCE_ULP * scale
        );
    }
}

#[test]
fn mass_density_newtype_is_reused_rather_than_redeclared() {
    // The catalog composes the existing property newtype; no parallel density
    // type exists in the elastic module.
    let solid = NamedIsotropicSolid::TitaniumGrade5
        .solid::<f64>()
        .expect("valid");
    let direct = MassDensity::new(DensityQuantity::from_base(4430.0_f64)).expect("positive");
    assert_eq!(*solid.density(), direct);
}
