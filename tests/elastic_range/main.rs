//! Independent native-precision range checks against unchanged Proteus main.
//! Fixtures use exact powers of two and rational elastic identities; no widened
//! arithmetic, production copies, or tolerance-based expected values.
use aequitas::systems::si::quantities::{Dimensionless, Pressure};
use eunomia::{NumericElement, RealField};
use proteus::{ElasticConstraint, ElasticQuantity, IsotropicModuli};

fn one<T: RealField>() -> T {
    <T as NumericElement>::ONE
}

fn reject_negative_bulk<T: RealField>(scale: T) {
    let unit = one::<T>();
    let three = unit + unit + unit;
    let mu = three * scale;
    let lambda = -mu;
    // Exact K = -3s + (2/3)(3s) = -s, finite and strictly negative.
    assert!(scale.is_finite() && scale > <T as NumericElement>::ZERO);
    assert!(mu.is_finite() && lambda.is_finite());
    let error = IsotropicModuli::from_lame(Pressure::from_base(lambda), Pressure::from_base(mu))
        .expect_err(
            "exact bulk=-scale is negative; finite inputs do not establish positive definiteness",
        );
    assert_eq!(error.quantity(), ElasticQuantity::LameLambda);
    assert_eq!(error.constraint(), ElasticConstraint::PositiveDefinite);
    assert_eq!(*error.value(), lambda);
}

fn retain_finite_young<T: RealField>(mu: T) {
    let unit = one::<T>();
    let two = unit + unit;
    let expected = two * mu;
    assert!(expected.is_finite());
    // For lambda=0, E=2mu and nu=0 exactly. E and the stored pair fit T.
    let state = IsotropicModuli::from_young_poisson(
        Pressure::from_base(expected),
        Dimensionless::from_base(<T as NumericElement>::ZERO),
    )
    .expect("positive finite engineering pair");
    assert_eq!(*state.shear_modulus().as_base(), mu);
    assert_eq!(*state.lame_lambda().as_base(), <T as NumericElement>::ZERO);
    assert_eq!(state.youngs_modulus().into_base(), expected);
}

fn retain_poisson_ratio<T: RealField>(scale: T) {
    let unit = one::<T>();
    let two = unit + unit;
    let four = two + two;
    let state = IsotropicModuli::from_lame(Pressure::from_base(scale), Pressure::from_base(scale))
        .expect("lambda=mu>0 has positive bulk");
    // nu = s / (2(s+s)) = 1/4, independent of scale.
    assert_eq!(state.poissons_ratio().into_base(), unit / four);
}

fn retain_finite_auxetic_moduli<T: RealField>(scale: T) {
    let unit = one::<T>();
    let two = unit + unit;
    let three = two + unit;
    let four = two + two;
    let mu = two * scale + scale / four;
    let expected_bulk = scale / two;
    let expected_p_wave = three * scale + scale / two;
    assert!(mu.is_finite() && expected_p_wave.is_finite());
    // lambda=-s, mu=9s/4: K=s/2>0 and M=7s/2, both representable.
    let state = IsotropicModuli::from_lame(Pressure::from_base(-scale), Pressure::from_base(mu))
        .expect("positive finite bulk=s/2");
    assert_eq!(
        (
            state.bulk_modulus().into_base(),
            state.p_wave_modulus().into_base()
        ),
        (expected_bulk, expected_p_wave)
    );
}

#[test]
fn regression_negative_bulk_f32() {
    reject_negative_bulk(f32::from_bits(253_u32 << 23)); // s=2^126
}
#[test]
fn regression_negative_bulk_f64() {
    reject_negative_bulk(f64::from_bits(2045_u64 << 52)); // s=2^1022
}
#[test]
fn regression_young_round_trip_f32() {
    retain_finite_young(f32::from_bits(191_u32 << 23)); // mu=2^64
}
#[test]
fn regression_young_round_trip_f64() {
    retain_finite_young(f64::from_bits(1535_u64 << 52)); // mu=2^512
}
#[test]
fn regression_poisson_ratio_f32() {
    retain_poisson_ratio(f32::from_bits(253_u32 << 23));
}
#[test]
fn regression_poisson_ratio_f64() {
    retain_poisson_ratio(f64::from_bits(2045_u64 << 52));
}
#[test]
fn regression_auxetic_finite_moduli_f32() {
    retain_finite_auxetic_moduli(f32::from_bits(253_u32 << 23));
}
#[test]
fn regression_auxetic_finite_moduli_f64() {
    retain_finite_auxetic_moduli(f64::from_bits(2045_u64 << 52));
}

#[test]
fn control_negative_bulk_unit_scale() {
    reject_negative_bulk(1.0_f32);
    reject_negative_bulk(1.0_f64);
}
#[test]
fn control_young_round_trip_unit_scale() {
    retain_finite_young(1.0_f32);
    retain_finite_young(1.0_f64);
}
#[test]
fn control_poisson_ratio_unit_scale() {
    retain_poisson_ratio(1.0_f32);
    retain_poisson_ratio(1.0_f64);
}
#[test]
fn control_auxetic_moduli_unit_scale() {
    retain_finite_auxetic_moduli(1.0_f32);
    retain_finite_auxetic_moduli(1.0_f64);
}
fn reject_nonfinite_lambda<T: RealField>(bad: T) {
    let error =
        IsotropicModuli::from_lame(Pressure::from_base(bad), Pressure::from_base(one::<T>()))
            .expect_err("nonfinite lambda is rejected");
    assert_eq!(error.quantity(), ElasticQuantity::LameLambda);
    assert_eq!(error.constraint(), ElasticConstraint::Finite);
    assert!(!error.value().is_finite());
}
#[test]
fn control_nonfinite_input_guard_preserved() {
    for bad in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
        reject_nonfinite_lambda(bad);
    }
    for bad in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        reject_nonfinite_lambda(bad);
    }
}

mod boundaries;
