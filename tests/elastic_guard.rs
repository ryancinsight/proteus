//! Public value and scalar-wrapper coverage at the residual guard transitions.
use aequitas::systems::si::quantities::Pressure;
use eunomia::{F32, F64, RealField};
use proteus::IsotropicModuli;

trait Native: RealField {
    type Wrapped: RealField;
    const WIDTH: &'static str;
    fn parse_bits(s: &str) -> Self;
    fn bits(self) -> u64;
    fn wrap(self) -> Self::Wrapped;
    fn unwrapped(value: Self::Wrapped) -> Self;
}
impl Native for f32 {
    type Wrapped = F32;
    const WIDTH: &'static str = "32";
    fn parse_bits(s: &str) -> Self {
        Self::from_bits(u32::from_str_radix(s, 16).expect("hex bits"))
    }
    fn bits(self) -> u64 {
        u64::from(self.to_bits())
    }
    fn wrap(self) -> Self::Wrapped {
        F32(self)
    }
    fn unwrapped(value: Self::Wrapped) -> Self {
        value.0
    }
}
impl Native for f64 {
    type Wrapped = F64;
    const WIDTH: &'static str = "64";
    fn parse_bits(s: &str) -> Self {
        Self::from_bits(u64::from_str_radix(s, 16).expect("hex bits"))
    }
    fn bits(self) -> u64 {
        self.to_bits()
    }
    fn wrap(self) -> Self::Wrapped {
        F64(self)
    }
    fn unwrapped(value: Self::Wrapped) -> Self {
        value.0
    }
}
fn boundary_values<T: Native>() {
    let mut cases = 0;
    for line in include_str!("elastic_guard_cases.txt").lines() {
        let f: Vec<_> = line.split_whitespace().collect();
        if f[0] != T::WIDTH {
            continue;
        }
        let values: Vec<_> = f[1..].iter().map(|v| T::parse_bits(v)).collect();
        let state = IsotropicModuli::from_lame(
            Pressure::from_base(values[0]),
            Pressure::from_base(values[1]),
        )
        .expect("positive exact bulk");
        for (got, want) in [
            (state.youngs_modulus().into_base(), values[2]),
            (state.bulk_modulus().into_base(), values[3]),
        ] {
            assert_eq!(got.is_finite(), want.is_finite(), "classification: {line}");
            assert!(
                got.bits().abs_diff(want.bits()) <= 8,
                "value: {line}; got {got:?}"
            );
        }
        cases += 1;
    }
    assert!(cases >= 100, "both scalar formats have transition coverage");
}
#[test]
fn residual_guard_neighbors_keep_values_and_infinity() {
    boundary_values::<f32>();
    boundary_values::<f64>();
}

fn wrappers<T: Native>() {
    for fixture in [
        include_str!("elastic_guard_cases.txt"),
        include_str!("elastic_range/overflow_cases.txt"),
    ] {
        for line in fixture.lines() {
            let f: Vec<_> = line.split_whitespace().collect();
            if f[0] != T::WIDTH {
                continue;
            }
            let l = T::parse_bits(f[1]);
            let m = T::parse_bits(f[2]);
            let native = IsotropicModuli::from_lame(Pressure::from_base(l), Pressure::from_base(m))
                .expect("native admitted state");
            let wrapped = IsotropicModuli::from_lame(
                Pressure::from_base(l.wrap()),
                Pressure::from_base(m.wrap()),
            )
            .expect("transparent wrapper admitted state");
            let actual = [
                T::unwrapped(wrapped.youngs_modulus().into_base()),
                T::unwrapped(wrapped.bulk_modulus().into_base()),
                T::unwrapped(wrapped.poissons_ratio().into_base()),
                T::unwrapped(wrapped.p_wave_modulus().into_base()),
            ];
            let expected = [
                native.youngs_modulus().into_base(),
                native.bulk_modulus().into_base(),
                native.poissons_ratio().into_base(),
                native.p_wave_modulus().into_base(),
            ];
            for (got, want) in actual.into_iter().zip(expected) {
                assert_eq!(got.bits(), want.bits(), "wrapper identity: {line}");
            }
        }
    }
}
#[test]
fn all_supported_real_scalar_wrappers_preserve_native_bits() {
    wrappers::<f32>();
    wrappers::<f64>();
}
