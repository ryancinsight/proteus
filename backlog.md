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
