//! Definitional identities: each modulus against the closed form that
//! defines it.

use super::*;

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
