# Changelog

## Unreleased

### Changed

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
