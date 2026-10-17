use aequitas::systems::si::quantities::Pressure;
use proteus::IsotropicModuli;
#[test]
fn finite_auxetic_young_single() {
    let s = IsotropicModuli::from_lame(
        Pressure::from_base(f32::from_bits(0xfea7_711f)),
        Pressure::from_base(f32::from_bits(0x7f48_0000)),
    )
    .expect("positive exact bulk");
    let got = s.youngs_modulus().into_base();
    assert!(got.is_finite(), "exact E is below MAX; got {got}");
    assert!(got.to_bits().abs_diff(0x7f7f_fffe) <= 8);
}
#[test]
fn finite_auxetic_young_double() {
    let s = IsotropicModuli::from_lame(
        Pressure::from_base(f64::from_bits(0xffd3_3333_3333_3334)),
        Pressure::from_base(f64::from_bits(0x7fe8_0000_0000_0000)),
    )
    .expect("positive exact bulk");
    let got = s.youngs_modulus().into_base();
    assert!(got.is_finite(), "exact E is below MAX; got {got}");
    assert!(got.to_bits().abs_diff(0x7fef_ffff_ffff_ffff) <= 8);
}
#[test]
fn bulk_below_ieee_overflow_midpoint_single() {
    let s = IsotropicModuli::from_lame(
        Pressure::from_base(f32::from_bits(0x7eaa_aaad)),
        Pressure::from_base(f32::from_bits(0x7f7f_fffd)),
    )
    .expect("positive exact bulk");
    let got = s.bulk_modulus().into_base();
    assert!(
        got.is_finite(),
        "exact K=MAX+ULP/6 rounds finite, below MAX+ULP/2; got {got}"
    );
    assert!(got.to_bits().abs_diff(f32::MAX.to_bits()) <= 8);
}
#[test]
fn bulk_below_ieee_overflow_midpoint_double() {
    let s = IsotropicModuli::from_lame(
        Pressure::from_base(f64::from_bits(0x7fd5_5555_5555_5555)),
        Pressure::from_base(f64::MAX),
    )
    .expect("positive exact bulk");
    let got = s.bulk_modulus().into_base();
    assert!(
        got.is_finite(),
        "exact K=MAX+ULP/6 rounds finite, below MAX+ULP/2; got {got}"
    );
    assert!(got.to_bits().abs_diff(f64::MAX.to_bits()) <= 8);
}

trait FixtureScalar: eunomia::RealField {
    const WIDTH: &'static str;
    fn parse_bits(bits: &str) -> Self;
    fn bits(self) -> u64;
}
impl FixtureScalar for f32 {
    const WIDTH: &'static str = "32";
    fn parse_bits(bits: &str) -> Self {
        Self::from_bits(u32::from_str_radix(bits, 16).expect("hex bits"))
    }
    fn bits(self) -> u64 {
        u64::from(self.to_bits())
    }
}
impl FixtureScalar for f64 {
    const WIDTH: &'static str = "64";
    fn parse_bits(bits: &str) -> Self {
        Self::from_bits(u64::from_str_radix(bits, 16).expect("hex bits"))
    }
    fn bits(self) -> u64 {
        self.to_bits()
    }
}

// Python's integer/Fraction checker audits the reference bits. These cases
// cover each report's neighbors, both signs of lambda, exact midpoint controls,
// true overflow, and adjacent tiny-term and path-activation boundaries.
fn neighborhoods<T: FixtureScalar>() {
    let mut count = 0;
    for line in include_str!("overflow_cases.txt").lines() {
        let fields: Vec<_> = line.split_whitespace().collect();
        if fields[0] != T::WIDTH {
            continue;
        }
        let values: Vec<_> = fields[1..].iter().map(|s| T::parse_bits(s)).collect();
        let state = IsotropicModuli::from_lame(
            Pressure::from_base(values[0]),
            Pressure::from_base(values[1]),
        )
        .expect("positive exact bulk");
        for (got, want) in [
            (state.youngs_modulus().into_base(), values[2]),
            (state.bulk_modulus().into_base(), values[3]),
        ] {
            assert_eq!(
                got.is_finite(),
                want.is_finite(),
                "finite classification: {line}"
            );
            assert!(
                got.bits().abs_diff(want.bits()) <= 8,
                "value: {line}; got {got:?}"
            );
        }
        count += 1;
    }
    assert!(count >= 150, "both native formats have boundary coverage");
}
#[test]
fn boundary_neighborhoods_single() {
    neighborhoods::<f32>();
}
#[test]
fn boundary_neighborhoods_double() {
    neighborhoods::<f64>();
}
