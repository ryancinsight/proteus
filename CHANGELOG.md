# Changelog

## Unreleased

### Added

- **Isotropic-elastic conversion contract.** `IsotropicModuli` owns the
  `(E, nu) <-> (lambda, mu) <-> (c_p, c_s)` conversion set over `T: RealField`,
  carrying the Lame pair as its single canonical state. Its validity domain is
  the positive-definite one, `mu > 0` and `K = lambda + 2mu/3 > 0`, which admits
  auxetic solids (`nu < 0`, `lambda < 0`) that a non-negative-`lambda` check
  would wrongly reject. Round trips, the `mu = E / (2(1 + nu))` and
  `K = E / (3(1 - 2nu))` closed forms, open-interval boundaries, non-finite
  input, and generic `f32`/`f64` instantiation carry executable evidence;
  round-trip tolerances are scaled by the derived cancellation condition number
  rather than a flat epsilon.

- **Named isotropic-solid catalog.** `NamedIsotropicSolid` publishes carbon
  steel, stainless steel 316L, aluminium 6061-T6, and Ti-6Al-4V with their
  elastic, thermophysical, and thermal-expansion constants. Each variant names
  one specific grade: the consumer catalogs this replaces both said "steel"
  while meaning different alloys, so entries stay grade-specific to keep that
  substitution unrepresentable.

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
