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

## gap-audit-2026-08-20 (owner: atlas-gap-audit)

Static audit at `944eed05`; no build, test, or lint command was run and no
source, manifest, or CI file was modified. Execution order for the filed items,
dependency-first:

1. [x] Re-establish the marker, shim, floor, and size scan at the current head
       and record it in `gap_audit.md`.
2. [x] Cross-check every README and Accepted-ADR verification claim against
       `.github/workflows/ci.yml` and the test tree; record each divergence.
3. [ ] PRO-GATE-004 — reconcile CI with the ADR verification claims
       (semver-checks, MSRV floor, second example), or strike the claims.
4. [ ] PRO-CODEGEN-005 — supply real codegen evidence or narrow the claim to
       the bitwise value equivalence the fixture actually asserts.
5. [ ] PRO-GENERIC-007 — instantiate the boundary suites at `f32` as well as
       `f64`.
6. [ ] PRO-PROV-002 — cite and bound the shipped `"generic tissue"` values, or
       rename them to a non-physical fixture.
7. [ ] PRO-SEMVER-006 — add `#[non_exhaustive]` to the four open public enums
       under a `[major]` classification.
8. [ ] PRO-SEAL-009 — record or lift the `ThermophysicalResponseSet` seal.
9. [ ] PRO-VOCAB-003 — draft ADR 0003 on canonical material identity; it
       governs whether the string-keyed `Material` stays the vocabulary.
10. [ ] PRO-DOC-008 — ground the cross-repository ownership claim in
        `stack_position.md`; hold until the book CI branch has landed.

Blocked here: none of the above may start while this branch carries the peer's
staged `Cargo.lock`; each item claims on the shared board first.
