//! Executable evidence for the isotropic-elastic conversion contract.
//!
//! Tolerances are derived, never tuned. Each conversion chain below performs a
//! bounded number of native multiplications and divisions (at most eight), so
//! the accumulated relative error is bounded by `8 * eps` for round-to-nearest
//! arithmetic. `RELATIVE_TOLERANCE_ULP = 16` carries one binary order of
//! headroom above that bound and is applied relative to the compared value.
//!
//! Where an identity is ill-conditioned the flat bound is analytically wrong,
//! so it is scaled by the cancellation condition number of that identity —
//! see [`round_trip_condition`]. Widening a bound to absorb an observed error
//! without such a derivation would be gaming, not evidence.

use aequitas::systems::si::quantities::{
    Dimensionless, MassDensity as DensityQuantity, Pressure, Velocity,
};
use eunomia::{NumericElement, RealField};
use proteus::{
    ElasticConstraint, ElasticQuantity, IsotropicModuli, NamedIsotropicSolid, property::MassDensity,
};

const RELATIVE_TOLERANCE_ULP: f64 = 16.0;

/// Amplification factor for a `(E, nu) -> (lambda, mu) -> (E, nu)` round trip.
///
/// The inverse map divides `mu (3 lambda + 2 mu)` by `lambda + mu`. Writing
/// `r = lambda / mu = 2 nu / (1 - 2 nu)`, both expressions are sums of
/// opposite-signed terms whenever `nu < 0`, so each carries a cancellation
/// condition number:
///
/// ```text
/// kappa_num = (3|r| + 2) / |3r + 2|      kappa_den = (|r| + 1) / |r + 1|
/// ```
///
/// As `nu -> -1` the numerator condition diverges (`3r + 2 -> 0` at
/// `r = -2/3`), so a flat ULP bound is analytically wrong near that edge. The
/// relative round-trip error is bounded by `eps * ULP * (kappa_num +
/// kappa_den)`; for `nu >= 0` both factors are exactly 1 and the bound reduces
/// to the flat `eps * ULP`.
fn round_trip_condition(poisson: f64) -> f64 {
    let ratio = 2.0 * poisson / (1.0 - 2.0 * poisson);
    let numerator = 3.0f64.mul_add(ratio.abs(), 2.0) / 3.0f64.mul_add(ratio, 2.0).abs();
    let denominator = (ratio.abs() + 1.0) / (ratio + 1.0).abs();
    numerator + denominator
}

fn assert_close<T: RealField>(actual: T, expected: T, context: &str) {
    assert_close_scaled(actual, expected, 1.0, context);
}

fn assert_close_scaled<T: RealField>(actual: T, expected: T, condition: f64, context: &str) {
    let tolerance = <T as RealField>::EPSILON
        * T::from_f64(RELATIVE_TOLERANCE_ULP * condition)
        * if expected == <T as NumericElement>::ZERO {
            <T as NumericElement>::ONE
        } else {
            expected.abs()
        };
    let error = (actual - expected).abs();
    assert!(
        error <= tolerance,
        "{context}: error exceeds the derived {RELATIVE_TOLERANCE_ULP}-ULP bound"
    );
}

fn moduli<T: RealField>(young: f64, poisson: f64) -> IsotropicModuli<T> {
    IsotropicModuli::from_young_poisson(
        Pressure::from_base(T::from_f64(young)),
        Dimensionless::from_base(T::from_f64(poisson)),
    )
    .expect("published constants lie inside the positive-definite domain")
}

// ---------------------------------------------------------------------------
// Round-trip theorems
// ---------------------------------------------------------------------------

fn assert_engineering_round_trip<T: RealField>(young: f64, poisson: f64) {
    let state = moduli::<T>(young, poisson);
    let condition = round_trip_condition(poisson);
    assert_close_scaled(
        state.youngs_modulus().into_base(),
        T::from_f64(young),
        condition,
        "E round-trip",
    );
    assert_close_scaled(
        state.poissons_ratio().into_base(),
        T::from_f64(poisson),
        condition,
        "nu round-trip",
    );
}

#[test]
fn engineering_pair_round_trips_for_every_supported_real_scalar() {
    for &(young, poisson) in &[
        (200e9, 0.30),
        (70e9, 0.33),
        (1e6, 0.0),
        (5e9, -0.25),
        (3e10, 0.49),
    ] {
        assert_engineering_round_trip::<f32>(young, poisson);
        assert_engineering_round_trip::<f64>(young, poisson);
    }
}

fn assert_wave_speed_round_trip<T: RealField>(c_p: f64, c_s: f64, rho: f64) {
    let density = DensityQuantity::from_base(T::from_f64(rho));
    let state = IsotropicModuli::from_wave_speeds(
        Velocity::from_base(T::from_f64(c_p)),
        Velocity::from_base(T::from_f64(c_s)),
        density,
    )
    .expect("speeds satisfy the positive-definite condition");

    assert_close(
        state
            .compressional_wave_speed(density)
            .expect("positive density")
            .into_base(),
        T::from_f64(c_p),
        "c_p round-trip",
    );
    assert_close(
        state
            .shear_wave_speed(density)
            .expect("positive density")
            .into_base(),
        T::from_f64(c_s),
        "c_s round-trip",
    );
}

#[test]
fn wave_speeds_round_trip_for_every_supported_real_scalar() {
    for &(c_p, c_s, rho) in &[
        (5960.0, 3200.0, 8000.0),
        (1500.0, 80.0, 1000.0),
        (6070.0, 3100.0, 4430.0),
    ] {
        assert_wave_speed_round_trip::<f32>(c_p, c_s, rho);
        assert_wave_speed_round_trip::<f64>(c_p, c_s, rho);
    }
}

// ---------------------------------------------------------------------------
// Definitional identities
// ---------------------------------------------------------------------------

#[test]
fn shear_modulus_matches_the_isotropic_definition() {
    // mu = E / (2 (1 + nu)) — the identity CFDrs carried as a default method.
    let state = moduli::<f64>(200e9, 0.3);
    assert_close(
        *state.shear_modulus().as_base(),
        200e9 / (2.0 * 1.3),
        "mu definition",
    );
}

#[test]
fn bulk_modulus_matches_the_isotropic_definition() {
    let state = moduli::<f64>(200e9, 0.3);
    let lambda = *state.lame_lambda().as_base();
    let mu = *state.shear_modulus().as_base();
    assert_close(
        state.bulk_modulus().into_base(),
        lambda + 2.0 * mu / 3.0,
        "K definition",
    );
    // K = E / (3 (1 - 2 nu)) is the independent closed form.
    assert_close(
        state.bulk_modulus().into_base(),
        200e9 / (3.0 * (1.0 - 0.6)),
        "K closed form",
    );
}

#[test]
fn speed_construction_reproduces_the_superseded_consumer_formula() {
    // Differential oracle against the kwavers `lame_from_speeds` body that this
    // contract replaces: mu = rho c_s^2 and lambda = rho (c_p^2 - 2 c_s^2).
    let (c_p, c_s, rho) = (5960.0_f64, 3200.0, 8000.0);
    let state = IsotropicModuli::from_wave_speeds(
        Velocity::from_base(c_p),
        Velocity::from_base(c_s),
        DensityQuantity::from_base(rho),
    )
    .expect("positive definite");

    assert_close(
        *state.shear_modulus().as_base(),
        rho * c_s * c_s,
        "mu = rho c_s^2",
    );
    assert_close(
        *state.lame_lambda().as_base(),
        rho * (c_p * c_p - 2.0 * c_s * c_s),
        "lambda = rho (c_p^2 - 2 c_s^2)",
    );
}

#[test]
fn p_wave_modulus_is_the_compressional_radicand() {
    let state = moduli::<f64>(200e9, 0.3);
    let lambda = *state.lame_lambda().as_base();
    let mu = *state.shear_modulus().as_base();
    assert_close(
        state.p_wave_modulus().into_base(),
        2.0f64.mul_add(mu, lambda),
        "M = lambda + 2 mu",
    );
}

#[test]
fn lame_construction_round_trips_through_the_engineering_pair() {
    let source = moduli::<f64>(114e9, 0.34);
    let rebuilt = IsotropicModuli::from_lame(*source.lame_lambda(), *source.shear_modulus())
        .expect("re-admitting a valid state");
    assert_eq!(source, rebuilt);
}

// ---------------------------------------------------------------------------
// Validity-domain boundaries and adversarial inputs
// ---------------------------------------------------------------------------

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

// ---------------------------------------------------------------------------
// Named catalog
// ---------------------------------------------------------------------------

#[test]
#[expect(
    clippy::float_cmp,
    reason = "catalog constants round-trip through from_base/into_base with no arithmetic, so exact equality is the correct assertion"
)]
fn catalog_entries_preserve_their_published_constants() {
    let steel = NamedIsotropicSolid::CarbonSteel
        .solid::<f64>()
        .expect("catalog entry is valid");
    assert_close(
        steel.moduli().youngs_modulus().into_base(),
        200e9,
        "carbon steel E",
    );
    assert_close(
        steel.moduli().poissons_ratio().into_base(),
        0.30,
        "carbon steel nu",
    );
    assert_eq!(steel.density().quantity().into_base(), 7850.0);

    let aluminium = NamedIsotropicSolid::Aluminium6061
        .solid::<f64>()
        .expect("catalog entry is valid");
    assert_close(
        aluminium.moduli().youngs_modulus().into_base(),
        70e9,
        "6061 E",
    );
    assert_close(
        aluminium.moduli().poissons_ratio().into_base(),
        0.33,
        "6061 nu",
    );
    assert_eq!(aluminium.density().quantity().into_base(), 2700.0);
}

#[test]
#[expect(
    clippy::float_cmp,
    reason = "catalog constants round-trip through from_base/into_base with no arithmetic, so exact equality is the correct assertion"
)]
fn distinct_steel_grades_are_not_conflated() {
    // The two consumer catalogs both said "steel" while meaning different
    // alloys. Merging them would silently substitute constants.
    let carbon = NamedIsotropicSolid::CarbonSteel
        .solid::<f64>()
        .expect("valid");
    let austenitic = NamedIsotropicSolid::StainlessSteel316L
        .solid::<f64>()
        .expect("valid");
    assert_ne!(
        carbon.density().quantity().into_base(),
        austenitic.density().quantity().into_base()
    );
    assert_ne!(
        carbon.moduli().youngs_modulus().into_base(),
        austenitic.moduli().youngs_modulus().into_base()
    );
}

#[test]
fn every_catalog_entry_is_constructible_at_every_supported_scalar() {
    for entry in [
        NamedIsotropicSolid::CarbonSteel,
        NamedIsotropicSolid::StainlessSteel316L,
        NamedIsotropicSolid::Aluminium6061,
        NamedIsotropicSolid::TitaniumGrade5,
    ] {
        let wide = entry.solid::<f64>().expect("f64 entry");
        let narrow = entry.solid::<f32>().expect("f32 entry");
        assert!(wide.moduli().bulk_modulus().into_base() > 0.0);
        assert!(narrow.moduli().bulk_modulus().into_base() > 0.0);
        assert!(entry.thermophysical::<f64>().is_ok());
        assert!(entry.thermal_expansion::<f64>().into_base() > 0.0);
        assert!(!entry.name().is_empty());
    }
}

#[test]
#[expect(
    clippy::float_cmp,
    reason = "catalog constants round-trip through from_base/into_base with no arithmetic, so exact equality is the correct assertion"
)]
fn catalog_density_agrees_between_the_elastic_and_thermophysical_views() {
    for entry in [
        NamedIsotropicSolid::CarbonSteel,
        NamedIsotropicSolid::StainlessSteel316L,
        NamedIsotropicSolid::Aluminium6061,
        NamedIsotropicSolid::TitaniumGrade5,
    ] {
        let solid = entry.solid::<f64>().expect("valid");
        let thermophysical = entry.thermophysical::<f64>().expect("valid");
        assert_eq!(
            solid.density().quantity().into_base(),
            thermophysical.density().quantity().into_base(),
        );
    }
}

// ---------------------------------------------------------------------------
// Properties
// ---------------------------------------------------------------------------

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
