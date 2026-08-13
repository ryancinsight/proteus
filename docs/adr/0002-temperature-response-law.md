# ADR 0002: Own typed temperature response in Proteus

- Status: accepted
- Change class: minor, architectural
- Date: 2026-07-20

## Context

Kwavers has two independent implementations of relative thermophysical
temperature response:

- `kwavers-physics/src/thermal/properties.rs` applies separate affine
  conductivity and heat-capacity functions around body temperature.
- `kwavers-medium/src/properties/temperature_dependent/thermal.rs` applies a
  quadratic conductivity response and affine heat-capacity response around a
  material reference temperature.

CFDrs uses the same relative affine form for density under thermal expansion.
Its current density implementation is under a separate live change and is not
part of this consumer migration.

The repeated mechanism is a dimensionless property multiplier evaluated from a
temperature difference. Tissue catalogs, coefficient selection, perfusion,
acoustics, rheology, and photon interaction remain domain-owned.

## Decision

Aequitas owns reciprocal-temperature and reciprocal-squared-temperature
dimensions. Proteus owns:

- `ConstantResponse`, a zero-sized invariant response;
- `LinearResponse<T>` for `f(ΔT) = 1 + β₁ΔT`;
- `QuadraticResponse<T>` for `f(ΔT) = 1 + β₁ΔT + β₂ΔT²`;
- `ResponseSet`, which composes independent density, heat-capacity, and
  conductivity strategies; and
- `TemperatureLaw<T, Responses>`, whose GAT state borrows an Aequitas
  thermodynamic temperature. `TemperatureLaw::new` uses the positive-
  temperature domain; `TemperatureLaw::with_validity` accepts the validated
  inclusive `TemperatureValidity` range for calibrated laws.

All routing is statically dispatched. An evaluated bundle re-enters the
existing Proteus property boundary, so a coefficient/temperature combination
that produces a negative or non-finite property returns a typed error.

## Theorems and proof obligations

### Dimensional closure

`[β₁ΔT] = Θ⁻¹Θ = 1` and `[β₂ΔT²] = Θ⁻²Θ² = 1`. Aequitas reduces both terms to
`Dimensionless<T>` at compile time. Multiplying a dimensional property by the
result preserves the property's dimension.

### Reference invariance

At `T = T₀`, `ΔT = 0`; therefore both response families reduce to `f(0) = 1`.
Every property equals its validated reference value.

### Static-dispatch equivalence

Response strategy types are generic parameters and carry no vtable. The
constant strategy is zero-sized. The linear response retains the consumer's
`mul_add` arithmetic order, and the code-generation fixture compares its typed
and raw value semantics bit-for-bit.

## Rejected alternatives

- Keep consumer-local scalar functions: rejected because the dimensional and
  reference-invariance laws are identical.
- Move coefficient catalogs into Proteus: rejected because material-specific
  empirical choices belong to their domains.
- One runtime enum for response order: rejected because response order is known
  at material construction and static strategy composition removes branching.
- Store heterogeneous polynomial coefficients in a raw scalar array: rejected
  because first- and second-order coefficients have different dimensions.

## Consequences

- Kwavers deletes its duplicate temperature-response arithmetic while
  retaining tissue coefficients, perfusion, and acoustic response.
- CFDrs can adopt the linear response after its overlapping live fluid change
  lands.
- Higher-order responses require a new dimensionally typed strategy rather
  than an untyped coefficient vector.

## Verification

- compile-time dimensional closure for K⁻¹ and K⁻² coefficients;
- `f32` and `f64` reference-state invariance;
- exact affine and bounded quadratic value oracles;
- invalid coefficient, temperature, and derived-property cases;
- bounded calibration-domain acceptance and typed extrapolation rejection;
- ZST layout for `ConstantResponse`;
- generated positive-domain properties;
- typed/raw linear-response equivalence;
- no-default-features, Clippy, Nextest, doctests, rustdoc, examples, dependency
  policy, and semver checks.
