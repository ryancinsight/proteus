# ADR 0004: Version the public scalar-provider transition

- Status: Proposed
- Change class: major, pre-1.0 release 0.2.0
- Date: 2026-11-04

## Context

Published `proteus-mat 0.1.1` contains source
`0d9931c5ab37f0e4d2aada040823daeb74ac8cd8` and depends on Eunomia 0.10.
Current default `bf850b12e857f8d423117b322adfa73ec3e5c865` already depends
on Eunomia 0.12 while retaining package version 0.1.1. These dependency
versions define distinct public `RealField` traits. This incompatibility
precedes the thermal-diffusivity numerical repair.

An unchanged real client demonstrates the public break:

```rust,ignore
pub fn diffusivity<T: eunomia::RealField>(
    properties: &proteus::ThermophysicalProperties<T>,
) -> T {
    properties.thermal_diffusivity().into_base()
}
```

With direct Eunomia 0.10, it compiles against published Proteus 0.1.1.
It fails with E0599 against the actual normalized current-default and
repair packages. Changing the client to Eunomia 0.12 compiles against
the normalized repair package. Testing normalized packages removes
Git-versus-registry source identity as an explanation for this result.
The 223 passing automated API checks did not detect this dependency-trait
break; they do not supersede the executable client evidence.

## Decision

Deliver the next package as 0.2.0 and document the breaking provider
transition. Retain current Eunomia 0.12, Aequitas requirements, and external
lock records. The thermal-diffusivity correction remains a patch-class
bug fix within that release; it adds no public signature or trait.

Generic consumers must align their Eunomia provider with Proteus and
update their Proteus requirement to admit 0.2. For source dependencies,
the shared scalar provider must also resolve to the same source identity.
Registry consumers use the normalized registry graph. A second provider
implementation, compatibility wrapper, or older provider pin would hide
the incompatibility rather than complete the transition.

## Delivery order and ownership

On 2026-11-04 the inspected default manifests of CFDrs, Helios, Kwavers,
and Hyperion all require Proteus 0.1; those ranges reject 0.2. This is a
verified minimum consumer set, not a complete reverse-dependency census.
Helios #183 already owns provider-cohort adoption; Hyperion #90 remains
held on its own consumer migration. Existing owner branches stay intact.

The provider repair may be delivered as a reviewed draft. Before merge,
the existing integration owners must qualify consumer requirements and
locks against the new provider, including generic scalar call sites and
their native/package gates. Coordinate any transitional compatible range
only after testing both admitted versions. Do not infer consumer acceptance
from provider tests or advance protected work in this branch.

Proteus main pushes invoke the pending-package publisher. Therefore merge
and publication remain held until final provider gates, independent review,
the consumer transition and release effects are qualified. No workflow,
publication setting, authentication, permission or gate changes implement
this decision. Automatic documentation deployment remains separately
allowed; a pull-request book build is not a production deployment receipt.

## Verification

- Preserve published archive checksum and VCS provenance.
- Preserve the passing old client, failing normalized-default and repaired
  clients, and passing migrated client with exact manifests and logs.
- Run API checks as well as the real generic-client boundary test.
- Qualify source and normalized package tests, MSRV, supply chain,
  documentation, normal hooks and exact-head hosted checks.
- Verify source, registry and consumer receipts before release closure.
