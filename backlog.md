# Proteus backlog

Strategic roadmap; tags `[patch]`/`[minor]`/`[major]`/`[arch]` per SemVer class.
Proteus is the Atlas material-property and constitutive-law SSOT: validated
property boundaries, cohesive bundles, named materials, dimensionally typed
temperature response, and statically dispatched constitutive evaluation over
Aequitas quantities and Eunomia scalars.

## Delivered — thermophysical foundation (0.1.0)

- [x] [arch] Own the shared thermophysical material boundary: validated
  mass-density, specific-heat-capacity, and thermal-conductivity property
  newtypes over Aequitas quantities, with typed `InvalidProperty` failures.
  Decision: [ADR 0001](docs/adr/0001-thermophysical-material-boundary.md).
- [x] [minor] `ThermophysicalProperties` bundle and the derived thermal-
  diffusivity law `alpha = k / (rho * c_p)` — dimensionally reduced to
  `L²/T` with no raw-scalar escape.
- [x] [arch] GAT-based static `ConstitutiveLaw` seam with a generic associated
  state family; `ConstantLaw` over the zero-sized `NoState`; typed
  state/property failures.
- [x] [minor] `Material` identity with `Cow<str>` names: static catalogs
  borrow, runtime materials own, through one API.
- [x] [minor] Dimensionally typed constant, linear, and quadratic temperature
  response strategies; first/second-order coefficients retain `K⁻¹`/`K⁻²`
  dimensions; independent response types monomorphize density, heat-capacity,
  and conductivity behavior.
- [x] [minor] `TemperatureLaw` composing independent thermophysical responses
  through its GAT state, borrowing the current Aequitas thermodynamic
  temperature. Decision:
  [ADR 0002](docs/adr/0002-temperature-response-law.md).
- [x] [patch] Evaluated offsets consume Aequitas `TemperatureDifference`
  values while reference/evaluation states remain absolute thermodynamic
  temperatures (Unreleased changelog head).

## Delivered — dependency alignment and documentation

- [x] [patch] Advance the Aequitas pin to the merged thermal-diffusivity,
  photon-interaction, and biological-response quantity revisions so Proteus
  and Asclepius/Hyperion share one dimensional-type identity across the
  boundary; drop the stale revision qualifier once the sibling default
  merges.
- [x] [minor] Require Aequitas 0.2.0 and Eunomia 0.8.0 (default-features =
  false with a `std` feature) and refresh the lock onto the merged sibling
  heads (`2918e5a`, `3d6021e`, `6f7a5dc`).
- [x] [patch] Author and close the provider book
  (ATLAS-PROTEUS-PROVIDER-DOCS-001): validated properties, constitutive
  laws, and stack position with two runnable example pages. Evidence: link
  detector 0/0/0 and mdBook build (`30e25f8`, merged `0003266`).

## Deferred (documented boundary)

- [ ] [minor] crates.io publication. `proteus` remains `publish = false`
  because the crates.io `proteus` namespace is occupied by an unrelated
  project. Resolution options (package rename, e.g. `proteus-materials`) are
  recorded in the Atlas root SSOT; no stored credential is used.
- [ ] [arch] Mechanical and electrical constitutive laws beyond the
  thermophysical slice (elasticity, viscoelasticity, dielectric response).
  Gated on a driving Atlas consumer; the `ConstitutiveLaw` GAT seam is the
  extension point.
- [ ] [minor] Stateful and time-dependent constitutive response (hysteresis,
  relaxation) beyond the current stateless constant/linear/quadratic family.
- [ ] [patch] Fluid rheology, acoustic attenuation, optical response, and
  perfusion laws are **explicitly out of scope** — owned by CFDrs, Kwavers,
  Hyperion, and Helios respectively (ADR 0001). No duplication is planned.

## Boundary (does not duplicate)

Kwavers retains acoustic attenuation, optical response, and perfusion; CFDrs
retains fluid rheology and flow closure; Helios retains photon interaction
and CT calibration. Aequitas owns dimensions and units; Eunomia owns scalar
representations.
