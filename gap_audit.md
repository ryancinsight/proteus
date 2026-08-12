# Proteus gap audit

## Closed gaps

### Duplicate thermophysical material law (resolved)

The stack audit found CFDrs, Helios, and Hyperion each carried local
thermophysical material implementations with overlapping validity rules.
Proteus now owns the shared contract; CFDrs delegates its shared
thermophysical path directly to Proteus with typed quantities, Helios
consumes `proteus::MassDensity` in its attenuation coefficient seam, and
Hyperion aligns to one quantity identity. No duplicate material-property
implementation remains in the Atlas graph.

### Raw-scalar property inputs (resolved)

Property constructors and the diffusivity law accepted or returned raw
scalars in early scaffolding. The boundary now types inputs and outputs as
Aequitas quantities end-to-end; scalars are extracted only inside validated
leaf predicates and the codegen equivalence fixture.

### Temperature-offset semantics (resolved)

Early temperature response used absolute temperatures for evaluated offsets,
which is ill-defined under the affine-temperature law. Evaluated offsets now
consume Aequitas `TemperatureDifference` while reference and evaluation
states remain absolute thermodynamic temperatures (Unreleased changelog
head).

### Stale Aequitas revision pin (resolved)

The manifest pinned a pre-merge Aequitas revision, splitting the
dimensional-type identity between Proteus and its siblings. The qualifier was
dropped once the sibling default merged (`9a8655d`, `6c002c2`, `2918e5a`),
restoring one resolved source identity per consumer lock.

## Deferred (documented boundary — see backlog.md)

- **crates.io publication** — `publish = false` because the `proteus`
  namespace is occupied; rename (e.g. `proteus-materials`) requires owner
  authorization. Recorded in the Atlas root SSOT.
- **Mechanical/electrical constitutive laws** — the thermophysical slice is
  Phase 1; the `ConstitutiveLaw` GAT seam is the extension point.
- **Stateful/time-dependent response** beyond the stateless
  constant/linear/quadratic family.

## Verified non-gaps (do not chase)

- **Domain laws stay with their owners** — Kwavers (acoustic attenuation,
  optical response, perfusion), CFDrs (fluid rheology, flow closure), Helios
  (photon interaction, CT calibration), Hyperion (Beer–Lambert coefficients).
  ADR 0001 records the boundary; no duplication is planned.
- **Thermal diffusivity positivity** — `rho > 0`, `c_p > 0`, `k >= 0`
  imply `alpha >= 0`; proven by theorem tests, not just asserted.
- **no_std** — builds and checks with `--no-default-features`; `std` is an
  additive feature.

## Current verified state (2026-08-12)

- Strict all-targets check: pass (warning-denied).
- Nextest: 18/18 (default features) at the provider head; all-feature gate
  re-verified in the Atlas foundation gate sweep.
- Doctests and rustdoc: pass; `cargo deny check`: clean.
- No `TODO`/`FIXME`/`unimplemented!` markers remain in `src/`.
