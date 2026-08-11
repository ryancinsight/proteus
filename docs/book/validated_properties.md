# Validated properties

Proteus wraps Aequitas quantities in newtypes that reject invalid material
values at the construction boundary. Each property carries exactly one
validity predicate, so an invalid value cannot silently reach a consumer.

- `MassDensity` requires a finite, non-negative value. Zero is admitted
  because it represents vacuum or a calibrated voxel below the material floor;
  cohesive property bundles tighten this.
- `SpecificHeatCapacity` requires a finite, strictly positive value.
- `ThermalConductivity` requires a finite, non-negative value.

A rejected value returns the typed `InvalidProperty<T>`, which names the
`PropertyKind`, the rejected canonical-SI scalar, and the violated
`PropertyConstraint`. The property newtypes stay transparent over their
Aequitas quantity and expose `quantity()` / `into_quantity()` without
conversion.

`ThermophysicalProperties` composes the three properties into one cohesive
bundle. Its constructor additionally requires strictly positive density, so
the derived law is total. The following is a focused, non-standalone API
fragment:

```rust,ignore
use proteus::{MassDensity, SpecificHeatCapacity, ThermalConductivity,
              ThermophysicalProperties};

let properties = ThermophysicalProperties::new(
    MassDensity::new(DensityQuantity::from_base(1_000.0_f64))?,
    SpecificHeatCapacity::new(HeatCapacityQuantity::from_base(4_000.0_f64))?,
    ThermalConductivity::new(ConductivityQuantity::from_base(0.6_f64))?,
)?;
```

`try_from_quantities` performs the same composition directly from Aequitas
quantities. From the bundle, `thermal_diffusivity` derives
`alpha = k / (rho c_p)`. Because density and heat capacity are positive and
conductivity is non-negative, the result is finite and non-negative whenever
native scalar arithmetic does not overflow; Aequitas proves the result
dimension is `L²/T`.

The runnable [constant-material example](examples/constant_material.md) is the
compiled end-to-end demonstration of construction and evaluation.
