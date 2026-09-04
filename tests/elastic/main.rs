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

// Each section below is a module of this one test binary: the sections
// were already named here, and a module per section keeps them that way
// without compiling and linking a separate binary for each.
mod catalog;
mod identities;
mod properties;
mod round_trip;
mod validity_domain;
