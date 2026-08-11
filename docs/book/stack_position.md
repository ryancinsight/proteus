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
- Hyperion is the direct material consumer for mass-to-linear attenuation:
  `proteus::MassDensity` feeds `MassAttenuation::to_linear` in its coefficient
  seam, and no duplicate material-property implementation exists in Hyperion,
  Helios, CFDrs, or Kwavers.

The crate is `no_std + alloc`; `Material` uses `Cow<str>` so static catalogs
borrow names and runtime materials own names through one API. It contains no
array, scheduler, geometry, backend, or consumer dependency.

The provider is Git-first and intentionally `publish = false` because the
crates.io name is occupied by an unrelated owner. Atlas consumers pin reviewed
revisions and their lockfiles provide reproducible source identity. Registry
release work remains a separate external gate; this chapter describes the
stable ownership boundary rather than claiming publication.
