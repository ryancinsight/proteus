# ADR 0003: Proteus release scope for 1.0

- Status: Accepted
- Date: 2026-11-10
- Class: `[arch]` `[minor]`

## Context

Proteus (`proteus-mat` 0.1.1) provides validated thermophysical properties,
temperature-dependent constitutive laws, and isotropic elastic contracts for
the Atlas stack. The backlog contains open correctness and verification items,
and no milestone ladder exists to 1.0.

## Decision

Adopt a three-milestone ladder to 1.0 recorded in `backlog/milestones.md`:

- **M-thermophysical-laws** (0.2.0): users evaluate validated thermophysical
  properties and temperature-dependent constitutive responses.
- **M-elastic-contracts** (0.3.0): users evaluate isotropic elastic conversions
  and catalog-backed material contracts with verified numeric behavior.
- **M-1.0** (1.0.0): stable material-law surface (authority-gated) with
  documented contracts and release gates satisfied.

## Alternatives

- Leave progress implicit in backlog items: rejected because stack policy
  requires a runnable capability ladder.
- Target 1.0 in one step: rejected because open correctness and verification
  work remains.

## Consequences

- Open items carry `milestone:` assignments and feature items name the exit
  check they serve.
- Milestones close only when exit checks pass on fetched default commits; 1.0
  release/publication remains authority-gated.
- This ADR is rewritten in place as scope evidence changes.
