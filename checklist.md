# Proteus checklist

## Provider adoption

- [x] Keep property validation at the Proteus boundary with Aequitas typed
      quantities.
- [x] Keep constitutive evaluation generic, GAT-based, and statically
      dispatched; do not add a vtable or a consumer-owned wrapper.
- [x] Verify the merged default with hosted CI and Pages at
      `f033a7eb8204b020bbdbfff970f710ef4836fa0e`.
- [x] Audit `src`, `tests`, and `examples` for placeholder markers and
      re-export shims; the exact fetched default has none.

## Re-open triggers

- A consumer adopts the provider and lacks a value-semantic contract test.
- The publish policy changes from `publish = false` under release authority.
