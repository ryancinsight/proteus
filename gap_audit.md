# Proteus gap audit

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
  `examples/temperature_material.rs:3` (`#![expect(clippy::print_stdout,
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
  `tests/temperature_law.rs:122-183,186-214`.

### Data provenance (primary gap)

The repository commits **no source-cited material property value**. Every
numeric value in the tree is a fixture or demonstration constant with no
citation and no encoded validity range:

- `examples/temperature_material.rs:20-31` ships a material named
  `"generic tissue"` (`:37`) with ρ = 1050, c_p = 3600, k = 0.5, β₁ = 2.0e-4
  K⁻¹ and 1.0e-3 K⁻¹, β₂ = 0.0 — no reference, and it uses
  `TemperatureLaw::new` (`:32`), i.e. the unbounded positive-temperature
  domain, not `with_validity`.
- That directly weakens the book claim that "the law never silently
  extrapolates a response beyond its declared evidence"
  (`docs/book/constitutive_laws.md:64-66`): the only shipped physically-named
  material declares no evidence domain at all.
- `tests/*.rs` fixtures (1000 / 4000 / 0.6, 310.15 K) are water-like round
  numbers used as arithmetic oracles, not reference data — legitimate as
  fixtures, but they mean the "source-traceable reference value" evidence
  category is empty rather than partially satisfied.

### Material identity and consumer vocabulary

`Material` identity is a bare `Cow<'name, str>` (`src/material/model.rs:10`)
with no validation, normalization, or canonical identifier type, and no named
material catalog exists anywhere in `src/`. A consumer therefore receives
validated *property types* but not a shared *material vocabulary*: two
consumers spelling the same tissue differently do not unify, and each still
owns its own table. ADR 0001 records the catalog omission as a deliberate
Phase-1 boundary ("not a speculative catalog of every material"), so this is a
scope decision to revisit against the stack role, not an undisclosed gap.

### Claim-vs-code cross-checks

- ADR 0002 Verification lists "semver checks"; `.github/workflows/ci.yml` runs
  fmt, `check --no-default-features`, clippy, nextest, doctests, rustdoc, one
  example, and `cargo-deny` — there is no `cargo-semver-checks` step.
- ADR 0001 Verification lists a "release codegen fixture" and ADR 0002 a
  "code-generation fixture". `tests/codegen_equivalence.rs` asserts only
  bitwise *value* equality (`:76-79,90-93`); no codegen or assembly comparison
  exists in the tree or in CI, and nextest runs the default (debug) profile,
  so the "release" qualifier is unverified.
- `Cargo.toml:5` declares `rust-version = "1.95"` while
  `rust-toolchain.toml:3` pins 1.97.0 and CI installs no second toolchain — the
  MSRV floor is an untested claim.
- `.github/workflows/ci.yml` runs only `--example constant_material`;
  `temperature_material` is compiled by `clippy --all-targets` and executed
  only through the book job's `mdbook-test`.
- `docs/book/stack_position.md:28-31` asserts that "no duplicate
  material-property implementation exists in Hyperion, Helios, CFDrs, or
  Kwavers". That is a cross-repository claim with no in-repo evidence, and
  `backlog.md` Open simultaneously records that no consumer contract test
  exists. The two statements cannot both be current.
- Generic instantiation is partial: `tests/theorems.rs:33-36` and
  `tests/temperature_law.rs:49-52` instantiate `f32` and `f64`; the validity
  boundary suite (`tests/properties.rs`), the calibration-domain suite, the
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
- Local build verification remains blocked by the shared target lock and the
  Atlas patch overlay; this audit is deliberately static-only.

## Clean provider surface — 2026-08-15

The exact fetched default `3887eacda7bc2a6f4bd90b04693e7070f05a894d` has no
`TODO`, `FIXME`, `HACK`, `todo!`, `unimplemented!`, or `panic!` markers in
`src`, `tests`, or `examples`, and no source re-export shim was found. The
provider README, ADR index, and domain book describe the same material and
constitutive-law boundary.

## Residuals

- Atlas-level consumer adoption still needs a direct contract test at the
  consuming repository; provider presence is not integration proof.
- Registry publication is intentionally disabled in `Cargo.toml`. Package or
  documentation checks do not prove release readiness while that policy holds.
- Local verification is limited by preserved peer work in `Cargo.lock`,
  `Cargo.toml`, and `examples/temperature_material.rs`. `cargo fmt --all --
  --check` passes, but `cargo check --locked --no-default-features` stops
  before compilation because the active `D:\atlas\.cargo\config.toml` patch
  overlay requires a lockfile update that `--locked` refuses. This does not
  weaken the hosted exact-head evidence below.

## Evidence

- Hosted CI: `31865355870`, exact head above, success.
- Hosted Pages build: `31865355539`, exact head above, success.
