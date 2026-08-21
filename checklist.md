# Proteus checklist

## PRO-GATE-004 — Reconcile CI with ADR verification claims

- [x] Audit the two accepted ADR verification lists against the current
      workflow and example targets.
- [x] Add complete, bounded CI jobs for SemVer, Rust 1.95, and both examples;
      keep the existing nextest and documentation gates intact.
- [x] Validate workflow YAML, manifests, and focused local gates.
- [ ] Push the branch and record exact-head hosted results before closing.

## Provider adoption

- [x] Keep property validation at the Proteus boundary with Aequitas typed
      quantities.
- [x] Keep constitutive evaluation generic, GAT-based, and statically
      dispatched; do not add a vtable or a consumer-owned wrapper.
- [x] Verify the merged default with hosted CI and Pages at
      `3887eacda7bc2a6f4bd90b04693e7070f05a894d`.
- [x] Audit `src`, `tests`, and `examples` for placeholder markers and
      re-export shims; the exact fetched default has none.

## Re-open triggers

- A consumer adopts the provider and lacks a value-semantic contract test.
- The publish policy changes from `publish = false` under release authority.
