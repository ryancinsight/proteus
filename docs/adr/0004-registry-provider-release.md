# ADR 0004: Publish the Aequitas 0.4 material boundary

- Status: Accepted
- Date: 2026-11-20
- Class: `[major]`

## Context

Atlas applications exchange public Rust types across independently published crates. Cargo gives nominal types different identities when their package source or incompatible version differs. A fresh registry lock must select one identity for each type shared across the boundary.

## Decision

Publish proteus-mat 0.2.0 from the verified provider source. Material inputs and results use Aequitas 0.4 quantities and Eunomia 0.12 scalars. Consumers move their direct dependencies to these same registry identities. In particular, callers can pass Aequitas 0.4 mass density and thermophysical quantities directly.

## Verification

Run package, API, and feature checks on the release revision. Verify the archived source and checksum and compile a consumer from an empty lockfile against the indexed versions.
