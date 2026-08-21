# Changelog

## Unreleased

### Changed

- **Verification:** CI now runs both shipped examples, checks the declared
  Rust 1.95 MSRV floor, and compares the public API with the change base using
  a pinned cargo-semver-checks action. Accepted ADRs now distinguish bitwise
  value evidence from unperformed release-code-generation evidence.

- **Book verification:** Hosted CI and the shared Pages build pass for both
  included material examples at PR #14; the pull-request Pages deployment is
  correctly skipped.

- **Book verification:** The two included material examples now declare their
  external crates for mdBook and run through the shared Atlas Pages package-
  staging gate with Rust 1.97.0. The three focused API fragments remain
  intentionally non-standalone.

- Add explicit `TemperatureValidity` calibration domains to temperature laws.
  Bounded laws validate the reference and every evaluation temperature and
  return a typed error instead of silently extrapolating beyond calibration
  data. The existing `TemperatureLaw::new` positive-temperature contract is
  preserved.

- Temperature-response laws now consume Aequitas `TemperatureDifference`
  values for evaluated offsets while retaining absolute thermodynamic
  temperatures for reference and evaluation states.

- Advance Aequitas to the merged photon-interaction quantity surface so
  downstream providers share one reciprocal-length, area-per-mass, and
  energy-per-area type identity.

### Fixed

- Pin Aequitas to its merged thermal-diffusivity revision so consumers resolve
  one dimensional-type identity across the Proteus boundary.
- Advance the Aequitas pin to the merged biological-response quantity revision
  so Proteus and Asclepius share one dimensional-type identity in integrators.

### Added

- Dimensionally typed constant, linear, and quadratic temperature responses;
  independent thermophysical response composition; and a GAT-based
  temperature constitutive law with typed state/property failures.
- Validated mass-density, specific-heat-capacity, and thermal-conductivity
  properties over Aequitas quantities.
- Thermophysical property composition and the derived thermal-diffusivity law.
- GAT-based static constitutive-law evaluation, a constant-law implementation,
  zero-sized state routing, and borrowed-or-owned material identity.
