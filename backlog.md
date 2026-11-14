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
- Next step, post-publication acceptance of 0.1.1 (packaged from `0d9931c5`, the #87 merge; none
  recorded as discharged in the tree): source/package/CI gates, independent exact-head review,
  performance/resource and lifecycle acceptance. Re-open trigger: that packet is recorded at the
  exact head. A failure is remedied by a yank plus a forward version.

<a id="PRO-ELASTIC-DERIVED-RANGE"></a>
## PRO-ELASTIC-DERIVED-RANGE — Representable derived moduli [patch]

- Status: blocked; priority: correctness; milestone: M-elastic-contracts; needs: none.
- basis: `d23144bb`. Merged in [#87](https://github.com/ryancinsight/proteus/pull/87) on 2026-10-17.
- Outcome: derived E/nu/K/M are finite whenever the exact result fits the native scalar; true
  overflow keeps IEEE infinity; the Lame pair and public API are unchanged.
- Scope: `src/elastic/`, `tests/elastic/`; `docs/verification/elastic-range.md`, `elastic-guard.md`.
- Acceptance oracle: exact-rational oracle (78,163 pairs incl. 27,594 review inputs; 336 + 305
  fixtures): no domain/classification mismatch, E/nu/K within one ULP, exact M rounding.
- Next step, post-publication acceptance of 0.1.1 (packaged from `0d9931c5`, the #87 merge, guard
  integration included; looked up 2026-11-14T01:13Z). Re-open trigger: (1) and (2) recorded at the
  exact head. (1) Renewed review of the guard integration (prior review does not approve it);
  (2) performance/resource acceptance incl. peak stack/RSS; (3) lifecycle: Iris #80's denied-read
  hold and all protected holds stay. A failure is remedied by a yank plus a forward version.

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

<a id="PRO-DOC-008"></a>
### PRO-DOC-008 — Ground the cross-repository ownership claim

- **Status:** todo; **priority:** verification; **milestone:** M-thermophysical-laws; **needs:** none.
- **basis:** `d23144bb`. **Class:** [docs] [patch]; effort S. No book CI branch is open at the basis.
- **Outcome:** the book states only what this repository can evidence.
- **Scope:** `docs/book/stack_position.md:28-31` and `:37-41` (the `publish = false` rationale
  contradicts `Cargo.toml` and the registry). Non-goals: editing consumer repositories.
- **Acceptance oracle:** the "no duplicate material-property implementation exists in Hyperion,
  Helios, CFDrs, or Kwavers" assertion becomes the boundary statement plus links pinned to commits
  of the adopters' tests: Hyperion `tests/contracts/transport_theorems.rs`, CFDrs `cfd-core`
  `thermophysical.rs` (closed-form test), Kwavers `kwavers-physics` `thermal/properties.rs`; the
  `publish` sentence states the registry fact. Must not land while a book CI branch is in flight.
- **Next step:** read each adopter test at its pinned commit, then edit the two passages.

<a id="PRO-SEMVER-006"></a>
### PRO-SEMVER-006 — Close the public enums for forward compatibility

- **Status:** todo; **priority:** tightening; **milestone:** M-thermophysical-laws; **needs:** none.
- **basis:** `d23144bb`. **Class:** [arch] [major]; effort S. The planned 0.2.0 admits the break
  (0.1.x to 0.2.0 is incompatible under Cargo's leftmost-nonzero rule).
- **Outcome:** adding a property family or coefficient order is not a breaking change for consumers.
- **Scope:** `#[non_exhaustive]` on `PropertyKind`, `PropertyConstraint` (`src/property/error.rs:5,16`),
  `CoefficientOrder` and `TemperatureRole` (`src/constitutive/temperature/error.rs:7,16`).
  Non-goals: the already non-exhaustive `TemperatureLawError` and `TemperatureValidity`.
- **Acceptance oracle:** all six public enums carry the attribute; in-repo matches compile;
  `cargo-semver-checks` classifies the change as major.
- **Next step:** add the attribute to the four enums, fix in-repo matches, run the SemVer gate.

<a id="PRO-SEAL-009"></a>
### PRO-SEAL-009 — Record or lift the response-set seal

- **Status:** todo; **priority:** tightening; **milestone:** M-thermophysical-laws; **needs:** none.
- **basis:** `d23144bb`. **Class:** [arch] [patch]; effort S.
- **Outcome:** the closure of `ThermophysicalResponseSet` is a recorded decision, not an
  undocumented seal.
- **Scope:** `src/constitutive/temperature/law.rs:12-14,68-71` and ADR 0002.
- **Acceptance oracle:** ADR 0002 gains a dated revision note stating that the set-level seam is
  sealed and that consumer variation enters through the unsealed `TemperatureResponse` trait; or
  the seal is lifted so a correlated or tabulated response set can be supplied downstream.
- **Next step:** write the ADR 0002 revision note unless a consumer needs a tabulated set.

<a id="PRO-VOCAB-003"></a>
### PRO-VOCAB-003 — Decide whether Proteus owns a named-material vocabulary

- **Status:** todo; **priority:** tightening; **milestone:** M-thermophysical-laws; **needs:** none.
- **basis:** `d23144bb`. **Class:** [arch] [minor]; effort L. Informs PRO-PROV-002.
- **Outcome:** an ADR records, with a recommended option, whether the provider owns canonical
  material identity (a validated `MaterialId`-class newtype, optionally a cited catalog) or
  identity stays a consumer-supplied string.
- **Scope:** a new ADR (next free number; 0003 is the release-scope ADR) and, if ownership is
  accepted, `src/material/`. Non-goals: acoustic/optical/rheology families (consumer-owned, ADR 0001).
- **Acceptance oracle:** the ADR is indexed in `docs/adr/README.md` and states the decision against
  the bare `Cow<'name, str>` identity (`src/material/model.rs:10`) and the "material-identity
  vocabulary" stack role; if ownership is accepted, a typed constructor rejects invalid
  identifiers with a value-semantic test.
- **Next step:** draft the ADR (Proposed) with the recommended option.
