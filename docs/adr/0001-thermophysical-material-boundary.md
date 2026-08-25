# ADR 0001: Own shared thermophysical material law in Proteus

- Status: Accepted
- Change class: minor, architectural
- Date: 2026-07-20

## Context

Three Atlas integrators independently represent material density:

- Kwavers combines density, specific heat, conductivity, acoustics, perfusion,
  and optics in one raw-`f64` structure.
- CFDrs repeats density, specific heat, and conductivity in fluid and solid
  state structures over Eunomia scalars.
- Helios validates attenuation coefficients but accepts mass density as a raw
  scalar during mass-to-linear attenuation conversion and CT calibration.

The overlap is thermophysical state, not each solver's complete material model.
Acoustic attenuation and perfusion are not CFD laws. Rheology is not a photon
interaction law. Moving all fields into one broad trait would couple unrelated
bounded contexts and create a god interface.

## Decision

Create `proteus` as one public `no_std + alloc` crate.

- Aequitas quantities carry density, specific heat, conductivity, and
  diffusivity dimensions.
- Transparent validating newtypes enforce the property domains.
- `ThermophysicalProperties<T>` composes the three independent properties and
  derives `alpha = k/(rho c_p)` once.
- `ConstitutiveLaw<T>` uses a GAT for a borrowed per-evaluation state family.
- `ConstantLaw<T>` is the initial complete implementation and consumes the
  zero-sized `NoState`.
- `Material<'a, L>` composes identity and law through `Cow<'a, str>`; static
  catalogs borrow names, while runtime-defined materials own them.
- Every law is generic and statically dispatched. No vtable, allocation, unit
  metadata, or scalar widening exists in property evaluation.

## Theorem and proof obligations

### Thermal-diffusivity dimension

Conductivity has dimension `M L T^-3 Theta^-1`. Density has `M L^-3`, and
specific heat has `L^2 T^-2 Theta^-1`. Therefore:

`[k/(rho c_p)] = (M L T^-3 Theta^-1) /
                 ((M L^-3)(L^2 T^-2 Theta^-1)) = L^2 T^-1`.

Aequitas performs this exponent reduction at compile time. Assigning the
expression to any other quantity type fails compilation.

### Non-negativity

If `rho > 0`, `c_p > 0`, and `k >= 0`, then `rho c_p > 0`; division of a
non-negative numerator by a positive denominator yields `alpha >= 0`.
Property tests exercise this law over bounded finite inputs. The claim excludes
native scalar overflow, which the property ranges avoid.

### Value-semantic equivalence

Transparent wrappers store only the Aequitas scalar and `ConstantLaw` performs
only a field copy. The diffusivity expression has the same arithmetic order as
`k/(rho*c_p)`. `tests/codegen_equivalence.rs` compares the typed and raw
implementations bit-for-bit; this is value evidence, not an assembly or
release-code-generation proof.

## Rejected alternatives

- Keep consumer-owned property bundles: rejected because the validity and
  diffusivity laws already repeat across integrators.
- Move all tissue fields into Proteus: rejected because attenuation, perfusion,
  rheology, and photon interaction have distinct domain owners.
- Depend on `uom` directly: rejected because Aequitas is the Atlas dimensional
  SSOT and supports Eunomia storage through one generic API.
- Dynamic dispatch: rejected because law types are known at solver operation
  boundaries and static dispatch preserves monomorphization.
- Parallel owned and borrowed material types: rejected because `Cow<str>`
  expresses the real ownership variation through one contract.

## Consequences

- Phase 1 starts with one complete shared property family, not a speculative
  catalog of every material or constitutive equation.
- Consumer migrations replace duplicated thermophysical types and formulas in
  dependency order. Domain-specific fields remain in consumer-local wrappers
  only while they are still real domain owners, not as Proteus adapters.
- New state-dependent laws extend the same GAT seam when a consumer supplies
  their mathematical specification and validation domain.

## Verification

- positive, negative, zero, NaN, and infinity validation boundaries;
- property tests for non-negativity and conductivity scaling;
- generic instantiation at `f32` and `f64`;
- compile-time Aequitas dimension identity;
- pointer identity for borrowed material names;
- zero-size assertion for `NoState`;
- transparent property layout and allocation-free borrowed material evaluation;
- typed-vs-raw bitwise value comparison;
- no-default-features, Clippy, nextest, doctests, rustdoc, example, and
  supply-chain, SemVer, MSRV, and both shipped example gates in CI.
