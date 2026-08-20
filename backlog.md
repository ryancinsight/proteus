# Proteus backlog

This board records provider-owned work. Cross-repository adoption remains in
the Atlas board.

## PRO-DOC-BOOK-001 — Execute included book examples [patch] — implementation complete; hosted verification pending

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

- Add a direct consumer contract test when an Atlas consumer adopts the
  constitutive-law surface. The consumer owns the acceptance oracle; Proteus
  owns the provider implementation.
- Release publication remains external authority because `Cargo.toml` sets
  `publish = false`; no registry state is inferred from local package checks.
