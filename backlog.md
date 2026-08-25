# Proteus backlog

This board records provider-owned work. Cross-repository adoption remains in
the Atlas board.

## PRO-DOC-BOOK-001 — Execute included book examples [patch] — closed 2026-08-20

- **Owner:** Atlas coordinator; Proteus source scope only. The peer-staged
  `Cargo.lock` remains outside this increment.
- **Scope:** shared Pages caller and the two included example sources;
  `constitutive_laws.md` and `validated_properties.md` retain their focused,
  non-standalone `rust,ignore` API fragments.
- **Acceptance:** the caller enables the shared `mdbook-test` job with Rust
  1.97.0 and package `proteus`; both included examples compile against staged
  `proteus` and `aequitas` crates and the hosted Pages build passes.
- **Baseline:** local `mdbook test docs/book` reached both examples and failed
  with unresolved `proteus`/`aequitas` crates because the caller did not stage
  a package. The failure was a real integration gap, not an existence-only
  assertion.
- **Local evidence target:** format, package build, book build, strict link
  scan, and executable book tests; the shared Windows cache may require the
  hosted Linux job for final mdBook staging evidence.
- **Published source:** commit `61879d4b4a68e2d201460af0fe6f6a0e7fe9919f`
  on `ci/proteus-book-test`, PR
  [#14](https://github.com/ryancinsight/proteus/pull/14); hosted verification
  passes at exact head `4d482709927aa464f9116b36dc016d516b01b8be` in CI and
  Pages runs `32338237653` and `32338238163`. PR #14 merged at default
  `1c73fdd17e45cb7d1feb63fcbea774c6bbb5a146`.
- **Definition of done:** met. The hosted Pages book build executes the
  package-staged examples; its deployment job is correctly skipped for the
  pull-request event.

## PROTEUS-PHASE-1-001 — Material and constitutive-law provider surface — closed

- Outcome: Proteus owns validated thermophysical properties, typed temperature
  response laws, constitutive evaluation, and borrowed-or-owned material names.
- Boundary: Aequitas owns dimensions and units; Eunomia owns scalar
  representations; consumers retain their domain-specific attenuation,
  perfusion, rheology, and photon-interaction laws.
- Evidence: provider default `3887eacda7bc2a6f4bd90b04693e7070f05a894d`;
  hosted CI and Pages runs `31865355870` and `31865355539` both pass at that
  exact head.

## Gap-audit 2026-08-20 items

Filed by the Atlas gap audit at `944eed05` from static evidence only; each
item cites its evidence in `gap_audit.md`. All are unclaimed.

### PRO-PROV-002 — Cite and bound every shipped material value

- **Outcome:** every property value and response coefficient committed outside
  a pure arithmetic fixture carries a resolvable citation and an explicit
  `TemperatureValidity::bounded` calibration domain.
- **Scope:** `examples/temperature_material.rs`, and any future named material.
  Non-goals: a material catalog (PRO-VOCAB-003), test fixtures whose role is an
  arithmetic oracle rather than reference data.
- **Acceptance oracle:** the `"generic tissue"` example is either renamed to a
  non-physical fixture or re-grounded on a cited source (section/table locator
  in a source comment) and constructed with `with_validity`; a test asserts
  that an evaluation outside the cited range returns
  `TemperatureLawError::OutsideValidityDomain`.
- **Dependencies:** none.
- **Risk/change class:** [verification] [patch]; effort M.
- **Status:** todo.

### PRO-VOCAB-003 — Decide whether Proteus owns a named-material vocabulary

- **Outcome:** an ADR (0003) records, with a recommended option, whether the
  provider owns canonical material identity (a validated `MaterialId`-class
  newtype and, optionally, a cited catalog) or whether identity stays a
  consumer-supplied string.
- **Scope:** ADR plus, if Accepted for ownership, the identity newtype in
  `src/material/`. Non-goals: acoustic/optical/rheology property families,
  which stay consumer-owned per ADR 0001.
- **Acceptance oracle:** ADR 0003 exists, is indexed in `docs/adr/README.md`,
  and states the decision against the current bare `Cow<'name, str>` identity
  (`src/material/model.rs:10`) and the "material-identity vocabulary" stack
  role; if ownership is accepted, invalid identifiers are rejected by a typed
  constructor with a value-semantic test.
- **Dependencies:** none; informs PRO-PROV-002.
- **Risk/change class:** [arch] [minor]; effort L.
- **Status:** todo.

### PRO-GATE-004 — Reconcile CI with the ADR verification claims

- **Outcome:** the gates the ADRs claim either run in CI or the ADR wording is
  corrected to the gates that exist.
- **Scope:** `.github/workflows/ci.yml`, `docs/adr/0001-*.md`,
  `docs/adr/0002-*.md`. Non-goals: the shared Pages caller.
- **Acceptance oracle:** `cargo-semver-checks` runs on pull requests (ADR 0002
  claims it and the workflow has no such step), an MSRV job builds at the
  declared `rust-version = 1.95` floor, and both examples run; or each removed
  claim is struck from the ADR with a dated revision note.
- **Dependencies:** none.
- **Risk/change class:** [verification] [patch]; effort S.
- **Status:** todo.

### PRO-CODEGEN-005 — Make the codegen-equivalence evidence real or narrow it

- **Outcome:** the zero-cost claim rests on evidence of its own category.
- **Scope:** `tests/codegen_equivalence.rs`, CI, and the ADR verification
  lists. Non-goals: adding a criterion suite.
- **Acceptance oracle:** either a committed codegen comparison (disassembly or
  `cargo-llvm-lines`-class artifact for the `#[inline(never)]` typed and raw
  pairs at release opt-level) with its output recorded, plus a release-profile
  test pass; or the ADRs are revised to claim bitwise value equivalence only,
  which is what the file asserts today (`:76-79,90-93`).
- **Dependencies:** PRO-GATE-004 shares the CI edit.
- **Risk/change class:** [verification] [patch]; effort M.
- **Status:** todo.

### PRO-SEMVER-006 — Close the public enums for forward compatibility

- **Outcome:** adding a property family or coefficient order is not a breaking
  change for consumers.
- **Scope:** `#[non_exhaustive]` on `PropertyKind`, `PropertyConstraint`
  (`src/property/error.rs:5,16`), `CoefficientOrder`, and `TemperatureRole`
  (`src/constitutive/temperature/error.rs:7,16`). Non-goals: the already
  non-exhaustive `TemperatureLawError` and `TemperatureValidity`.
- **Acceptance oracle:** all six public enums carry the attribute; in-repo
  matches compile; the change is classified by `cargo-semver-checks`.
- **Dependencies:** none.
- **Risk/change class:** [arch] [major] (adding `#[non_exhaustive]` breaks
  external exhaustive matches); effort S.
- **Status:** todo.

### PRO-GENERIC-007 — Instantiate the boundary suites at every supported scalar

- **Outcome:** the validity-boundary and calibration-domain guarantees are
  verified for each scalar the crate ships, not only `f64`.
- **Scope:** `tests/properties.rs`, the calibration-domain tests in
  `tests/temperature_law.rs`, `tests/composition.rs`. Non-goals: the codegen
  fixture, which is deliberately `f64`-concrete.
- **Acceptance oracle:** each boundary assertion runs through a generic
  `fn assert_*<T: RealField>()` instantiated at `f32` and `f64`, matching the
  existing pattern at `tests/theorems.rs:33-36`.
- **Dependencies:** none.
- **Risk/change class:** [verification] [patch]; effort S.
- **Status:** todo.

### PRO-DOC-008 — Ground the cross-repository ownership claim

- **Outcome:** the book states only what this repository can evidence.
- **Scope:** `docs/book/stack_position.md:28-31`. Non-goals: editing consumer
  repositories.
- **Acceptance oracle:** the assertion that "no duplicate material-property
  implementation exists in Hyperion, Helios, CFDrs, or Kwavers" is replaced by
  the boundary statement plus a link to the consumer contract test that
  evidences it, or is dated and attributed to the Atlas board item that
  verified it. Must not land while a book CI branch is in flight.
- **Dependencies:** the open consumer-contract-test item below.
- **Risk/change class:** [docs] [patch]; effort S.
- **Status:** todo.

### PRO-SEAL-009 — Record or lift the response-set seal

- **Outcome:** the closure of `ThermophysicalResponseSet` is a recorded
  decision rather than an undocumented seal.
- **Scope:** `src/constitutive/temperature/law.rs:12-14,68-71` and ADR 0002.
- **Acceptance oracle:** ADR 0002 gains a dated revision note stating that the
  set-level seam is sealed and that consumer variation enters through the
  unsealed `TemperatureResponse` trait; or the seal is lifted so a correlated
  or tabulated response set can be supplied downstream.
- **Dependencies:** none.
- **Risk/change class:** [arch] [patch]; effort S.
- **Status:** todo.

## Open

### PRO-GATE-004 — Reconcile CI with ADR verification claims [verification] [patch] — in progress

- **Owner:** Codex; branch `fix/proteus-gates-reconcile`.
- **Scope:** `.github/workflows/ci.yml`, `docs/adr/0001-thermophysical-material-boundary.md`,
  and `docs/adr/0002-temperature-response-law.md`.
- **Non-goals:** shared Pages workflow, provider implementation, and release
  publication.
- **Acceptance:** CI runs the SemVer check claimed by ADR 0002, builds at the
  declared Rust 1.95 MSRV floor, and executes both shipped examples; ADR
  verification lists remain synchronized with the runnable gates.
- **Evidence:** YAML parsing passed; Rust 1.95 all-target check, Rust 1.97
  format/clippy/Nextest (20/20), doctests (1/1), rustdoc, both examples, and
  `cargo-semver-checks` against `origin/main` (196 checks, 196 pass) passed
  locally. Hosted CI remains the delivery check after the branch is pushed.

- Add a direct consumer contract test when an Atlas consumer adopts the
  constitutive-law surface. The consumer owns the acceptance oracle; Proteus
  owns the provider implementation.
- Release publication remains external authority because `Cargo.toml` sets
  `publish = false`; no registry state is inferred from local package checks.
