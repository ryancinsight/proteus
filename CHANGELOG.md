# Changelog

## Unreleased

### Fixed

- Pin Aequitas to its merged thermal-diffusivity revision so consumers resolve
  one dimensional-type identity across the Proteus boundary.

### Added

- Dimensionally typed constant, linear, and quadratic temperature responses;
  independent thermophysical response composition; and a GAT-based
  temperature constitutive law with typed state/property failures.
- Validated mass-density, specific-heat-capacity, and thermal-conductivity
  properties over Aequitas quantities.
- Thermophysical property composition and the derived thermal-diffusivity law.
- GAT-based static constitutive-law evaluation, a constant-law implementation,
  zero-sized state routing, and borrowed-or-owned material identity.
