# Position in the Atlas stack

Proteus is the Atlas provider for shared material properties, material
identity, and constitutive-law contracts.

```text
Aequitas  -> dimensions and SI units
Eunomia   -> RealField scalar and numeric law
    \
     Proteus -> validated properties, material identity, static laws
      |
Hyperion / Helios / CFDrs / Kwavers -> photon, thermal, and fluid consumers
```

Proteus consumes Aequitas quantities for its property newtypes and Eunomia's
`RealField` boundary for scalar arithmetic. It does not define a second units
vocabulary or scalar abstraction.

The ownership split is deliberate:

- Proteus owns material-property validity boundaries, cohesive property
  bundles, named material composition, dimensionally typed temperature-response
  strategies, and statically dispatched constitutive-law evaluation.
- Aequitas owns dimensions and units; Eunomia owns scalar representations.
- Kwavers retains acoustic attenuation, optical response, and perfusion; CFDrs
  retains fluid rheology and flow closure; Helios retains photon interaction
  and CT calibration. Proteus does not duplicate those domain laws.

Three consumers exercise the provider contract, each pinned to the commit
named in its link. This chapter states what those tests show and makes no
claim about the absence of material-property code elsewhere.

- Hyperion feeds `proteus::MassDensity` into `MassAttenuation::to_linear`
  ([`src/coefficient/mass.rs:34`](https://github.com/ryancinsight/hyperion/blob/b840f71cabb398bd43c8ab5e93e5646380f3c065/src/coefficient/mass.rs#L34)). Its
  contract tests compose a Proteus `ConstantLaw` material with that
  conversion for `f32` and `f64`
  ([`tests/contracts/transport_theorems.rs:255-283`](https://github.com/ryancinsight/hyperion/blob/b840f71cabb398bd43c8ab5e93e5646380f3c065/tests/contracts/transport_theorems.rs#L255-L283)).
- CFDrs `cfd-core` delegates thermal diffusivity and a linear
  temperature-dependent density to Proteus, with a closed-form check of the
  density
  ([`thermophysical.rs:17`](https://github.com/ryancinsight/CFDrs/blob/b76f00a72132087e6805516cac1b5c4a7c7e2ae3/crates/cfd-core/src/physics/fluid/thermophysical.rs#L17),
  [`:33`](https://github.com/ryancinsight/CFDrs/blob/b76f00a72132087e6805516cac1b5c4a7c7e2ae3/crates/cfd-core/src/physics/fluid/thermophysical.rs#L33),
  [`:102`](https://github.com/ryancinsight/CFDrs/blob/b76f00a72132087e6805516cac1b5c4a7c7e2ae3/crates/cfd-core/src/physics/fluid/thermophysical.rs#L102)).
- Kwavers `kwavers-physics` evaluates a Proteus `TemperatureLaw` in
  `update_properties`, with tests for the direction of the property changes
  and for rejection of a non-finite temperature
  ([`thermal/properties.rs:87`](https://github.com/ryancinsight/kwavers/blob/34fa3f47ae07e28d433b10a22c746f2c982e042e/crates/kwavers-physics/src/thermal/properties.rs#L87),
  [`:175`](https://github.com/ryancinsight/kwavers/blob/34fa3f47ae07e28d433b10a22c746f2c982e042e/crates/kwavers-physics/src/thermal/properties.rs#L175),
  [`:290`](https://github.com/ryancinsight/kwavers/blob/34fa3f47ae07e28d433b10a22c746f2c982e042e/crates/kwavers-physics/src/thermal/properties.rs#L290)).

The crate is `no_std + alloc`; `Material` uses `Cow<str>` so static catalogs
borrow names and runtime materials own names through one API. It contains no
array, scheduler, geometry, backend, or consumer dependency.

The provider is published on crates.io as `proteus-mat`, because the
`proteus` crate name is already taken on crates.io; the library keeps the
import path `proteus`. At the commits linked above, the Hyperion, CFDrs and Kwavers
workspace manifests each depend on `proteus-mat` `0.3.0` from the registry
([Hyperion `Cargo.toml:18`](https://github.com/ryancinsight/hyperion/blob/b840f71cabb398bd43c8ab5e93e5646380f3c065/Cargo.toml#L18),
[CFDrs `Cargo.toml:45`](https://github.com/ryancinsight/CFDrs/blob/b76f00a72132087e6805516cac1b5c4a7c7e2ae3/Cargo.toml#L45),
[Kwavers `Cargo.toml:11`](https://github.com/ryancinsight/kwavers/blob/34fa3f47ae07e28d433b10a22c746f2c982e042e/Cargo.toml#L11)).
