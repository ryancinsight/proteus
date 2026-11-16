//! Admission of public calibration domains uses the bounded constructor's contract.

use aequitas::systems::si::quantities::ThermodynamicTemperature;
use eunomia::{F32, F64, NumericElement, RealField};
use proteus::{
    ConstantResponse, ConstitutiveLaw, ResponseSet, TemperatureLaw, TemperatureLawError,
    TemperatureRole, TemperatureValidity,
};

mod common;

fn assert_direct_bounds_are_validated<T: RealField>() {
    let reference = T::from_f64(300.0);
    let lower = T::from_f64(280.0);
    let upper = T::from_f64(340.0);
    let zero = <T as NumericElement>::ZERO;
    let negative = -<T as NumericElement>::ONE;
    let invalid = [
        (T::NAN, upper),
        (lower, T::NAN),
        (T::NAN, T::NAN),
        (-T::INFINITY, upper),
        (T::INFINITY, upper),
        (lower, -T::INFINITY),
        (lower, T::INFINITY),
        (-T::INFINITY, T::INFINITY),
        (zero, upper),
        (-zero, upper),
        (negative, upper),
        (lower, zero),
        (lower, -zero),
        (lower, negative),
        (zero, zero),
        (negative, negative),
        (upper, lower),
    ];

    for (minimum, maximum) in invalid {
        let minimum = ThermodynamicTemperature::from_base(minimum);
        let maximum = ThermodynamicTemperature::from_base(maximum);
        let expected = TemperatureValidity::bounded(minimum, maximum)
            .expect_err("invalid finite-positive ordered bounds");
        let actual = TemperatureLaw::with_validity(
            common::reference(),
            ThermodynamicTemperature::from_base(reference),
            TemperatureValidity::Bounded { minimum, maximum },
            ResponseSet::new(ConstantResponse, ConstantResponse, ConstantResponse),
        )
        .expect_err("direct enum construction must not bypass calibration validation");
        assert_eq!(actual.to_string(), expected.to_string());
        let TemperatureLawError::InvalidValidity(actual) = actual else {
            panic!("invalid bounds must report the domain error: {actual:?}");
        };
        assert_same_value(*actual.minimum(), minimum.into_base());
        assert_same_value(*actual.maximum(), maximum.into_base());
    }
}

#[test]
fn direct_calibration_bounds_cannot_bypass_validation() {
    assert_direct_bounds_are_validated::<f32>();
    assert_direct_bounds_are_validated::<f64>();
    assert_direct_bounds_are_validated::<F32>();
    assert_direct_bounds_are_validated::<F64>();
}

fn assert_same_value<T: RealField>(actual: T, expected: T) {
    if expected.is_nan() {
        assert!(actual.is_nan(), "rejected NaN must remain NaN: {actual:?}");
    } else {
        assert_eq!(actual, expected);
    }
}

fn assert_valid_direct_domains<T: RealField>() {
    // Whole kelvin values and the adjacent-value step in [256, 512) are
    // exactly representable in both native precisions and their wrappers.
    let reference = T::from_f64(300.0);
    for (minimum, maximum) in [(280.0, 340.0), (300.0, 300.0)] {
        let minimum = ThermodynamicTemperature::from_base(T::from_f64(minimum));
        let maximum = ThermodynamicTemperature::from_base(T::from_f64(maximum));
        let responses = ResponseSet::new(ConstantResponse, ConstantResponse, ConstantResponse);
        let direct = TemperatureLaw::with_validity(
            common::reference(),
            ThermodynamicTemperature::from_base(reference),
            TemperatureValidity::Bounded { minimum, maximum },
            responses,
        )
        .expect("direct valid bounds admit the reference");
        let constructed = TemperatureLaw::with_validity(
            common::reference(),
            ThermodynamicTemperature::from_base(reference),
            TemperatureValidity::bounded(minimum, maximum).expect("valid bounds"),
            responses,
        )
        .expect("constructor bounds admit the reference");
        assert_eq!(direct, constructed);

        for temperature in [
            minimum,
            ThermodynamicTemperature::from_base(reference),
            maximum,
        ] {
            assert_eq!(direct.properties(&temperature), Ok(common::reference()));
        }
        let step = T::from_f64(256.0) * T::EPSILON;
        for value in [minimum.into_base() - step, maximum.into_base() + step] {
            assert_eq!(
                direct.properties(&ThermodynamicTemperature::from_base(value)),
                Err(TemperatureLawError::OutsideValidityDomain {
                    role: TemperatureRole::Evaluation,
                    value,
                    minimum: minimum.into_base(),
                    maximum: maximum.into_base(),
                })
            );
        }
    }
}

#[test]
fn valid_direct_domains_preserve_values_and_inclusive_bounds() {
    assert_valid_direct_domains::<f32>();
    assert_valid_direct_domains::<f64>();
    assert_valid_direct_domains::<F32>();
    assert_valid_direct_domains::<F64>();
}

fn assert_invalid_domain_precedes_reference<T: RealField>() {
    let minimum = ThermodynamicTemperature::from_base(<T as NumericElement>::ZERO);
    let maximum = ThermodynamicTemperature::from_base(T::from_f64(340.0));
    let error = TemperatureLaw::with_validity(
        common::reference(),
        ThermodynamicTemperature::from_base(T::NAN),
        TemperatureValidity::Bounded { minimum, maximum },
        ResponseSet::new(ConstantResponse, ConstantResponse, ConstantResponse),
    )
    .expect_err("validate the domain before the reference state");
    let TemperatureLawError::InvalidValidity(error) = error else {
        panic!("invalid domain must take precedence: {error:?}");
    };
    assert_eq!(*error.minimum(), minimum.into_base());
    assert_eq!(*error.maximum(), maximum.into_base());
}

#[test]
fn invalid_domains_are_rejected_before_reference_validation() {
    assert_invalid_domain_precedes_reference::<f32>();
    assert_invalid_domain_precedes_reference::<f64>();
    assert_invalid_domain_precedes_reference::<F32>();
    assert_invalid_domain_precedes_reference::<F64>();
}

fn assert_reference_errors<T: RealField>() {
    let lower = T::from_f64(280.0);
    let upper = T::from_f64(340.0);
    let responses = ResponseSet::new(ConstantResponse, ConstantResponse, ConstantResponse);
    let validity = TemperatureValidity::Bounded {
        minimum: ThermodynamicTemperature::from_base(lower),
        maximum: ThermodynamicTemperature::from_base(upper),
    };
    for value in [T::NAN, T::INFINITY, <T as NumericElement>::ZERO] {
        let error = TemperatureLaw::with_validity(
            common::reference(),
            ThermodynamicTemperature::from_base(value),
            validity,
            responses,
        )
        .expect_err("valid domain retains the invalid-reference error");
        let TemperatureLawError::InvalidTemperature {
            role,
            value: actual,
        } = error
        else {
            panic!("reference error must retain its type: {error:?}");
        };
        assert_eq!(role, TemperatureRole::Reference);
        assert_same_value(actual, value);
    }
    for value in [T::from_f64(279.0), T::from_f64(341.0)] {
        assert_eq!(
            TemperatureLaw::with_validity(
                common::reference(),
                ThermodynamicTemperature::from_base(value),
                validity,
                responses,
            ),
            Err(TemperatureLawError::OutsideValidityDomain {
                role: TemperatureRole::Reference,
                value,
                minimum: lower,
                maximum: upper,
            })
        );
    }
}

#[test]
fn valid_domains_retain_typed_reference_errors() {
    assert_reference_errors::<f32>();
    assert_reference_errors::<f64>();
    assert_reference_errors::<F32>();
    assert_reference_errors::<F64>();
}
