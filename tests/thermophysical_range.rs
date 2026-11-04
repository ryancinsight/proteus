//! Public-law regressions with exact power-of-two reference values.

use eunomia::RealField;

mod common;

fn zero_conductivity<T: RealField>(exponent: i32) {
    let small = T::ONE.scale_binary(-exponent);
    let actual = common::properties(small, small, T::ZERO)
        .thermal_diffusivity()
        .into_base();
    assert_eq!(actual, T::ZERO);
}

fn small_product<T: RealField>(exponent: i32) {
    let small = T::ONE.scale_binary(-exponent);
    let actual = common::properties(small, small, small)
        .thermal_diffusivity()
        .into_base();
    assert_eq!(actual, T::ONE.scale_binary(exponent));
}

fn large_product<T: RealField>(exponent: i32) {
    let large = T::ONE.scale_binary(exponent);
    let actual = common::properties(large, large, large)
        .thermal_diffusivity()
        .into_base();
    assert_eq!(actual, T::ONE.scale_binary(-exponent));
}

fn rounded_subnormal_product<T: RealField>(first: i32, second: i32, minimum: i32) {
    // rho*c_p = 3/4 of the least positive subnormal; k/(rho*c_p) = 4/3.
    let actual = common::properties(
        T::ONE.scale_binary(first),
        T::from_f64(1.5).scale_binary(second),
        T::ONE.scale_binary(minimum),
    )
    .thermal_diffusivity()
    .into_base();
    let expected = T::from_f64(4.0) / T::from_f64(3.0);
    assert!((actual - expected).abs() <= T::EPSILON * expected);
}

#[test]
fn zero_conductivity_f32() {
    zero_conductivity::<f32>(100);
}

#[test]
fn zero_conductivity_f64() {
    zero_conductivity::<f64>(600);
}

#[test]
fn small_product_f32() {
    small_product::<f32>(100);
}

#[test]
fn small_product_f64() {
    small_product::<f64>(600);
}

#[test]
fn large_product_f32() {
    large_product::<f32>(100);
}

#[test]
fn large_product_f64() {
    large_product::<f64>(600);
}

#[test]
fn rounded_subnormal_product_f32() {
    rounded_subnormal_product::<f32>(-75, -75, -149);
}

#[test]
fn rounded_subnormal_product_f64() {
    rounded_subnormal_product::<f64>(-538, -537, -1074);
}

#[test]
fn ordinary_reference_retains_legacy_native_order() {
    let reference = common::reference::<f64>();
    assert_eq!(
        reference.thermal_diffusivity().into_base().to_bits(),
        (0.6_f64 / (1_000.0 * 4_000.0)).to_bits()
    );
}

fn signed_zero<T: RealField>(exponent: i32) {
    for scale in [-exponent, 0, exponent] {
        let property = T::ONE.scale_binary(scale);
        for zero in [T::ZERO, -T::ZERO] {
            let result = common::properties(property, property, zero)
                .thermal_diffusivity()
                .into_base();
            assert_eq!(result, T::ZERO);
            assert_eq!(result.is_sign_negative(), zero.is_sign_negative());
        }
    }
}

fn final_range<T: RealField>(maximum: i32, subnormal: i32) {
    let first = maximum / 2 + 1;
    let second = maximum - first + 2;
    let density = T::ONE.scale_binary(first);
    let heat = T::ONE.scale_binary(second);
    let midpoint = T::ONE.scale_binary(first + second + subnormal - 1);
    let result = |conductivity| {
        common::properties(density, heat, conductivity)
            .thermal_diffusivity()
            .into_base()
    };
    assert_eq!(result(midpoint), T::ZERO);
    assert_eq!(
        result(midpoint * (T::ONE + T::EPSILON)),
        T::ONE.scale_binary(subnormal)
    );
    assert_eq!(
        result(midpoint * (T::ONE + T::ONE)),
        T::ONE.scale_binary(subnormal)
    );
    let small = T::ONE.scale_binary(subnormal);
    assert_eq!(
        common::properties(small, small, T::ONE)
            .thermal_diffusivity()
            .into_base(),
        T::INFINITY
    );
}

#[test]
fn signed_zero_and_final_range_native_and_transparent_scalars() {
    signed_zero::<f32>(100);
    signed_zero::<f64>(600);
    signed_zero::<eunomia::F32>(100);
    signed_zero::<eunomia::F64>(600);
    final_range::<f32>(127, -149);
    final_range::<f64>(1023, -1074);
    final_range::<eunomia::F32>(127, -149);
    final_range::<eunomia::F64>(1023, -1074);
}

#[test]
fn transparent_scalars_use_the_same_range_law() {
    zero_conductivity::<eunomia::F32>(100);
    zero_conductivity::<eunomia::F64>(600);
    small_product::<eunomia::F32>(100);
    small_product::<eunomia::F64>(600);
    large_product::<eunomia::F32>(100);
    large_product::<eunomia::F64>(600);
    rounded_subnormal_product::<eunomia::F32>(-75, -75, -149);
    rounded_subnormal_product::<eunomia::F64>(-538, -537, -1074);
}
