# Proteus checklist

Target version: 0.1.0

Sprint phase: Closed — delivered 2026-08-12 (all committed scope verified green)

## PRO-001 [arch] — thermophysical material boundary

- [x] Define validated mass-density, specific-heat-capacity, and
      thermal-conductivity property newtypes over Aequitas quantities.
- [x] Enforce positivity/law validity boundaries at construction with typed
      `InvalidProperty` failures; canonical scalar predicates in one leaf
      module.
- [x] Record the boundary decision, consumer overlap, proof obligations, and
      rejected alternatives in ADR 0001.

## PRO-002 [minor] — property bundle and diffusivity law

- [x] Compose `ThermophysicalProperties` and derive thermal diffusivity
      `alpha = k / (rho * c_p)` through Aequitas dimensional algebra with no
      raw-scalar escape.
- [x] Prove `alpha >= 0` for `rho > 0`, `c_p > 0`, `k >= 0`; property tests
      cover positivity and linear conductivity scaling; codegen fixture
      compares typed and raw expressions bit-for-bit.

## PRO-003 [arch] — constitutive law seam

- [x] GAT-based static `ConstitutiveLaw` with generic associated state family
      so state-dependent implementations can borrow solver state.
- [x] `ConstantLaw` over the zero-sized `NoState`; `Material` with `Cow<str>`
      identity (borrowed catalogs / owned runtime names through one API).

## PRO-004 [minor] — temperature response

- [x] Constant, linear, and quadratic response strategies with `K⁻¹`/`K⁻²`
      coefficient dimensions.
- [x] Independent thermophysical response composition; invariant properties
      use the zero-sized `ConstantResponse`; typed coefficient/state/property
      failures.
- [x] `TemperatureLaw` borrows the current Aequitas thermodynamic temperature
      through its GAT state and consumes `TemperatureDifference` for
      evaluated offsets. ADR 0002.

## PRO-005 [patch] — dependency alignment

- [x] Align to Aequitas 0.2.0 / Eunomia 0.8.0 default-features = false with a
      `std` feature; refresh the lock onto merged sibling heads.
- [x] Drop the stale Aequitas revision qualifier once the sibling default
      merged; restore one resolved source identity.

## PRO-006 [patch] — provider book closure

- [x] Replace all three `Chapter prose deferred` placeholders with
      API-accurate prose (validated properties, constitutive laws, stack
      position) plus two runnable example pages.
- [x] Verify link detector 0/0/0 and the mdBook build; land the closure
      commit (`30e25f8`, merged `0003266`).

## PRO-007 [chore] — release gate

- [x] Confirm `publish = false` is intentional (occupied crates.io namespace)
      and recorded in the Atlas root SSOT; no stored credential is used.
- [ ] Revisit publication under a renamed package (e.g. `proteus-materials`)
      when owner authorizes the release path.
