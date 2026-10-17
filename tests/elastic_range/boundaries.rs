//! Exact boundaries, cancellation, underflow, and genuinely overflowing results.
use super::*;

fn boundary<T: RealField>(mu: T, at: T, below: T, above: T) {
    for lambda in [below, at] {
        let error =
            IsotropicModuli::from_lame(Pressure::from_base(lambda), Pressure::from_base(mu))
                .expect_err("zero and negative bulk are excluded");
        assert_eq!(error.constraint(), ElasticConstraint::PositiveDefinite);
        assert_eq!(error.quantity(), ElasticQuantity::LameLambda);
        assert_eq!(*error.value(), lambda);
    }
    let state = IsotropicModuli::from_lame(Pressure::from_base(above), Pressure::from_base(mu))
        .expect("one step above the exact boundary");
    let delta = above - at;
    assert_eq!(state.bulk_modulus().into_base(), delta);
    let three = one::<T>() + one::<T>() + one::<T>();
    let expected = (three * delta) * (mu / (above + mu));
    let error = (state.youngs_modulus().into_base() - expected).abs();
    // One division and two products per route, with an 8-epsilon forward bound.
    let eight = three + three + one::<T>() + one::<T>();
    assert!(error <= eight * T::EPSILON * expected);
}

#[test]
fn exact_bulk_boundary_at_native_scales_and_adjacent_values() {
    for scale in [
        f32::from_bits(1),
        f32::MIN_POSITIVE,
        1.0,
        f32::from_bits(253_u32 << 23),
    ] {
        let at = -2.0 * scale;
        boundary(3.0 * scale, at, at.next_down(), at.next_up());
    }
    for scale in [
        f64::from_bits(1),
        f64::MIN_POSITIVE,
        1.0,
        f64::from_bits(2045_u64 << 52),
    ] {
        let at = -2.0 * scale;
        boundary(3.0 * scale, at, at.next_down(), at.next_up());
    }
}

#[test]
fn rounded_two_thirds_does_not_decide_the_exact_domain() {
    // f32's nearest 2/3 is above it; f64's is below it. These neighbouring
    // binary inputs have opposite exact signs, not a configurable tolerance.
    let bad = IsotropicModuli::from_lame(
        Pressure::from_base(f32::from_bits(0xbf2a_aaab)),
        Pressure::from_base(1.0),
    )
    .expect_err("negative exact K");
    assert_eq!(bad.constraint(), ElasticConstraint::PositiveDefinite);
    let good = IsotropicModuli::from_lame(
        Pressure::from_base(f32::from_bits(0xbf2a_aaaa)),
        Pressure::from_base(1.0),
    )
    .expect("positive exact K");
    assert_eq!(
        good.bulk_modulus().into_base().to_bits(),
        (f32::EPSILON / 3.0).to_bits()
    );
    let bad = IsotropicModuli::from_lame(
        Pressure::from_base(f64::from_bits(0xbfe5_5555_5555_5556)),
        Pressure::from_base(1.0),
    )
    .expect_err("negative exact K");
    assert_eq!(bad.constraint(), ElasticConstraint::PositiveDefinite);
    let good = IsotropicModuli::from_lame(
        Pressure::from_base(f64::from_bits(0xbfe5_5555_5555_5555)),
        Pressure::from_base(1.0),
    )
    .expect("positive exact K");
    assert_eq!(
        good.bulk_modulus().into_base().to_bits(),
        (f64::EPSILON / 6.0).to_bits()
    );
}

#[test]
fn young_round_trip_avoids_small_intermediate_product_loss() {
    for mu in [
        f32::from_bits(1),
        f32::MIN_POSITIVE,
        f32::from_bits(54_u32 << 23),
    ] {
        retain_finite_young(mu);
    }
    for mu in [
        f64::from_bits(1),
        f64::MIN_POSITIVE,
        f64::from_bits(489_u64 << 52),
    ] {
        retain_finite_young(mu);
    }
}

#[test]
fn poisson_recovery_retains_a_subnormal_result() {
    let state = IsotropicModuli::from_lame(
        Pressure::from_base(5.0_f32 * f32::from_bits(103_u32 << 23)),
        Pressure::from_base(f32::from_bits(254_u32 << 23)),
    )
    .expect("positive inputs");
    assert_eq!(
        state.poissons_ratio().into_base().to_bits(),
        (f32::from_bits(1)).to_bits()
    );
    let state = IsotropicModuli::from_lame(
        Pressure::from_base(5.0_f64 * f64::from_bits(970_u64 << 52)),
        Pressure::from_base(f64::from_bits(2046_u64 << 52)),
    )
    .expect("positive inputs");
    assert_eq!(
        state.poissons_ratio().into_base().to_bits(),
        (f64::from_bits(1)).to_bits()
    );
}

fn overflowing_sum<T: RealField>(scale: T) {
    let unit = one::<T>();
    let two = unit + unit;
    let four = two + two;
    let nine = four + four + unit;
    let lambda = scale + scale / two + scale / four;
    let mu = scale / two;
    assert!(!(lambda + mu).is_finite());
    let state = IsotropicModuli::from_lame(Pressure::from_base(lambda), Pressure::from_base(mu))
        .expect("positive bulk, even when some derived moduli exceed range");
    // lambda=7s/4,mu=s/2: E=25s/18 and nu=7/18. E fits T.
    let seven = four + two + unit;
    let twenty_five = nine + nine + seven;
    let expected = (scale / (nine + nine)) * twenty_five;
    assert!(expected.is_finite());
    let bound = (four + four) * T::EPSILON;
    assert!((state.youngs_modulus().into_base() - expected).abs() <= bound * expected);
    assert!((state.poissons_ratio().into_base() - seven / (nine + nine)).abs() <= bound);
    assert!(!state.bulk_modulus().into_base().is_finite());
    assert!(!state.p_wave_modulus().into_base().is_finite());
}

#[test]
fn finite_young_and_poisson_survive_an_overflowing_lame_sum() {
    overflowing_sum(f32::from_bits(254_u32 << 23));
    overflowing_sum(f64::from_bits(2046_u64 << 52));
}

#[test]
fn genuine_derived_overflow_keeps_ieee_behavior() {
    let state =
        IsotropicModuli::from_lame(Pressure::from_base(0.0_f32), Pressure::from_base(f32::MAX))
            .expect("finite stored pair and positive exact bulk");
    assert_eq!(
        state.youngs_modulus().into_base().to_bits(),
        f32::INFINITY.to_bits()
    );
    assert_eq!(
        state.p_wave_modulus().into_base().to_bits(),
        f32::INFINITY.to_bits()
    );
    assert_eq!(
        state.poissons_ratio().into_base().to_bits(),
        0.0_f32.to_bits()
    );
    assert!(state.bulk_modulus().into_base().is_finite());
    let state =
        IsotropicModuli::from_lame(Pressure::from_base(0.0_f64), Pressure::from_base(f64::MAX))
            .expect("finite stored pair and positive exact bulk");
    assert_eq!(
        state.youngs_modulus().into_base().to_bits(),
        f64::INFINITY.to_bits()
    );
    assert_eq!(
        state.p_wave_modulus().into_base().to_bits(),
        f64::INFINITY.to_bits()
    );
    assert_eq!(
        state.poissons_ratio().into_base().to_bits(),
        0.0_f64.to_bits()
    );
    assert!(state.bulk_modulus().into_base().is_finite());
}

#[test]
fn a_small_lambda_term_still_decides_youngs_overflow_boundary() {
    let state = IsotropicModuli::from_lame(
        Pressure::from_base(f32::from_bits(0x737f_ffff)),
        Pressure::from_base(f32::from_bits(0x7eff_ffff)),
    )
    .expect("finite stored state");
    assert_eq!(
        state.youngs_modulus().into_base().to_bits(),
        f32::INFINITY.to_bits()
    );
    let state = IsotropicModuli::from_lame(
        Pressure::from_base(f64::from_bits(0x7c9f_ffff_ffff_ffff)),
        Pressure::from_base(f64::from_bits(0x7fdf_ffff_ffff_ffff)),
    )
    .expect("finite stored state");
    assert_eq!(
        state.youngs_modulus().into_base().to_bits(),
        f64::INFINITY.to_bits()
    );
}

fn ordinary<T: RealField>(mu: T) {
    let unit = one::<T>();
    let two = unit + unit;
    let three = two + unit;
    let four = two + two;
    let eight = four + four;
    let eleven = eight + three;
    let state =
        IsotropicModuli::from_lame(Pressure::from_base(three * mu), Pressure::from_base(mu))
            .expect("positive ordinary state");
    assert_eq!(state.youngs_modulus().into_base(), (eleven / four) * mu);
    assert_eq!(state.poissons_ratio().into_base(), three / eight);
    assert_eq!(state.p_wave_modulus().into_base(), (three + two) * mu);
    let expected_bulk = (eleven * mu) / three;
    assert!(
        (state.bulk_modulus().into_base() - expected_bulk).abs()
            <= eight * T::EPSILON * expected_bulk
    );
}

#[test]
fn ordinary_native_identities_at_separated_scales() {
    for mu in [
        4.0_f32,
        f32::from_bits(157_u32 << 23),
        f32::from_bits(97_u32 << 23),
    ] {
        ordinary(mu);
    }
    for mu in [
        4.0_f64,
        f64::from_bits(1053_u64 << 52),
        f64::from_bits(993_u64 << 52),
    ] {
        ordinary(mu);
    }
}

fn constructor_guards<T: RealField>() {
    let unit = one::<T>();
    for bad in [
        <T as NumericElement>::NAN,
        <T as NumericElement>::INFINITY,
        -<T as NumericElement>::INFINITY,
    ] {
        let error = IsotropicModuli::from_lame(Pressure::from_base(bad), Pressure::from_base(unit))
            .expect_err("nonfinite lambda policy retained");
        assert_eq!(error.quantity(), ElasticQuantity::LameLambda);
        assert_eq!(error.constraint(), ElasticConstraint::Finite);
        assert_eq!(error.value().is_nan(), bad.is_nan());
        if !bad.is_nan() {
            assert_eq!(*error.value(), bad);
        }
    }
    for bad in [
        <T as NumericElement>::ZERO,
        -unit,
        <T as NumericElement>::NAN,
        <T as NumericElement>::INFINITY,
        -<T as NumericElement>::INFINITY,
    ] {
        let error = IsotropicModuli::from_lame(Pressure::from_base(unit), Pressure::from_base(bad))
            .expect_err("finite positive mu policy retained");
        assert_eq!(error.quantity(), ElasticQuantity::ShearModulus);
        assert_eq!(error.constraint(), ElasticConstraint::FinitePositive);
        assert_eq!(error.value().is_nan(), bad.is_nan());
        if !bad.is_nan() {
            assert_eq!(*error.value(), bad);
        }
    }
}

#[test]
fn both_native_scalar_constructor_guards_keep_the_typed_policy() {
    constructor_guards::<f32>();
    constructor_guards::<f64>();
}

fn speed_consumer<T: RealField>(scale: T) {
    let unit = one::<T>();
    let two = unit + unit;
    let three = two + unit;
    let four = two + two;
    let seven = four + three;
    let nine = seven + two;
    let state = IsotropicModuli::from_lame(
        Pressure::from_base(-scale),
        Pressure::from_base(two * scale + scale / four),
    )
    .expect("positive exact bulk");
    let rho =
        aequitas::systems::si::quantities::MassDensity::from_base(three * scale + scale / two);
    assert_eq!(
        state
            .compressional_wave_speed(rho)
            .expect("positive finite density")
            .into_base(),
        unit
    );
    assert_eq!(
        state
            .shear_wave_speed(rho)
            .expect("positive finite density")
            .into_base(),
        (nine / (seven + seven)).sqrt()
    );
}

#[test]
fn recovered_modulus_feeds_the_typed_wave_speed_consumer() {
    speed_consumer(f32::from_bits(253_u32 << 23));
    speed_consumer(f64::from_bits(2045_u64 << 52));
}
