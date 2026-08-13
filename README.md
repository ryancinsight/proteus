# Proteus

Proteus is the Atlas owner for shared material properties and constitutive-law
contracts. Phase 1 provides validated isotropic thermophysical properties and
statically dispatched constant, linear, and quadratic temperature response.

The name refers to Proteus, the shape-changing Greek sea god.

## Boundary

Proteus owns:

- material-property validity boundaries;
- cohesive material property bundles;
- named material composition;
- dimensionally typed temperature-response strategies;
- statically dispatched constitutive-law evaluation.

Aequitas owns dimensions and units. Eunomia owns scalar representations.
Kwavers retains acoustic attenuation, optical response, and perfusion; `CFDrs`
retains fluid rheology and flow closure; Helios retains photon interaction and
CT calibration. Proteus does not duplicate those domain laws.

## Example

```rust
use aequitas::systems::si::quantities::{
    MassDensity as DensityQuantity,
    SpecificHeatCapacity as HeatCapacityQuantity,
    ThermalConductivity as ConductivityQuantity,
};
use proteus::{
    ConstantLaw, MassDensity, Material, NoState, SpecificHeatCapacity,
    ThermalConductivity, ThermophysicalProperties,
};

let properties = ThermophysicalProperties::new(
    MassDensity::new(DensityQuantity::from_base(1_000.0_f64))?,
    SpecificHeatCapacity::new(HeatCapacityQuantity::from_base(4_000.0_f64))?,
    ThermalConductivity::new(ConductivityQuantity::from_base(0.6_f64))?,
)?;
let water = Material::borrowed("reference liquid", ConstantLaw::new(properties));
let evaluated = match water.properties(NoState) {
    Ok(properties) => properties,
    Err(never) => match never {},
};

assert_eq!(evaluated.thermal_diffusivity().into_base(), 1.5e-7);
# Ok::<(), proteus::InvalidProperty<f64>>(())
```

## Architecture

```text
src/
├── constitutive/
│   ├── contract.rs       # GAT-based static constitutive seam
│   ├── constant.rs       # constant law and zero-sized state
│   └── temperature/
│       ├── response.rs   # constant/linear/quadratic response strategies
│       ├── law.rs        # independent thermophysical response composition
│       └── error.rs      # typed coefficient/state/property failures
├── material/
│   └── model.rs          # Cow-backed material identity + law
├── property/
│   ├── density.rs        # mass-density validity boundary
│   ├── heat_capacity.rs  # heat-capacity validity boundary
│   ├── conductivity.rs   # conductivity validity boundary
│   ├── error.rs          # typed validation failure
│   └── validation.rs     # canonical scalar predicates
└── thermophysical/
    └── model.rs          # property bundle and diffusivity law
```

`lib.rs` and every `mod.rs` are manifests. Property newtypes are transparent
over Aequitas quantities. `ConstitutiveLaw` uses a generic associated state
family so state-dependent implementations can borrow solver state. `ConstantLaw`
uses the zero-sized `NoState`. `Material` uses `Cow<str>` so static catalogs
borrow names and runtime materials own names through one API.

`TemperatureLaw` borrows the current Aequitas thermodynamic temperature through
its GAT state. First- and second-order coefficients retain K⁻¹ and K⁻²
dimensions, respectively. Independent response types monomorphize density,
heat-capacity, and conductivity behavior without runtime dispatch; invariant
properties use the zero-sized `ConstantResponse`. `TemperatureLaw::new` uses
the positive-temperature domain. A calibrated law uses
`TemperatureValidity::bounded(minimum, maximum)` with
`TemperatureLaw::with_validity`; both bounds are inclusive and every
evaluation outside them returns `TemperatureLawError::OutsideValidityDomain`.

## Mathematical evidence

For density `rho > 0`, specific heat `c_p > 0`, and conductivity `k >= 0`,

`alpha = k / (rho c_p)`.

The denominator is positive, so `alpha >= 0`. Aequitas dimensional algebra
reduces the result to `L^2/T`. Property tests cover positivity and linear
conductivity scaling; generic tests instantiate `f32` and `f64`; the codegen
fixture compares the typed and raw expressions bit-for-bit.

The boundary decisions, consumer overlap, proof obligations, and rejected
alternatives are recorded in
[ADR 0001](docs/adr/0001-thermophysical-material-boundary.md) and
[ADR 0002](docs/adr/0002-temperature-response-law.md).

## Verification

```text
cargo fmt --check
cargo check --no-default-features
cargo clippy --all-targets --all-features -- -D warnings
cargo nextest run --all-features
cargo test --doc --all-features
cargo doc --no-deps --all-features
cargo deny check
```

## License

Licensed under either the MIT License or Apache License 2.0.
