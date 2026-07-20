//! Executable laws for temperature-dependent thermophysical response.

use aequitas::systems::si::quantities::{
    MassDensity as DensityQuantity, ReciprocalTemperature, ReciprocalTemperatureSquared,
    SpecificHeatCapacity as HeatCapacityQuantity, ThermalConductivity as ConductivityQuantity,
    ThermodynamicTemperature,
};
use eunomia::RealField;
use proteus::{
    CoefficientOrder, ConstantResponse, ConstitutiveLaw, LinearResponse, MassDensity,
    QuadraticResponse, ResponseSet, SpecificHeatCapacity, TemperatureLaw, TemperatureLawError,
    TemperatureRole, ThermalConductivity, ThermophysicalProperties,
};

fn properties<T: RealField>() -> ThermophysicalProperties<T> {
    ThermophysicalProperties::new(
        MassDensity::new(DensityQuantity::from_base(T::from_f64(1_000.0)))
            .expect("positive density"),
        SpecificHeatCapacity::new(HeatCapacityQuantity::from_base(T::from_f64(4_000.0)))
            .expect("positive heat capacity"),
        ThermalConductivity::new(ConductivityQuantity::from_base(T::from_f64(0.6)))
            .expect("positive conductivity"),
    )
    .expect("positive continuum density")
}

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
    let law = TemperatureLaw::new(properties(), reference_temperature, responses)
        .expect("positive reference temperature");
    let evaluated = law
        .properties(&reference_temperature)
        .expect("reference state is admissible");

    assert_eq!(evaluated, properties());
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
    let law = TemperatureLaw::new(properties(), reference_temperature, responses)
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
        properties(),
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
fn evaluation_revalidates_every_derived_property() {
    let responses = ResponseSet::new(
        LinearResponse::new(ReciprocalTemperature::from_base(-1.0_f64))
            .expect("finite coefficient"),
        ConstantResponse,
        ConstantResponse,
    );
    let law = TemperatureLaw::new(
        properties(),
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
        let law = TemperatureLaw::new(properties(), reference, responses)
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
