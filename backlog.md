# Proteus backlog

<a id="proteus-stable-qualification"></a>
## PROTEUS-STABLE-QUALIFICATION — proteus stable-release qualification [patch] — todo
- priority: verification; milestone: M-elastic-contracts; basis: `1ab9ef7492c7`.
- Outcome: Close the post-publication evidence for elastic validity and derived moduli. Follow the [release scope](docs/adr/0003-release-scope-1-0.md) and [milestone exits](backlog/milestones.md). Planning targets for `proteus-mat` are 0.3.1, then 0.4.0 only for additive material contracts, then 1.0.0. Verification alone can advance the current version.
- Representative manifest `proteus-mat 0.3.0`. Target 1.0.0.
- [ ] [patch] Replay PRO-ELASTIC-VALIDITY-RANGE and PRO-ELASTIC-DERIVED-RANGE using the exact-rational reference at the zero-bulk boundary, adjacent values, subnormals and true overflow.
- [ ] [patch] Attach independent review and measured time, peak stack and memory for the guard on the selected revision. Exercise it through an Ares constitutive calculation.
- [ ] [major] Complete the thermophysical law examples and elastic contract gates, then freeze the documented rejection behavior and validate the published material package.
- [ ] [patch] **0.3.1.** Check derived moduli against the rational oracle at the zero-bulk boundary, its neighbors and overflow. [Source and baseline](gap_audit.md#incremental-release-baseline).
- [ ] [minor] **0.4.0 only for additive material contracts.** Verify the property and elastic milestones against the reconciled package in Ares. Compare recovered stress with the constitutive reference and record units, bounds and rejection behavior.
- [ ] [major] Block 1.0 until every critical feature is fully implemented through the supported product path. Map each critical requirement, including thermophysical properties, isotropic conversions and invalid-material rejection, to implementation and passing acceptance evidence on the release candidate. Any missing implementation or required result keeps this item open.
- [ ] [major] **1.0.0-beta.1 → 1.0.0-rc.1 → 1.0.0.** Freeze the supported surface at beta. The downstream material caller reproduces thermophysical and elastic values using the same released package and feature graph. Repeat the applicable exits on the exact RC before stable promotion. Attach exact-revision evidence for the local exits and this member’s [Atlas qualification criteria](https://github.com/ryancinsight/atlas/blob/17b0124a7548ad81e817284afc5cb115c1d1bc78/docs/parity/repositories.json).

- [x] [patch] Verify the published proteus-mat 0.3.0 source and run a downstream value test. [Archive and consumer evidence](gap_audit.md#incremental-release-baseline).

This board records provider-owned work. Cross-repository adoption remains in
the Atlas board.

<a id="PRO-ELASTIC-VALIDITY-RANGE"></a>
## PRO-ELASTIC-VALIDITY-RANGE — Exact-sign bulk validity [patch]

- Status: blocked; priority: correctness; milestone: M-elastic-contracts; needs: none.
- basis: `d23144bb`. Merged in [#87](https://github.com/ryancinsight/proteus/pull/87) on 2026-10-17.
- Outcome: bulk validity uses the exact sign of `3lambda+2mu` by native FMA, with no widened
  arithmetic, saturation or tolerance (baseline `1681e9a` accepted exact `K<0` after `2mu`
  overflowed). Detail: `docs/verification/elastic-range.md`.
- Scope: `src/elastic/`, `tests/elastic/`.
- Acceptance oracle: exact-rational domain oracle at the zero-bulk boundary and its neighbours,
  rounded-two-thirds and subnormal cases, both scalar guards.
- Next step, post-publication acceptance of 0.1.1 (packaged from `0d9931c5`, the #87 merge; none
  recorded as discharged in the tree): source/package/CI gates, independent exact-head review,
  performance/resource and lifecycle acceptance. Re-open trigger: that packet is recorded at the
  exact head. A failure is remedied by a yank plus a forward version.

<a id="PRO-ELASTIC-DERIVED-RANGE"></a>
## PRO-ELASTIC-DERIVED-RANGE — Representable derived moduli [patch]

- Status: blocked; priority: correctness; milestone: M-elastic-contracts; needs: none.
- basis: `d23144bb`. Merged in [#87](https://github.com/ryancinsight/proteus/pull/87) on 2026-10-17.
- Outcome: derived E/nu/K/M are finite whenever the exact result fits the native scalar; true
  overflow keeps IEEE infinity; the Lame pair and public API are unchanged.
- Scope: `src/elastic/`, `tests/elastic/`; `docs/verification/elastic-range.md`, `elastic-guard.md`.
- Acceptance oracle: exact-rational oracle (78,163 pairs incl. 27,594 review inputs; 336 + 305
  fixtures): no domain/classification mismatch, E/nu/K within one ULP, exact M rounding.
- Next step, post-publication acceptance of 0.1.1 (packaged from `0d9931c5`, the #87 merge, guard
  integration included; looked up 2026-11-14T01:13Z). Re-open trigger: (1) and (2) recorded at the
  exact head. (1) Renewed review of the guard integration (prior review does not approve it);
  (2) performance/resource acceptance incl. peak stack/RSS; (3) lifecycle: Iris #80's denied-read
  hold and all protected holds stay. A failure is remedied by a yank plus a forward version.

<a id="PROTEUS-RELEASE-MATRIX"></a>
## PROTEUS-RELEASE-MATRIX — proteus-mat 0.3.0 release feature and API matrix [major] — todo
- priority: verification; milestone: M-elastic-contracts; basis: `9bb6e29`.
- Outcome: Complete the release feature and API matrix for the published `proteus-mat 0.3.0`. The two provider lines this item inherits are done and evidenced: the published Aequitas 0.5 and Eunomia 0.12.1 identities are selected, and the published archive plus a fresh registry consumer were verified against the released provider graph after the downstream revisions merged. Only the matrix itself is open.
- Scope: `docs/`, `src/`, `tests/`.
- Acceptance oracle: every critical feature in the published 0.3.0 surface maps to an implemented API and a passing acceptance test on the released package; the matrix names the file and test for each.
- Next step: enumerate the matrix against [the incremental release baseline](gap_audit.md#incremental-release-baseline) and close each missing row.

<a id="PRO-DOC-009"></a>
## PRO-DOC-009 — Link the user manual and a first example from the entry points [patch]

- **Status:** todo; **priority:** verification; **milestone:** M-thermophysical-laws; **needs:** none.
- **basis:** `3d83811cd39a24a423317f0d2921dd01d6d85c22`. **Class:** [docs] [patch]; effort S.
- **Outcome:** the README (the crates.io page) renders links to the manual root and to the Constant Material example, by absolute Pages URL, and the book index renders links to `validated_properties.md` and `examples/constant_material.md`. Manual root and example page answer HTTP 200 (curl, 2026-11-28).
- **Scope:** `README.md`, `docs/book/README.md`. Source work: the commit at `refs/pull/127/head` adds a README manual line and a book "Start here" list and merges cleanly onto the basis; it names no example in the README. `examples/constant_material.rs` exists and `.github/workflows/ci.yml:55` runs it.
- **Acceptance oracle:** with mdbook 0.5.4 (pulldown-cmark) and `A() { python -I -c "import sys,re,xml.etree.ElementTree as E; r=E.fromstring('<r>'+re.sub(r'<(img|br|hr|input|wbr|col|source)\b([^>]*?)/?>',r'<\1\2/>',re.search(r'<main>(.*)</main>',open(sys.argv[1],encoding='utf-8').read(),re.S).group(1))+'</r>'); H=[a.get('href') for a in r.iter('a') if a.find('img') is None and ''.join(a.itertext()).strip()]; print([H.count(t) for t in sys.argv[2:]])" "$@"; }`, `R() { E=$(mktemp -d); git show $1:README.md > $E/README.md; printf '# S\n\n[R](README.md)\n' > $E/SUMMARY.md; printf '[book]\nsrc = "."\n' > $E/book.toml; mdbook build $E -d $E/o >/dev/null 2>&1 && A $E/o/index.html "${@:2}"; }`, `K() { D=$(mktemp -d); git archive $1 | tar -x -C $D; mdbook build $D/docs/book -d $D/o >/dev/null 2>&1 && A $D/o/index.html "${@:2}"; }`: `R <rev> https://ryancinsight.github.io/proteus/ https://ryancinsight.github.io/proteus/examples/constant_material.html` and `K <rev> validated_properties.html examples/constant_material.html` each print `[n, m]` with n and m 1 or more. The counts are of rendered `<a href>` elements inside `<main>` with that exact href and visible text, image anchors excluded. At the basis both print `[0, 0]`.
- **Rejected (run):** README: the rescued commit alone (`[1, 0]`); the example URL in a comment, code span, double-backtick span, backtick, tilde or nested fence, indented block, `title` attribute, `div`, `pre`, `script` or `textarea` block, an `<a>` without `href`, escaped brackets, empty link text, a bare URL, an `http` URL or a path typo (`[1, 0]` each); relative links (`[0, 0]`). Book index: the `examples/` prefix missing, the example commented out or in a code span (`[1, 0]`); the validated link in a tilde fence, `div` or indented block (`[0, 1]`). Accepted (run): reference-style, titled, angle-bracket, list, table, heading and raw-HTML anchors. Not tested: CSS-hidden anchors and zero-width link text pass (`[1, 1]`), and mdbook-only syntax is not excluded; a `./` prefix, a `#fragment` or a GitHub-only bare-URL autolink is rejected though it renders.
- **Next step:** fetch `pull/127/head`, port its commit, add the example link to the README line, run the oracle.

<a id="PRO-DOC-010"></a>
## PRO-DOC-010 — Make the stack chapter's consumer list and manifest wording true [patch]

- **Status:** todo; **priority:** verification; **milestone:** M-thermophysical-laws; **needs:** none.
- **basis:** `3d83811cd39a24a423317f0d2921dd01d6d85c22`. **Class:** [docs] [patch]; effort S.
- **Outcome:** the rendered consumer list of `docs/book/stack_position.md` has one top-level item per consumer, Helios as its own item with a pinned link, and the sentence above states that number; the chapter names Hyperion's manifest a package manifest and keeps the pinned Hyperion `Cargo.toml` link.
- **Scope:** `docs/book/stack_position.md:29-61`: the count sentence `:29`, the Helios item after `:49`, the manifest sentence `:57-58`. Non-goals: editing consumer repositories.
- **Read at the basis:** `:29` says "Three consumers" over three bullets, but Helios `ea487343c0ad` also consumes `proteus-mat` (`Cargo.toml:133`, `crates/helios-solver/src/attenuation_map.rs:14`, test `:285`). `:57-58` says "Hyperion, CFDrs and Kwavers workspace manifests"; Hyperion `b840f71cabb3` has no `[workspace]` table, CFDrs `b76f00a72132` and Kwavers `34fa3f47ae07` have one at `Cargo.toml:1`.
- **Acceptance oracle:** with mdbook 0.5.4 and `S() { D=$(mktemp -d); git archive $1 | tar -x -C $D; mdbook build $D/docs/book -d $D/o >/dev/null 2>&1 && python -I -c "import sys,re,xml.etree.ElementTree as E; r=E.fromstring('<r>'+re.sub(r'<(img|br|hr|input|wbr|col|source)\b([^>]*?)/?>',r'<\1\2/>',re.search(r'<main>(.*)</main>',open(sys.argv[1],encoding='utf-8').read(),re.S).group(1))+'</r>'); T=' '.join(''.join(r.itertext()).split()); P=lambda e,s:[a.get('href','') for a in e.iter('a') if re.search(r'ryancinsight/'+s+r'/blob/[0-9a-f]{40}/',a.get('href',''),re.I)]; U=[u for u in r.iter() if u.tag in ('ul','ol') and any('/tests/' in h for h in P(u,'hyperion'))]; L=U[0].findall('li') if U else []; N=len(L); W=dict(two=2,three=3,four=4,five=5,six=6); S=[int(w) if w.isdigit() else W[w.lower()] for w in re.findall(r'(?i)\b(two|three|four|five|six|\d+)\s+(?:\w+\s+){0,3}consumers',T)]; O=[bool(P(l,'helios')) and not (P(l,'hyperion') or P(l,'CFDrs') or P(l,'kwavers')) for l in L]; M=bool(re.search(r'Hyperion(?:\x27s|\u2019s)? (?:root )?package manifest',T)); K=any('/Cargo.toml#L' in h for h in P(r,'hyperion')); print('PASS' if U and S and set(S)=={N} and any(O) and M and K else ['FAIL',N,S,O,M,K])" $D/o/stack_position.html; }`, `S <rev>` prints `PASS`. It requires: a top-level list holding the Hyperion tests link; every count word or digit before "consumers" equal to that list's item count, and at least one such count; an item holding a pinned Helios link and no other consumer's pin; the phrase "Hyperion package manifest" (or "Hyperion's"); a pinned Hyperion `Cargo.toml#L` link. At the basis it prints `['FAIL', 3, [3], [False, False, False], False, True]`.
- **Rejected (run):** "Four" over three bullets; "Three" kept over four; the list deleted, nested, split in two, or Helios in a second list; Helios in the Kwavers bullet or without a pin; no count; a stale "two consumers" sentence; "workspace manifests" kept, reworded in any order, or the phrase only in a comment; the publish paragraph deleted. Accepted (run): "Four", "4", "Four of the", an ordered list, a "Hyperion's" wording. Not tested: a fake SHA passes, and so does the package phrase beside a sentence still calling Hyperion's manifest a workspace manifest; both are judged by hand.
- **Next step:** add the Helios item pinned at `ea487343c0ad` (`attenuation_map.rs:14`), change `:29` to "Four consumers", rewrite `:57-58`.

<a id="PRO-DOC-011"></a>
## PRO-DOC-011 — Pin what Kwavers, CFDrs and Helios retain [patch]

- **Status:** todo; **priority:** verification; **milestone:** M-thermophysical-laws; **needs:** none.
- **basis:** `3d83811cd39a24a423317f0d2921dd01d6d85c22`. **Class:** [docs] [patch]; effort S.
- **Outcome:** each sentence saying what Kwavers, CFDrs and Helios retain, in `docs/book/stack_position.md` and `README.md`, renders a pinned link (full 40-hex SHA) into that repository.
- **Scope:** `docs/book/stack_position.md:25-27`, `README.md:25-27`. Non-goals: editing consumer repositories; Kwavers adoption of the Proteus diffusivity (an Atlas or Kwavers board matter).
- **Read at the basis:** neither sentence carries a URL. Kwavers `34fa3f47ae07` computes k/(rho*c) itself outside tests at `crates/kwavers-medium/src/homogeneous/implementation/traits_physical.rs:111`, `crates/kwavers-medium/src/heterogeneous/tissue/implementation/thermal.rs:31`, `crates/kwavers-medium/src/properties/material.rs:217`, `crates/kwavers-solver/src/multiphysics/monolithic/config/physics.rs:63` and five liquid-phase sites in `crates/kwavers-physics/src/acoustics/bubble_dynamics/` (`energy/heat_transfer.rs:182`, `imex_integration/stiffness.rs:12`, `imex_integration/thermal_mass_transfer.rs:44,68,89`), so a link may claim no more than the code shows.
- **Acceptance oracle:** with mdbook 0.5.4 and `P() { python -I -c "import sys,re,xml.etree.ElementTree as E; r=E.fromstring('<r>'+re.sub(r'<(img|br|hr|input|wbr|col|source)\b([^>]*?)/?>',r'<\1\2/>',re.search(r'<main>(.*)</main>',open(sys.argv[1],encoding='utf-8').read(),re.S).group(1))+'</r>'); B=[e for e in r.iter() if e.tag in ('p','li') and not any(c.tag in ('p','li') for c in e.iter() if c is not e)]; X=lambda e:' '.join(''.join(e.itertext()).split()); print(sorted(n.lower() for n in ('kwavers','CFDrs','helios') if any(re.search(n+' retains',X(e),re.I) and any(re.search(r'ryancinsight/'+n+r'/blob/[0-9a-f]{40}/',a.get('href',''),re.I) and X(a) for a in e.iter('a')) for e in B)) or 'FAIL no pinned retention sentence')" "$@"; }`, `S() { D=$(mktemp -d); git archive $1 | tar -x -C $D; mdbook build $D/docs/book -d $D/o >/dev/null 2>&1 && P $D/o/stack_position.html; }`, `R() { E=$(mktemp -d); git show $1:README.md > $E/README.md; printf '# S\n\n[R](README.md)\n' > $E/SUMMARY.md; printf '[book]\nsrc = "."\n' > $E/book.toml; mdbook build $E -d $E/o >/dev/null 2>&1 && P $E/o/index.html; }`: `S <rev>` and `R <rev>` print `['cfdrs', 'helios', 'kwavers']`. A name is listed when a rendered paragraph or item saying "<name> retains" holds an anchor with visible text pinned into that repository. At the basis both print `FAIL no pinned retention sentence`.
- **Rejected (run):** pins in a comment, a code span, a separate paragraph or a different "retains" sentence; empty link text; the sentence deleted or reworded ("keeps"); a branch name instead of a SHA; one or two repositories pinned (a shorter list). Accepted (run): two Kwavers pins, the clause split into three bullets. Not tested: a pin into the wrong file passes and is judged against the cited lines.
- **Next step:** read each retaining repository at its tip, pin one link per clause in both files, keeping the word "retains".

## Gap-audit 2026-08-20 items

Filed by the Atlas gap audit at `944eed05` from static evidence only; each
item cites its evidence in `gap_audit.md`. All are unclaimed.

<a id="PRO-PROV-002"></a>
### PRO-PROV-002 — Cite and bound every named isotropic solid

- **Outcome:** each `NamedIsotropicSolid` entry carries a resolvable citation
  with a table locator and an encoded temperature validity domain.
- **Scope:** `src/elastic/catalog.rs`, `tests/elastic/catalog.rs`, any future
  named material. Non-goals: PRO-VOCAB-003; arithmetic-oracle fixtures.
- **Acceptance oracle:** Rustdoc beside each catalog constant names source and
  table locator; a value test per entry asserts the cited value and an
  out-of-domain request returns a typed error; carbon steel names its source
  grade or is removed; the domain encoding is recorded in a design note first.
- **Status:** todo; **priority:** verification; **milestone:** M-elastic-contracts; **needs:** none;
  **basis:** `d23144bb`.
- **Risk/change class:** [verification] [patch], [minor] if the domain is a public accessor; effort L.
- **Next step:** design note for the domain encoding.

<a id="PRO-SEMVER-006"></a>
### PRO-SEMVER-006 — Close the public enums for forward compatibility

- **Status:** todo; **priority:** tightening; **milestone:** M-thermophysical-laws; **needs:** none.
- **basis:** `d23144bb`. **Class:** [arch] [major]; effort S. The planned 0.2.0 admits the break
  (0.1.x to 0.2.0 is incompatible under Cargo's leftmost-nonzero rule).
- **Outcome:** adding a property family or coefficient order is not a breaking change for consumers.
- **Scope:** `#[non_exhaustive]` on `PropertyKind`, `PropertyConstraint` (`src/property/error.rs:5,16`),
  `CoefficientOrder` and `TemperatureRole` (`src/constitutive/temperature/error.rs:7,16`).
  Non-goals: the already non-exhaustive `TemperatureLawError` and `TemperatureValidity`.
- **Acceptance oracle:** all six public enums carry the attribute; in-repo matches compile;
  `cargo-semver-checks` classifies the change as major.
- **Next step:** add the attribute to the four enums, fix in-repo matches, run the SemVer gate.

<a id="PRO-SEAL-009"></a>
### PRO-SEAL-009 — Record or lift the response-set seal

- **Status:** todo; **priority:** tightening; **milestone:** M-thermophysical-laws; **needs:** none.
- **basis:** `d23144bb`. **Class:** [arch] [patch]; effort S.
- **Outcome:** the closure of `ThermophysicalResponseSet` is a recorded decision, not an
  undocumented seal.
- **Scope:** `src/constitutive/temperature/law.rs:12-14,68-71` and ADR 0002.
- **Acceptance oracle:** ADR 0002 gains a dated revision note stating that the set-level seam is
  sealed and that consumer variation enters through the unsealed `TemperatureResponse` trait; or
  the seal is lifted so a correlated or tabulated response set can be supplied downstream.
- **Next step:** write the ADR 0002 revision note unless a consumer needs a tabulated set.

<a id="PRO-VOCAB-003"></a>
### PRO-VOCAB-003 — Decide whether Proteus owns a named-material vocabulary

- **Status:** todo; **priority:** tightening; **milestone:** M-thermophysical-laws; **needs:** none.
- **basis:** `d23144bb`. **Class:** [arch] [minor]; effort L. Informs PRO-PROV-002.
- **Outcome:** an ADR records, with a recommended option, whether the provider owns canonical
  material identity (a validated `MaterialId`-class newtype, optionally a cited catalog) or
  identity stays a consumer-supplied string.
- **Scope:** a new ADR (next free number; 0003 is the release-scope ADR) and, if ownership is
  accepted, `src/material/`. Non-goals: acoustic/optical/rheology families (consumer-owned, ADR 0001).
- **Acceptance oracle:** the ADR is indexed in `docs/adr/README.md` and states the decision against
  the bare `Cow<'name, str>` identity (`src/material/model.rs:10`) and the "material-identity
  vocabulary" stack role; if ownership is accepted, a typed constructor rejects invalid
  identifiers with a value-semantic test.
- **Next step:** draft the ADR (Proposed) with the recommended option.
