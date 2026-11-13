//! Executable laws for temperature-dependent thermophysical response.

use aequitas::systems::si::quantities::{
    ReciprocalTemperature, ReciprocalTemperatureSquared, ThermodynamicTemperature,
};
use eunomia::RealField;
use proteus::{
    CoefficientOrder, ConstantResponse, ConstitutiveLaw, LinearResponse, QuadraticResponse,
    ResponseSet, TemperatureLaw, TemperatureLawError, TemperatureRole, TemperatureValidity,
};

mod common;

fn assert_reference_invariance<T: RealField>() {
    let reference_temperature = ThermodynamicTemperature::from_base(T::from_f64(310.15));
    let responses = ResponseSet::new(
        ConstantResponse,
        LinearResponse::new(ReciprocalTemperature::from_base(T::from_f64(0.001)))
            .expect("finite coefficient"),
        QuadraticResponse::new(
            ReciprocalTemperature::from_base(T::from_f64(0.002)),
            ReciprocalTemperatureSquared::from_base(T::from_f64(-1.0e-5)),
        )
        .expect("finite coefficients"),
    );
    let law = TemperatureLaw::new(common::reference(), reference_temperature, responses)
        .expect("positive reference temperature");
    let evaluated = law
        .properties(&reference_temperature)
        .expect("reference state is admissible");

    assert_eq!(evaluated, common::reference());
}

#[test]
fn reference_state_is_invariant_for_every_supported_real_scalar() {
    assert_reference_invariance::<f32>();
    assert_reference_invariance::<f64>();
}

#[test]
fn independent_response_orders_match_their_polynomials() {
    let reference_temperature = ThermodynamicTemperature::from_base(300.0_f64);
    let responses = ResponseSet::new(
        ConstantResponse,
        LinearResponse::new(ReciprocalTemperature::from_base(0.001)).expect("finite coefficient"),
        QuadraticResponse::new(
            ReciprocalTemperature::from_base(0.002),
            ReciprocalTemperatureSquared::from_base(-1.0e-5),
        )
        .expect("finite coefficients"),
    );
    let law = TemperatureLaw::new(common::reference(), reference_temperature, responses)
        .expect("positive reference temperature");
    let temperature = ThermodynamicTemperature::from_base(310.0);
    let evaluated = law
        .properties(&temperature)
        .expect("positive response factors");

    assert_eq!(
        evaluated.density().quantity().into_base().to_bits(),
        1_000.0_f64.to_bits()
    );
    assert_eq!(
        evaluated
            .specific_heat_capacity()
            .quantity()
            .into_base()
            .to_bits(),
        (4_000.0_f64 * (1.0 + 0.001 * 10.0)).to_bits()
    );
    let expected_conductivity = 0.6_f64 * (1.0 + 0.002 * 10.0 - 1.0e-5 * 10.0 * 10.0);
    let actual_conductivity = evaluated.thermal_conductivity().quantity().into_base();
    let rounding = 8.0 * f64::EPSILON * expected_conductivity;
    assert!((actual_conductivity - expected_conductivity).abs() <= rounding);
}

#[test]
fn coefficient_and_temperature_boundaries_are_typed() {
    let invalid = LinearResponse::new(ReciprocalTemperature::from_base(f64::NAN))
        .expect_err("NaN coefficient must be rejected");
    assert_eq!(invalid.order(), CoefficientOrder::Linear);
    assert!(invalid.value().is_nan());

    let invalid = QuadraticResponse::new(
        ReciprocalTemperature::from_base(0.0_f64),
        ReciprocalTemperatureSquared::from_base(f64::INFINITY),
    )
    .expect_err("infinite coefficient must be rejected");
    assert_eq!(invalid.order(), CoefficientOrder::Quadratic);

    let responses = ResponseSet::new(ConstantResponse, ConstantResponse, ConstantResponse);
    let invalid_reference = TemperatureLaw::new(
        common::reference(),
        ThermodynamicTemperature::from_base(0.0),
        responses,
    )
    .expect_err("absolute zero is outside the law domain");
    assert!(matches!(
        invalid_reference,
        TemperatureLawError::InvalidTemperature {
            role: TemperatureRole::Reference,
            value: 0.0
        }
    ));
}

#[test]
fn bounded_calibration_domain_rejects_extrapolation() {
    let validity = TemperatureValidity::bounded(
        ThermodynamicTemperature::from_base(273.0_f64),
        ThermodynamicTemperature::from_base(373.0_f64),
    )
    .expect("finite positive ordered calibration bounds");
    let responses = ResponseSet::new(ConstantResponse, ConstantResponse, ConstantResponse);
    let law = TemperatureLaw::with_validity(
        common::reference(),
        ThermodynamicTemperature::from_base(300.0),
        validity,
        responses,
    )
    .expect("reference temperature is calibrated");

    assert_eq!(
        law.properties(&ThermodynamicTemperature::from_base(350.0))
            .expect("350 K lies inside the calibration domain"),
        common::reference()
    );
    assert!(matches!(
        law.properties(&ThermodynamicTemperature::from_base(1_500.0)),
        Err(TemperatureLawError::OutsideValidityDomain {
            role: TemperatureRole::Evaluation,
            value: 1_500.0,
            minimum: 273.0,
            maximum: 373.0,
        })
    ));
}

fn assert_calibration_bounds_are_inclusive<T: RealField>() {
    let minimum = T::from_f64(273.0);
    let maximum = T::from_f64(373.0);
    let validity = TemperatureValidity::bounded(
        ThermodynamicTemperature::from_base(minimum),
        ThermodynamicTemperature::from_base(maximum),
    )
    .expect("finite positive ordered calibration bounds");
    // beta = 2^-10 with offsets of -27 K and +73 K from the 300 K reference
    // gives the heat-capacity factors 997/1024 and 1097/1024, and their
    // products with c_p = 4000 are 3894.53125 and 4285.15625. Every operand
    // and result of that heat-capacity chain is a dyadic rational that `f32`
    // and `f64` both hold exactly. Density and conductivity pass through
    // `ConstantResponse` unchanged, so their expected values are the same
    // rounded literals the reference carries. The assertions are therefore
    // equalities rather than tolerances.
    let coefficient = ReciprocalTemperature::from_base(T::from_f64(0.000_976_562_5));
    let responses = ResponseSet::new(
        ConstantResponse,
        LinearResponse::new(coefficient).expect("finite coefficient"),
        ConstantResponse,
    );
    let law = TemperatureLaw::with_validity(
        common::reference(),
        ThermodynamicTemperature::from_base(T::from_f64(300.0)),
        validity,
        responses,
    )
    .expect("reference temperature is calibrated");

    for (bound, heat_capacity) in [(minimum, 3_894.531_25), (maximum, 4_285.156_25)] {
        assert_eq!(
            law.properties(&ThermodynamicTemperature::from_base(bound)),
            Ok(common::properties(
                T::from_f64(1_000.0),
                T::from_f64(heat_capacity),
                T::from_f64(0.6),
            )),
            "the inclusive bound itself is calibrated and evaluates the response"
        );
    }
    // One kelvin beyond each bound: the smallest whole-kelvin probe that is
    // exactly representable at both scalars and strictly outside the domain.
    for value in [T::from_f64(272.0), T::from_f64(374.0)] {
        assert_eq!(
            law.properties(&ThermodynamicTemperature::from_base(value)),
            Err(TemperatureLawError::OutsideValidityDomain {
                role: TemperatureRole::Evaluation,
                value,
                minimum,
                maximum,
            })
        );
    }
}

#[test]
fn calibration_bounds_are_inclusive() {
    assert_calibration_bounds_are_inclusive::<f32>();
    assert_calibration_bounds_are_inclusive::<f64>();
}

fn assert_adjacent_values_decide_calibration_bounds<T: RealField>() {
    let minimum = T::from_f64(273.0);
    let maximum = T::from_f64(373.0);
    // Both bounds lie in [256, 512), where adjacent representable values are
    // 2^8 * EPSILON apart: a power of two, so the sums below are exact.
    let spacing = T::EPSILON * T::from_f64(256.0);
    assert!(
        T::from_f64(256.0) <= minimum && maximum < T::from_f64(512.0),
        "the spacing derivation holds only for bounds inside [256, 512)"
    );
    assert!(minimum - spacing < minimum && minimum < minimum + spacing);
    assert!(maximum - spacing < maximum && maximum < maximum + spacing);
    let validity = TemperatureValidity::bounded(
        ThermodynamicTemperature::from_base(minimum),
        ThermodynamicTemperature::from_base(maximum),
    )
    .expect("finite positive ordered calibration bounds");
    // Constant responses make every admitted evaluation equal the reference
    // exactly, so only the domain decision varies between the probes.
    let law = TemperatureLaw::with_validity(
        common::reference(),
        ThermodynamicTemperature::from_base(T::from_f64(300.0)),
        validity,
        ResponseSet::new(ConstantResponse, ConstantResponse, ConstantResponse),
    )
    .expect("reference temperature is calibrated");

    for value in [minimum + spacing, maximum - spacing] {
        assert_eq!(
            law.properties(&ThermodynamicTemperature::from_base(value)),
            Ok(common::reference()),
            "the representable value just inside a bound is calibrated"
        );
    }
    for value in [minimum - spacing, maximum + spacing] {
        assert_eq!(
            law.properties(&ThermodynamicTemperature::from_base(value)),
            Err(TemperatureLawError::OutsideValidityDomain {
                role: TemperatureRole::Evaluation,
                value,
                minimum,
                maximum,
            }),
            "the representable value just outside a bound is rejected"
        );
    }
}

#[test]
fn adjacent_representable_values_decide_calibration_bounds() {
    assert_adjacent_values_decide_calibration_bounds::<f32>();
    assert_adjacent_values_decide_calibration_bounds::<f64>();
}

#[test]
fn bounded_calibration_domain_rejects_invalid_bounds_and_reference() {
    let invalid = TemperatureValidity::bounded(
        ThermodynamicTemperature::from_base(373.0_f64),
        ThermodynamicTemperature::from_base(273.0_f64),
    )
    .expect_err("lower bound must not exceed upper bound");
    assert_eq!(invalid.minimum().to_bits(), 373.0_f64.to_bits());
    assert_eq!(invalid.maximum().to_bits(), 273.0_f64.to_bits());

    let validity = TemperatureValidity::bounded(
        ThermodynamicTemperature::from_base(273.0_f64),
        ThermodynamicTemperature::from_base(373.0_f64),
    )
    .expect("finite positive ordered calibration bounds");
    let responses = ResponseSet::new(ConstantResponse, ConstantResponse, ConstantResponse);
    assert!(matches!(
        TemperatureLaw::with_validity(
            common::reference(),
            ThermodynamicTemperature::from_base(400.0),
            validity,
            responses,
        ),
        Err(TemperatureLawError::OutsideValidityDomain {
            role: TemperatureRole::Reference,
            value: 400.0,
            minimum: 273.0,
            maximum: 373.0,
        })
    ));
}

#[test]
fn evaluation_revalidates_every_derived_property() {
    let responses = ResponseSet::new(
        LinearResponse::new(ReciprocalTemperature::from_base(-1.0_f64))
            .expect("finite coefficient"),
        ConstantResponse,
        ConstantResponse,
    );
    let law = TemperatureLaw::new(
        common::reference(),
        ThermodynamicTemperature::from_base(300.0),
        responses,
    )
    .expect("positive reference temperature");
    let invalid_temperature = ThermodynamicTemperature::from_base(f64::NAN);
    assert!(matches!(
        law.properties(&invalid_temperature),
        Err(TemperatureLawError::InvalidTemperature {
            role: TemperatureRole::Evaluation,
            value
        }) if value.is_nan()
    ));

    let temperature = ThermodynamicTemperature::from_base(302.0);
    assert!(matches!(
        law.properties(&temperature),
        Err(TemperatureLawError::InvalidProperty(error))
            if error.kind() == proteus::PropertyKind::MassDensity
    ));
}

#[test]
fn constant_response_is_zero_sized() {
    assert_eq!(core::mem::size_of::<ConstantResponse>(), 0);
}

proptest::proptest! {
    #[test]
    fn positive_affine_factors_preserve_property_domains(
        delta in -20.0_f64..20.0,
        slope in -0.01_f64..0.01,
    ) {
        let responses = ResponseSet::new(
            LinearResponse::new(ReciprocalTemperature::from_base(slope))
                .expect("generated coefficient is finite"),
            ConstantResponse,
            ConstantResponse,
        );
        let reference = ThermodynamicTemperature::from_base(300.0);
        let law = TemperatureLaw::new(common::reference(), reference, responses)
            .expect("positive reference temperature");
        let temperature = ThermodynamicTemperature::from_base(300.0 + delta);
        let evaluated = law.properties(&temperature)
            .expect("generated response factor remains positive");
        let expected = 1_000.0 * slope.mul_add(delta, 1.0);
        let actual = evaluated.density().quantity().into_base();
        let rounding = 8.0 * f64::EPSILON * expected.abs().max(1.0);
        proptest::prop_assert!((actual - expected).abs() <= rounding);
    }
}
