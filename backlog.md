# Proteus backlog

This board records provider-owned work. Cross-repository adoption remains in
the Atlas board.

## PROTEUS-PHASE-1-001 — Material and constitutive-law provider surface — closed

- Outcome: Proteus owns validated thermophysical properties, typed temperature
  response laws, constitutive evaluation, and borrowed-or-owned material names.
- Boundary: Aequitas owns dimensions and units; Eunomia owns scalar
  representations; consumers retain their domain-specific attenuation,
  perfusion, rheology, and photon-interaction laws.
- Evidence: provider default `7fb5109aa56195eccd407b83e7b22406ab689d73`;
  hosted CI and Pages runs `31820651932` and `31820650749` both pass at that
  exact head.

## Open

- Add a direct consumer contract test when an Atlas consumer adopts the
  constitutive-law surface. The consumer owns the acceptance oracle; Proteus
  owns the provider implementation.
- Release publication remains external authority because `Cargo.toml` sets
  `publish = false`; no registry state is inferred from local package checks.
