# Constitutive laws

`ConstitutiveLaw<T>` is the static seam that maps a material state to a
validated property bundle. Its generic associated state family lets
implementations borrow temperature, pressure, phase fraction, or solver state
without allocation, and everything remains statically dispatched.

## Condition-independent law

`ConstantLaw<T>` wraps validated properties and evaluates to the same bundle
for every state. Its state is the zero-sized `NoState`, and its error is
`Infallible`, so evaluation cannot fail. The following is a focused,
non-standalone API fragment:

```rust,ignore
use proteus::{ConstantLaw, Material, NoState};

let water = Material::borrowed("reference liquid", ConstantLaw::new(properties));
let evaluated = water.properties(NoState)?;
```

## Temperature-dependent law

`TemperatureLaw<T, Responses>` evaluates thermophysical properties relative to
a validated reference temperature. `Responses` is a `ResponseSet` of three
independent property responses — one each for density, specific heat, and
conductivity — composed from `ConstantResponse`, `LinearResponse<T>`, and
`QuadraticResponse<T>` markers. The following is a focused, non-standalone API
fragment:

```rust,ignore
use proteus::{ConstantResponse, LinearResponse, QuadraticResponse,
              ResponseSet, TemperatureLaw, TemperatureValidity};

let responses = ResponseSet::new(
    ConstantResponse,
    LinearResponse::new(ReciprocalTemperature::from_base(2.0e-4))?,
    QuadraticResponse::new(
        ReciprocalTemperature::from_base(1.0e-3),
        ReciprocalTemperatureSquared::from_base(0.0),
    )?,
);
let law = TemperatureLaw::new(reference, ThermodynamicTemperature::from_base(310.15), responses)?;
```

A response multiplies its reference property by a dimensionless factor
`f(ΔT)`: `ConstantResponse` returns 1, `LinearResponse` evaluates
`1 + β₁ΔT`, and `QuadraticResponse` evaluates `1 + β₁ΔT + β₂ΔT²`. First-order
coefficients carry `K⁻¹` and second-order coefficients `K⁻²` dimensions, so a
mis-scaled coefficient cannot type-check. Coefficients must be finite, and the
reference and evaluation temperatures must be finite and strictly positive;
violations return the typed `TemperatureLawError<T>`.

Calibration data can narrow the positive-temperature domain:

```rust,ignore
let validity = TemperatureValidity::bounded(
    ThermodynamicTemperature::from_base(273.15),
    ThermodynamicTemperature::from_base(373.15),
)?;
let law = TemperatureLaw::with_validity(reference, reference_temperature, validity, responses)?;
```

The bounds are inclusive. A reference or evaluation temperature outside the
calibration range returns `TemperatureLawError::OutsideValidityDomain`; the
law never silently extrapolates a response beyond its declared evidence.

The runnable [temperature-material example](examples/temperature_material.md)
walks a tissue-style material through this path.
