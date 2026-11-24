# Temperature calibration-domain admission

## Contract and demonstrated defect

An admitted law's validity is either `Positive`, or `Bounded` with native
scalar bounds satisfying

```text
is_finite(minimum) && is_finite(maximum)
&& 0 < minimum <= maximum.
```

The reference must then be a finite positive temperature inside that domain.
The public enum allows callers to construct `Bounded` without `bounded()`;
law construction must enforce the domain invariant regardless of that route.

At base `47632f92f7eec394a3a4a16bae2ec715741d0b1f`, a 300 K reference
with directly supplied `[NaN, 340]` bounds is admitted. The existing
`value < minimum || value > maximum` check cannot detect a NaN bound.
Infinite and nonpositive bounds can also be admitted. The original public-API
regression fails because `with_validity` returns a law instead of an error.

## Decision

`with_validity` calls the existing bounded-domain constructor before reference
validation. This keeps one authoritative bound predicate, adds no new state,
and performs no response evaluation or arithmetic conversion. `Positive`
requires no bounds check. A valid bounded domain is stored unchanged, so the
existing inclusive evaluation predicate and response computation are unchanged.

The non-exhaustive `TemperatureLawError` gains `InvalidValidity`, containing
the existing `InvalidTemperatureValidity` with both original bounds. The
domain error takes precedence over reference errors for malformed domains;
valid domains retain `InvalidTemperature` and `OutsideValidityDomain`.
Removing the public enum fields would break callers. Reusing a reference-
temperature or extrapolation error would misidentify a malformed domain.

## Executable verification

`tests/temperature_validity.rs` instantiates every test at `f32`, `f64`, `F32`,
and `F64` through the same `RealField` helper. The negative cases include NaN
in either or both bounds, both infinities, signed zero, negative bounds,
and reversed bounds. Tests inspect the error variant, both rejected values,
and error formatting. Positive controls compare direct and constructor domains,
assert the full computed property bundle at both inclusive bounds and the
reference, and reject their adjacent representable exterior temperatures.
A singleton domain also remains valid. Invalid references retain their typed
role and value. All whole-kelvin inputs and adjacent steps are exact in both
native precisions; no tolerance or widened arithmetic is needed.

## Version and delivery boundary

The original candidate targeted 0.2.0. The integrated source uses the current 0.3 provider cohort. Independent source review, consumer qualification, hosted checks, and coordinator review of deployment settings remain open before merge.
