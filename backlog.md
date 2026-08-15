# Proteus backlog

This board records provider-owned work. Cross-repository adoption remains in
the Atlas board.

## PROTEUS-PHASE-1-001 — Material and constitutive-law provider surface — closed

- Outcome: Proteus owns validated thermophysical properties, typed temperature
  response laws, constitutive evaluation, and borrowed-or-owned material names.
- Boundary: Aequitas owns dimensions and units; Eunomia owns scalar
  representations; consumers retain their domain-specific attenuation,
  perfusion, rheology, and photon-interaction laws.
- Evidence: provider default `f033a7eb8204b020bbdbfff970f710ef4836fa0e`;
  hosted CI and Pages runs `31860973878` and `31860973039` both pass at that
  exact head.

## Open

- Add a direct consumer contract test when an Atlas consumer adopts the
  constitutive-law surface. The consumer owns the acceptance oracle; Proteus
  owns the provider implementation.
- Release publication remains external authority because `Cargo.toml` sets
  `publish = false`; no registry state is inferred from local package checks.
