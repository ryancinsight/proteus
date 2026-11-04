//! Exact-rational generated fixtures: range classification and native error.

use eunomia::RealField;

mod common;

trait OracleField: RealField {
    const WIDTH: u32;
    fn from_encoding(value: u64) -> Self;
    fn encoding(self) -> u64;
}

impl OracleField for f32 {
    const WIDTH: u32 = 32;
    fn from_encoding(value: u64) -> Self {
        Self::from_bits(u32::try_from(value).expect("binary32 fixture"))
    }
    fn encoding(self) -> u64 {
        u64::from(self.to_bits())
    }
}

impl OracleField for f64 {
    const WIDTH: u32 = 64;
    fn from_encoding(value: u64) -> Self {
        Self::from_bits(value)
    }
    fn encoding(self) -> u64 {
        self.to_bits()
    }
}

fn verify<T: OracleField>() {
    let mut failures = Vec::new();
    let mut count = 0;
    for line in include_str!("fixtures/thermophysical_range.txt")
        .lines()
        .skip(1)
    {
        let columns: Vec<_> = line.split_whitespace().collect();
        if columns[0].parse::<u32>().expect("width") != T::WIDTH {
            continue;
        }
        let values: Vec<_> = columns[2..]
            .iter()
            .map(|value| T::from_encoding(u64::from_str_radix(value, 16).expect("encoding")))
            .collect();
        let actual = common::properties(values[0], values[1], values[2])
            .thermal_diffusivity()
            .into_base();
        let expected = values[3];
        let classification_matches = actual.is_finite() == expected.is_finite()
            && (actual == T::ZERO) == (expected == T::ZERO);
        if !classification_matches || actual.encoding().abs_diff(expected.encoding()) > 4 {
            failures.push(format!("{line}: actual {:x}", actual.encoding()));
        }
        count += 1;
    }
    assert_eq!(count, 372);
    assert!(
        failures.is_empty(),
        "{} failures: {failures:?}",
        failures.len()
    );
    // Keep the shared ordinary fixture covered in this integration binary.
    assert!(common::reference::<T>().thermal_diffusivity().into_base() > T::ZERO);
}

#[test]
fn exact_rational_binary32() {
    verify::<f32>();
}

#[test]
fn exact_rational_binary64() {
    verify::<f64>();
}
