//! The validity domain's boundaries, and inputs chosen to sit just outside
//! them.

use super::*;

#[test]
fn auxetic_solids_are_inside_the_validity_domain() {
    // nu < 0 yields lambda < 0; rejecting it would exclude real materials.
    let state = moduli::<f64>(5e9, -0.25);
    assert!(*state.lame_lambda().as_base() < 0.0);
    assert!(state.bulk_modulus().into_base() > 0.0);
    assert_close(state.poissons_ratio().into_base(), -0.25, "auxetic nu");
}

#[test]
fn poisson_ratio_bounds_are_open_and_rejected_exactly() {
    for &poisson in &[0.5, -1.0, 0.75, -1.5] {
        let rejected = IsotropicModuli::from_young_poisson(
            Pressure::from_base(200e9_f64),
            Dimensionless::from_base(poisson),
        )
        .expect_err("nu outside (-1, 1/2) must be rejected");
        assert_eq!(rejected.quantity(), ElasticQuantity::PoissonsRatio);
        assert_eq!(rejected.constraint(), ElasticConstraint::PoissonRatioRange);
    }
}

#[test]
fn non_positive_youngs_modulus_is_rejected() {
    for &young in &[0.0, -1.0] {
        let rejected = IsotropicModuli::from_young_poisson(
            Pressure::from_base(young),
            Dimensionless::from_base(0.3_f64),
        )
        .expect_err("E must be positive");
        assert_eq!(rejected.quantity(), ElasticQuantity::YoungsModulus);
        assert_eq!(rejected.constraint(), ElasticConstraint::FinitePositive);
    }
}

#[test]
fn non_positive_shear_modulus_is_rejected() {
    for &mu in &[0.0, -1.0] {
        let rejected =
            IsotropicModuli::from_lame(Pressure::from_base(1e9_f64), Pressure::from_base(mu))
                .expect_err("mu must be positive");
        assert_eq!(rejected.quantity(), ElasticQuantity::ShearModulus);
        assert_eq!(rejected.constraint(), ElasticConstraint::FinitePositive);
    }
}

#[test]
fn non_positive_bulk_modulus_is_rejected() {
    // lambda = -2 mu / 3 puts K exactly at zero; the domain is open there.
    let mu = 3e9_f64;
    let rejected = IsotropicModuli::from_lame(
        Pressure::from_base(-2.0 * mu / 3.0),
        Pressure::from_base(mu),
    )
    .expect_err("K must be positive");
    assert_eq!(rejected.constraint(), ElasticConstraint::PositiveDefinite);
}

#[test]
fn wave_speeds_violating_positive_definiteness_are_rejected() {
    // c_p^2 = (4/3) c_s^2 sits exactly on the excluded boundary.
    let c_s = 3000.0_f64;
    let c_p = (4.0 / 3.0 * c_s * c_s).sqrt();
    let rejected = IsotropicModuli::from_wave_speeds(
        Velocity::from_base(c_p),
        Velocity::from_base(c_s),
        DensityQuantity::from_base(8000.0),
    )
    .expect_err("c_p^2 must exceed (4/3) c_s^2");
    assert_eq!(rejected.constraint(), ElasticConstraint::PositiveDefinite);
}

#[test]
fn non_finite_inputs_are_rejected_at_every_constructor() {
    for &bad in &[f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        assert!(
            IsotropicModuli::from_young_poisson(
                Pressure::from_base(bad),
                Dimensionless::from_base(0.3_f64),
            )
            .is_err()
        );
        assert!(
            IsotropicModuli::from_young_poisson(
                Pressure::from_base(200e9_f64),
                Dimensionless::from_base(bad),
            )
            .is_err()
        );
        assert!(
            IsotropicModuli::from_lame(Pressure::from_base(bad), Pressure::from_base(1e9_f64))
                .is_err()
        );
        assert!(
            IsotropicModuli::from_lame(Pressure::from_base(1e9_f64), Pressure::from_base(bad))
                .is_err()
        );
        assert!(
            IsotropicModuli::from_wave_speeds(
                Velocity::from_base(bad),
                Velocity::from_base(3000.0_f64),
                DensityQuantity::from_base(8000.0_f64),
            )
            .is_err()
        );
    }
}

#[test]
fn non_positive_density_is_rejected_by_the_speed_accessors() {
    let state = moduli::<f64>(200e9, 0.3);
    for &rho in &[0.0, -1.0, f64::NAN] {
        let density = DensityQuantity::from_base(rho);
        let rejected = state
            .shear_wave_speed(density)
            .expect_err("density must be positive");
        assert_eq!(rejected.quantity(), ElasticQuantity::MassDensity);
        assert!(state.compressional_wave_speed(density).is_err());
    }
}
