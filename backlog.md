# Proteus backlog

This board records provider-owned work. Cross-repository adoption remains in
the Atlas board.

<a id="PRO-ELASTIC-VALIDITY-RANGE"></a>
## PRO-ELASTIC-VALIDITY-RANGE — Exact-sign bulk validity [patch]

- Status: blocked; priority: correctness; milestone: M-elastic-contracts; needs: none.
- basis: `d23144bb`. Merged in [#87](https://github.com/ryancinsight/proteus/pull/87) on 2026-10-17.
- Outcome: bulk validity uses the exact sign of `3lambda+2mu` by native FMA, with no widened
  arithmetic, saturation or tolerance (baseline `1681e9a` accepted exact `K<0` after `2mu`
  overflowed). Detail: `docs/verification/elastic-range.md`.
- Scope: `src/elastic/`, `tests/elastic/`.
- Acceptance oracle: exact-rational domain oracle at the zero-bulk boundary and its neighbours,
  rounded-two-thirds and subnormal cases, both scalar guards.
- Next step, open holds (none recorded as discharged in the tree): final source/package/CI
  gates, independent exact-head correctness review, performance/resource acceptance. Re-open
  trigger: that packet is recorded at the exact head. Publication is not part of this item.

<a id="PRO-ELASTIC-DERIVED-RANGE"></a>
## PRO-ELASTIC-DERIVED-RANGE — Representable derived moduli [patch]

- Status: blocked; priority: correctness; milestone: M-elastic-contracts; needs: none.
- basis: `d23144bb`. Merged in [#87](https://github.com/ryancinsight/proteus/pull/87) on 2026-10-17.
- Outcome: derived E/nu/K/M are finite whenever the exact result fits the native scalar; true
  overflow keeps IEEE infinity; the Lame pair and public API are unchanged.
- Scope: `src/elastic/`, `tests/elastic/`; `docs/verification/elastic-range.md`, `elastic-guard.md`.
- Acceptance oracle: exact-rational oracle (78,163 pairs incl. 27,594 review inputs; 336 + 305
  fixtures): no domain/classification mismatch, E/nu/K within one ULP, exact M rounding.
- Next step, open holds; re-open trigger: (1) and (2) recorded at the exact head. (1) Renewed
  review of the guard integration (prior review does not approve it); (2) performance/resource
  acceptance incl. peak stack/RSS; (3) lifecycle: `proteus-mat` 0.1.1 is on the registry (looked up
  2026-11-14T00:45Z), so first publication is moot; releasing the guard integration needs release
  authority; keep Iris #80's denied-read hold and all protected holds.

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

<a id="PRO-PROV-002"></a>
### PRO-PROV-002 — Cite and bound every named isotropic solid

- **Outcome:** each `NamedIsotropicSolid` entry carries a resolvable citation
  with a table locator and an encoded temperature validity domain.
- **Scope:** `src/elastic/catalog.rs`, `tests/elastic/catalog.rs`, any future
  named material. Non-goals: PRO-VOCAB-003; arithmetic-oracle fixtures.
- **Acceptance oracle:** Rustdoc beside each catalog constant names source and
  table locator; a value test per entry asserts the cited value and an
  out-of-domain request returns a typed error; carbon steel names its source
  grade or is removed; the domain encoding is recorded in a design note first.
- **Status:** todo; **priority:** verification; **milestone:** M-elastic-contracts; **needs:** none;
  **basis:** `d23144bb`.
- **Risk/change class:** [verification] [patch], [minor] if the domain is a public accessor; effort L.
- **Next step:** design note for the domain encoding.

<a id="PRO-PROV-010"></a>
### PRO-PROV-010 — Cite or rename the "reference liquid" example

- **Outcome:** the `"reference liquid"` example (a `ConstantLaw` over `NoState`, so no temperature
  state to bound) is cited or renamed to a non-physical fixture; split from PRO-PROV-002.
- **Scope:** `examples/constant_material.rs`, `README.md:52`,
  `docs/book/examples/constant_material.md`. Non-goals: PRO-PROV-002; arithmetic-oracle fixtures.
- **Acceptance oracle:** a source comment cites source and table locator for
  1000 / 4000 / 0.6, or `git grep -n -i -E 'reference liquid|water' --
  examples/constant_material.rs README.md docs/book/examples/constant_material.md`
  is empty (it is not at `d23144bb`).
- **Status:** todo; **priority:** verification; **milestone:** M-thermophysical-laws; **needs:** none;
  **basis:** `d23144bb`.
- **Risk/change class:** [verification] [patch]; effort S.
- **Next step:** cite or rename, then run the oracle grep.

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

<a id="PRO-CODEGEN-005"></a>
### PRO-CODEGEN-005 — Make the codegen-equivalence evidence real or narrow it

- **Outcome:** the zero-cost claim rests on evidence of its own category.
- **Scope:** `tests/codegen_equivalence.rs`, CI, and the ADR verification
  lists. Non-goals: adding a criterion suite.
- **Acceptance oracle:** either a committed codegen comparison (disassembly or
  `cargo-llvm-lines`-class artifact for the `#[inline(never)]` typed and raw
  pairs at release opt-level) with its output recorded, plus a release-profile
  test pass; or the ADRs are revised to claim bitwise value equivalence only,
  which is what the file asserts today (`:76-79,90-93`).
- **Dependencies:** none; the CI edit it shared with PRO-GATE-004 landed in
  [#17](https://github.com/ryancinsight/proteus/pull/17).
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

<a id="PRO-GENERIC-007"></a>
### PRO-GENERIC-007 — Instantiate the boundary suites at every supported scalar

- **Outcome:** the validity-boundary and calibration-domain guarantees are
  verified for each scalar the crate ships, not only `f64`.
- **Scope:** `tests/properties.rs`, the `f64`-only calibration-domain tests in
  `tests/temperature_law.rs`, `tests/composition.rs`. Non-goals: the codegen
  fixture, which is deliberately `f64`-concrete.
- **Acceptance oracle:** each boundary assertion runs through a generic
  `fn assert_*<T: RealField>()` instantiated at `f32` and `f64`, matching the
  existing pattern at `tests/theorems.rs:7-18`.
- **Dependencies:** none.
- **Risk/change class:** [verification] [patch]; effort S.
- **Status:** todo.

<a id="PRO-DOC-008"></a>
### PRO-DOC-008 — Ground the cross-repository ownership claim

- **Outcome:** the book states only what this repository can evidence.
- **Scope:** `docs/book/stack_position.md:28-31`. Non-goals: editing consumer
  repositories.
- **Acceptance oracle:** the assertion that "no duplicate material-property
  implementation exists in Hyperion, Helios, CFDrs, or Kwavers" is replaced by
  the boundary statement plus a link to the consumer contract test that
  evidences it, or is dated and attributed to the Atlas board item that
  verified it. Must not land while a book CI branch is in flight.
- **Dependencies:** PRO-CONSUMER-011.
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

<a id="PRO-CONSUMER-011"></a>
### PRO-CONSUMER-011 — Add a consumer contract test for the constitutive-law surface

- **Outcome:** an Atlas consumer tests the Proteus constitutive-law surface
  directly; provider presence is not integration proof.
- **Scope:** the adopting consumer's tests; the consumer owns the acceptance
  oracle and Proteus owns the provider implementation. Non-goals: editing
  consumer repositories from here.
- **Acceptance oracle:** the consumer's contract test merges and PRO-DOC-008
  cites it.
- **priority:** verification; **milestone:** M-1.0; **needs:** none.
- **Status:** blocked. Blocker: no Atlas consumer has adopted the
  constitutive-law surface. Re-open trigger: a consumer adopts it.
