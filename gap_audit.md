# Proteus gap audit

## Native isotropic-elastic range audit (2026-10-17) [patch]

Two correctness findings at `1681e9a` are implemented and merged in
[#87](https://github.com/ryancinsight/proteus/pull/87) on 2026-10-17:
`PRO-ELASTIC-VALIDITY-RANGE` (overflow admitted negative exact bulk) and
`PRO-ELASTIC-DERIVED-RANGE` (representable E/nu/K/M lost to intermediate range).
Native fused linear combinations preserve the domain sign, including adjacent
and subnormal boundaries; bounded ratios and exact power-of-two scaling keep
intermediates in range. The canonical representation, nonfinite input errors,
provider cohort, hooks and verification limits remain unchanged.

The original eight failures/five controls and expanded baseline red replay are
preserved outside the member. Independent review of [#87 prior reviewed head](https://github.com/ryancinsight/proteus/pull/87/commits/d7ed3f83503372532b688e45a545e5f3b65b0265) adds two P2 findings at Young and bulk overflow
midpoints, despite the historical 20,148-pair oracle. The continuation adds
native residual polynomial comparisons and quotient correction, preserves the
constructor predicate, and retains the four new failures plus 336 rational
fixture cases. All six new tests fail on [#87 prior reviewed head](https://github.com/ryancinsight/proteus/pull/87/commits/d7ed3f83503372532b688e45a545e5f3b65b0265) in all feature/profile modes.
The repaired source passes 73 tests per mode and 78,163 exact-rational pairs
per O2/O3/default/no-default combination, including all independent inputs;
zero domain/classification mismatches, E/nu/K <=1 ULP, M exact rounding. No change to the
wave-speed multiplication/division kernels, constitutive laws or other member
is claimed. Review and performance/resource acceptance remain open; the
sparse index lists `proteus-mat` 0.1.1 (published 2026-10-19T03:26:46Z, looked
up 2026-11-14T00:45Z). See the two backlog rows and
`docs/verification/elastic-range.md`. Older planning/rescue entries below are
preserved; this work does not claim or close them.


## Guarded recovery continuation [patch]

The accepted prior correctness implementation pays residual costs for every
large input. The guarded integration retains the exact comparison inside a
mathematically derived band and uses a bounded native evaluation outside it.
The proof covers the locked f32/f64/F32/F64 domain and cancellation; it is not
inferred solely from the finite oracle. Ordinary outputs and existing repaired
boundaries retain their policies. New transition/wrapper tests and renewed
source/package/gates/cost evidence accompany the changed head. Independent
review and resource acceptance remain separate holds; the registry already lists
`proteus-mat` 0.1.1, so releasing this integration is a release-authority act.

## Finding 2026-08-20: proteus scope-vs-delivery audit

Static audit at `944eed056c1be7bf4b5fd332aa8037c5e6950d7a` on branch
`ci/proteus-book-test` (one staged peer file: `Cargo.lock`). No build, test, or
lint command was executed; every statement below is a file-and-line or
command-output citation. No prior green claim is repeated as this audit's
evidence.

### Measured surface

- 1 package, 18 `src/*.rs` files, 1121 src LOC (`find src -name '*.rs' | xargs
  wc -l`); largest file `src/constitutive/temperature/law.rs` at 288 lines.
- 20 `#[test]` functions across 6 files in `tests/` (637 LOC including
  `examples/`), of which 3 are `proptest!` cases; 0 `#[cfg(test)]` modules in
  `src/`; 0 `benches/`.
- 7 book pages (`docs/book/*.md`, `docs/book/examples/*.md`, 182 lines),
  2 Accepted ADRs, both indexed in `docs/adr/README.md`.
- Conformance floor is clean: 0 `todo!(`/`unimplemented!(`/`TODO`/`FIXME`/
  `HACK`/`panic!(`; 0 production `unwrap()`; 0 `dyn `; 0 `pub use ... as `
  shims; 0 files over 500 lines; `#![deny(missing_docs)]` at
  `src/lib.rs:10`; the single suppression is
  `examples/temperature_material.rs:6` (`#![expect(clippy::print_stdout,
  reason = ...)]`).

### Property and law coverage

- Property families delivered: thermophysical only — `MassDensity`,
  `SpecificHeatCapacity`, `ThermalConductivity` (`src/property/`), composed by
  `ThermophysicalProperties` with the derived thermal diffusivity
  (`src/thermophysical/model.rs:76`). Acoustic, optical, mechanical/elastic,
  electromagnetic, and mass-transport families are absent and are explicitly
  disclaimed as consumer-owned in `README.md:22-26` and ADR 0001; the absence
  is a declared boundary, not undeclared drift.
- Constitutive laws delivered: `ConstantLaw` (`src/constitutive/constant.rs`)
  and `TemperatureLaw` with constant/linear/quadratic responses
  (`src/constitutive/temperature/`). Both route through Aequitas quantities, so
  dimensional consistency is a compile-time property; `ReciprocalTemperature`
  and `ReciprocalTemperatureSquared` keep the K⁻¹/K⁻² coefficient distinction
  (`src/constitutive/temperature/response.rs:32,79-80`). The `ConstitutiveLaw`
  doc advertises pressure, phase-fraction, and solver state
  (`src/constitutive/contract.rs:8`); no law consuming those states exists yet.
- Domain-bound rejection is implemented and typed: `TemperatureValidity::
  bounded` plus `validate_temperature` reject a non-finite, non-positive, or
  out-of-range reference or evaluation temperature
  (`src/constitutive/temperature/law.rs:136-154,266-288`), and every derived
  property re-enters the property boundary
  (`src/constitutive/temperature/law.rs:261`). Rejection is covered by
  `tests/temperature_law.rs:78-317`.

### Data provenance (primary gap)

The repository commits **no source-cited material property value**. No
shipped numeric value carries a citation or a physical validity range; the
one bounded domain is a labelled arithmetic fixture:

- `src/elastic/catalog.rs` ships the public `NamedIsotropicSolid` catalog:
  carbon steel, 316L, 6061-T6 and Ti-6Al-4V with six constants each (density,
  Young's modulus, Poisson's ratio, conductivity, specific heat, expansion),
  exposed through `solid`, `thermophysical` and `thermal_expansion`. The
  Rustdoc calls them "published" but names no source or table locator (316L
  cites ASTM F138/F139 as the grade, not as the origin of its values), the
  carbon-steel entry names no grade, and no entry encodes a validity range.
  PRO-PROV-002 tracks it.
- `examples/constant_material.rs:24` ships `"reference liquid"` with
  1000 / 4000 / 0.6, uncited and unbounded; `README.md:52` binds it to the
  name `water` and `docs/book/examples/constant_material.md:5` calls it
  "water-like". PRO-PROV-010 tracks it.
- `examples/temperature_material.rs` is a labelled `"arithmetic fixture"`
  whose values are documented as non-physical. It is built with
  `TemperatureLaw::with_validity` over an inclusive 280 K to 340 K fixture
  domain and returns an error unless a 400 K evaluation yields
  `TemperatureLawError::OutsideValidityDomain`, so the book claim at
  `docs/book/constitutive_laws.md:64-66` holds for the shipped example. The
  domain is a fixture choice, not calibration evidence.
- `src/elastic/moduli.rs:38-41` binds a doctest value named `steel` (200 GPa,
  nu = 0.3) with no citation. The doctest checks the identity
  mu = E / (2 (1 + nu)), so it is an arithmetic oracle, not reference data,
  and sits outside PRO-PROV-002 as that item's non-goal states.
- `tests/*.rs` fixtures (1000 / 4000 / 0.6, 310.15 K) are water-like round
  numbers used as arithmetic oracles, not reference data — legitimate as
  fixtures, but they mean the "source-traceable reference value" evidence
  category is empty rather than partially satisfied.

### Material identity and consumer vocabulary

`Material` identity is a bare `Cow<'name, str>` (`src/material/model.rs:10`)
with no validation, normalization, or canonical identifier type, and
`Material` is not linked to a catalog: the only named-material data in `src/`
is the elastic `NamedIsotropicSolid` enum (`src/elastic/catalog.rs`), which
`Material` identity does not reference. A consumer therefore receives
validated *property types* but not a shared *material vocabulary*: two
consumers spelling the same tissue differently do not unify, and each still
owns its own table. ADR 0001 limits Phase 1 to "not a speculative catalog of
every material", and ADR 0003 places "catalog-backed material contracts" in
the 0.3.0 milestone; canonical identity and an optional cited catalog remain
the PRO-VOCAB-003 decision, so this is a scope decision to revisit against the
stack role, not an undisclosed gap.

### Claim-vs-code cross-checks

- ADR 0002 Verification lists "semver checks"; `.github/workflows/ci.yml` runs
  fmt, `check --no-default-features`, clippy, nextest, doctests, rustdoc, both
  examples (`:54-57`), a SemVer gate through the shared workflow (`:82-84`),
  and `cargo-deny` (`:91-97`).
- ADR 0001 Verification lists a "release codegen fixture" and ADR 0002 a
  "code-generation fixture". `tests/codegen_equivalence.rs` asserts only
  bitwise *value* equality (`:76-79,90-93`); no codegen or assembly comparison
  exists in the tree or in CI, and nextest runs the default (debug) profile,
  so the "release" qualifier is unverified.
- `Cargo.toml:8` declares `rust-version = "1.95"` while
  `rust-toolchain.toml:3` pins 1.97.0; the `msrv` job (`ci.yml:59-75`) runs
  `cargo check --locked --all-targets --all-features` under
  `RUSTUP_TOOLCHAIN: 1.95.0`, so the floor is exercised.
- `.github/workflows/ci.yml:54-57` runs both shipped examples,
  `constant_material` and `temperature_material`; the book workflow sets
  `mdbook-test: true` (`.github/workflows/book-pages.yml:47`).
- `docs/book/stack_position.md:28-31` asserts that "no duplicate
  material-property implementation exists in Hyperion, Helios, CFDrs, or
  Kwavers". That is a cross-repository claim with no in-repo evidence, and
  `backlog.md` Open simultaneously records that no consumer contract test
  exists. The two statements cannot both be current.
- Generic instantiation is partial: `tests/theorems.rs:14-18`,
  `tests/temperature_law.rs:35-39` and the calibration-bound tests at
  `tests/temperature_law.rs:196-200,250-254` instantiate `f32` and `f64`; the
  validity boundary suite (`tests/properties.rs`), the remaining
  calibration-domain tests (`tests/temperature_law.rs:108-138,256-286`), the
  codegen fixture, and `tests/composition.rs` are `f64`-only.
- Public enums `PropertyKind`, `PropertyConstraint` (`src/property/error.rs:5,
  16`), `CoefficientOrder`, and `TemperatureRole`
  (`src/constitutive/temperature/error.rs:7,16`) carry no `#[non_exhaustive]`,
  while `TemperatureLawError` and `TemperatureValidity` do
  (`.../error.rs:99`, `.../law.rs:108`). Adding a fourth property kind is
  therefore a breaking change for any consumer that matches exhaustively.
- `ThermophysicalResponseSet` is sealed (`src/constitutive/temperature/
  law.rs:12-14,68-71`) with `ResponseSet` as its only implementor, so a
  consumer cannot supply a correlated or tabulated response set. ADR 0002
  records the seam but not the seal.

### Residuals carried forward

- The 2026-08-15 clean-surface verdict below was recorded at
  `3887eacd...`; HEAD has advanced by five commits, so that verdict has
  decayed. The marker and shim scans in this finding re-establish it at
  `944eed05`; the hosted-run evidence is not re-verified here.
- The audit at `944eed05` was static-only. Later changes (PR #104) are gated
  locally on a `git archive` export of the revision with `--manifest-path`,
  `--locked` and the shared target, outside the Atlas patch overlay.

## Clean provider surface — 2026-08-15

The exact fetched default `3887eacda7bc2a6f4bd90b04693e7070f05a894d` has no
`TODO`, `FIXME`, `HACK`, `todo!`, `unimplemented!`, or `panic!` markers in
`src`, `tests`, or `examples`, and no source re-export shim was found. The
provider README, ADR index, and domain book describe the same material and
constitutive-law boundary.

## Residuals

- Atlas-level consumer adoption still needs a direct contract test at the
  consuming repository (PRO-CONSUMER-011); provider presence is not
  integration proof.
- `Cargo.toml` sets `publish = true` and the registry lists `proteus-mat` 0.1.1
  (index `pubtime` 2026-10-19T03:26:46Z, looked up 2026-11-14T00:45Z). The
  trigger "publish policy changes from `publish = false`" fired at 9e9609d and
  is retired. Re-open trigger: the manifest version exceeds the registry's
  latest or a release is requested; each needs release authority (M-1.0 for
  the 1.0 release). Package or documentation checks do not prove release
  readiness.
- `Cargo.lock`, `Cargo.toml` and `examples/temperature_material.rs` are
  committed at this revision; no preserved peer work limits local
  verification, which runs on an export outside the patch overlay.

## Evidence

- Hosted CI: `31865355870`, exact head above, success.
- Hosted Pages build: `31865355539`, exact head above, success.
