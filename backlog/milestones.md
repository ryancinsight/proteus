# Milestone ladder ([ADR 0003](../docs/adr/0003-release-scope-1-0.md))
<a id="M-thermophysical-laws"></a> **M-thermophysical-laws** (0.2.0) needs: (none) — thermophysical property and response evaluation.
- exit: evaluate validated material properties and diffusivity (`cargo test --locked --all-features --test properties` and `cargo test --locked --all-features --test theorems` pass).
- exit: evaluate temperature-dependent constitutive responses (`cargo test --locked --all-features --test temperature_law` passes).
- exit: run thermophysical material examples (`cargo run --locked --example constant_material --all-features` and `cargo run --locked --example temperature_material --all-features` exit successfully).
- standing: no open correctness item.
<a id="M-elastic-contracts"></a> **M-elastic-contracts** (0.3.0) needs: M-thermophysical-laws — elastic conversion and catalog contracts.
- exit: evaluate isotropic elastic identities and conversions (`cargo test --locked --all-features --test elastic` passes).
- exit: preserve elastic validity and derived-range behavior (`cargo test --locked --all-features --test elastic_guard` and `cargo test --locked --all-features --test elastic_range` pass).
- exit: resolve `PRO-ELASTIC-VALIDITY-RANGE` and `PRO-ELASTIC-DERIVED-RANGE` with landed evidence on the default branch.
- standing: no open correctness item.
<a id="M-1.0"></a> **M-1.0** (1.0.0) needs: M-elastic-contracts — stable material-law release (authority-gated).
- standing: `cargo fmt --all -- --check`, `cargo clippy --locked --all-targets --all-features -- -D warnings`, `cargo test --locked --all-features`, `cargo test --locked --doc --all-features`, and `cargo doc --locked --no-deps --all-features` pass; SemVer and publish dry-run gates required for the declared release pass; README, book, and rustdoc are current; no open correctness item.
