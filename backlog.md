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

<a id="PRO-DOC-010"></a>
## PRO-DOC-010 — Make the stack chapter's consumer list and manifest wording true [patch]

- **Status:** todo; **priority:** verification; **milestone:** M-thermophysical-laws; **needs:** none.
- **basis:** `3d83811cd39a24a423317f0d2921dd01d6d85c22`. **Class:** [docs] [patch]; effort S. Prior attempt: `refs/pull/128/head`.
- **Outcome:** the rendered consumer list of `docs/book/stack_position.md` has one top-level item per consumer (Hyperion, CFDrs, Kwavers, Helios), each with a pinned link into its own repository only; the Helios item pins the evidence read below; the sentence above the list states that count. Hyperion's manifest is called a package manifest, no sentence calls it a workspace manifest, and none calls another consumer's manifest a package manifest.
- **Scope:** `docs/book/stack_position.md:29-61`: the count sentence `:29`, a Helios item after `:49`, the manifest sentence `:57-58`. Non-goals: editing consumer repositories; the three existing consumer bullets and their pins.
- **Read at the basis:** `:29` says "Three consumers" over three bullets. Helios `ea487343c0ada745402dec54e195d1a28f5acd30` consumes `proteus-mat` at `Cargo.toml:133` and `crates/helios-solver/src/attenuation_map.rs:14` (`use proteus::{InvalidProperty, MassDensity};`, test `:285`). `:57-58` says "Hyperion, CFDrs and Kwavers workspace manifests", but Hyperion `b840f71cabb398bd43c8ab5e93e5646380f3c065` has no `[workspace]` table (`Cargo.toml:18` is its dependency); CFDrs `b76f00a72132` and Kwavers `34fa3f47ae07` have one at `Cargo.toml:1`.
- **Acceptance oracle:** with mdbook 0.5.4 and `CL() ( D=$(mktemp -d); trap 'python -I -c "import shutil,sys; shutil.rmtree(sys.argv[1])" $D' EXIT; git archive $1 | tar -x -C $D; mdbook build $D/docs/book -d $D/o >/dev/null 2>&1 && python -I -c "import sys,re,xml.etree.ElementTree as E;r=E.fromstring('<r>'+re.sub(r'<(img|br|hr|input|wbr|col|source)\b([^>]*?)/?>',r'<\1\2/>',re.search(r'<main>(.*)</main>',open(sys.argv[1],encoding='utf-8').read(),re.S).group(1))+'</r>');B=('p','li','td','th','dd','dt','div','blockquote','h1','h2','h3','h4','h5','h6');Z=lambda e:(e.text or '')+''.join((' ' if c.tag in B or c.tag=='a' else Z(c))+(c.tail or '') for c in e);V=lambda e,n,p:any(' '.join(''.join(a.itertext()).split()) and re.fullmatch(r'https://github\.com/ryancinsight/'+n+r'/blob/'+p,a.get('href',''),re.I) for a in e.iter('a'));HY='b840f71cabb398bd43c8ab5e93e5646380f3c065';HE='ea487343c0ada745402dec54e195d1a28f5acd30';U=[u for u in r.iter() if u.tag in ('ul','ol') and V(u,'hyperion',HY+'/tests/.*')];L=U[0].findall('li') if U else [];W=dict(two=2,three=3,four=4,five=5,six=6);S=[int(w) if w.isdigit() else W[w.lower()] for w in re.findall(r'(?i)\b(two|three|four|five|six|\d+)\s+(?:\w+\s+){0,3}consumers',' '.join(''.join(r.itertext()).split()))];R=[sorted(n for n in ('cfdrs','helios','hyperion','kwavers') if V(l,n,r'[0-9a-f]{40}/.*')) for l in L];P=[' '.join(Z(e).split()) for e in r.iter() if e.tag in B];Q=r'Hyperion(?:\x27s|\u2019s)? (?:root )?package manifest';H=[t for p in P for s in re.split(r'(?<=[.!?])\s+',p) for t in [re.sub(Q,'',s,flags=re.I)] if (re.search('workspace manifest',t,re.I) and re.search('Hyperion',t,re.I)) or re.search('(CFDrs|Kwavers|Helios).*package manifest|package manifest.*(CFDrs|Kwavers|Helios)',t,re.I)];F=[k for k,ok in (('items',sorted(R)==[['cfdrs'],['helios'],['hyperion'],['kwavers']]),('count',bool(S) and set(S)=={len(L)}),('helios-pin',any(x==['helios'] and V(l,'helios',HE+r'/(Cargo\.toml#L133|crates/helios-solver/src/attenuation_map\.rs#L14)') for x,l in zip(R,L))),('hyperion-cargo',V(r,'hyperion',HY+r'/Cargo\.toml#L18')),('phrase',any(re.search(Q,p,re.I) for p in P)),('manifest',not H)) if not ok];print('PASS' if not F else ['FAIL']+F)" $D/o/stack_position.html )`: `CL <rev>` prints `PASS`, otherwise `['FAIL', ...]` naming each failed condition. `items`: the list holding the pinned Hyperion tests link has one item per consumer, each holding a pinned link with visible text into exactly one of the four repositories, each repository once. `count`: at least one count word or digit before "consumers", all equal to the item count. `helios-pin`: the Helios item links `Cargo.toml#L133` or `crates/helios-solver/src/attenuation_map.rs#L14` at the full Helios SHA above. `hyperion-cargo`: a link to `Cargo.toml#L18` at the full Hyperion SHA. `phrase`: "Hyperion('s) package manifest" in rendered prose. `manifest`: after deleting that phrase from each sentence, no sentence outside link text holds both "Hyperion" and "workspace manifest", or "package manifest" with CFDrs, Kwavers or Helios. The program removes its temporary directory on exit. At the basis it prints `['FAIL', 'items', 'helios-pin', 'phrase', 'manifest']`.
- **Rejected (run):** wrong-file Helios pins (`README.md#L1`, `Cargo.toml#L1`, `attenuation_map.rs#L1`) and empty link text on the Helios pins; "Five consumers" with an unpinned extra bullet (`['FAIL', 'items']`); a hybrid as an extra sentence, a bullet addition, an inline clause or a `;` clause, and "workspace manifests of Hyperion" (`manifest`); CFDrs and Kwavers called package manifests; a zero SHA and a right 12-hex prefix with a wrong tail; "Four" over three bullets; "Three" kept over four; the list deleted, nested, split in two, as a table or in a code fence; Helios in a second list or without a link; no count; a stale "other two consumers"; the publish paragraph deleted; the phrase only in a comment. Accepted (run): "Four", "4 consumers", "Four of the consumers", an ordered list, a loose list, "Hyperion's" (straight or curly), a `Cargo.toml#L133` Helios pin, a `Helios` URL spelling, "Two of them also run contract tests".
- **Not caught (run), judged by hand:** a Helios item whose prose does not describe the pinned lines; a sentence like "Its manifest is a workspace manifest." that names no consumer; the hybrid only inside link text; changes to the three existing bullets' pins.
- **Next step:** add the Helios item pinned at `ea487343c0ad` (`attenuation_map.rs:14`), change `:29` to "Four consumers", rewrite `:57-58`, run `CL`.

<a id="PRO-DOC-011"></a>
## PRO-DOC-011 — Pin what Kwavers, CFDrs and Helios retain [patch]

- **Status:** todo; **priority:** verification; **milestone:** M-thermophysical-laws; **needs:** none.
- **basis:** `3d83811cd39a24a423317f0d2921dd01d6d85c22`. **Class:** [docs] [patch]; effort S. Prior attempt: `refs/pull/128/head`.
- **Outcome:** at every occurrence of "<name> retains" in `docs/book/stack_position.md` and `README.md`, each clause is linked, with visible text and on or right after the clause words, to that clause's own evidence file at the revision below: acoustic attenuation to `power_law.rs`, optical response to `optical.rs`, perfusion to `bioheat.rs`, fluid rheology to `casson.rs`, photon interaction to `compton.rs`, CT calibration to `ct_calibration.rs`. "Flow closure" is not bound by this item: neither file defines the term, and its meaning is a separate question.
- **Scope:** `docs/book/stack_position.md:25-27`, `README.md:25-27`. Non-goals: editing consumer repositories; Kwavers adoption of the Proteus diffusivity.
- **Read at the basis:** neither sentence carries a URL, and no other page of `README.md` or `docs/book` says "retains" of these repositories. Evidence read at the pinned revision. Kwavers `34fa3f47ae07e28d433b10a22c746f2c982e042e`: `crates/kwavers-medium/src/absorption/power_law.rs` (`PowerLawAbsorption`), `crates/kwavers-medium/src/optical.rs` (`MediumOpticalProperties`), `crates/kwavers-physics/src/thermal/diffusion/bioheat.rs:76-81` (perfusion source). CFDrs `b76f00a72132087e6805516cac1b5c4a7c7e2ae3`: `crates/cfd-core/src/physics/fluid/non_newtonian/casson.rs`. Helios `ea487343c0ada745402dec54e195d1a28f5acd30`: `crates/helios-physics/src/compton.rs`, `crates/helios-physics/src/ct_calibration.rs`. Kwavers also computes k/(rho*c) itself outside tests (`crates/kwavers-medium/src/properties/material.rs:217`, `.../heterogeneous/tissue/implementation/thermal.rs:31`, `.../homogeneous/implementation/traits_physical.rs:111`, `crates/kwavers-solver/src/multiphysics/monolithic/config/physics.rs:63`), which the sentence does not claim and no link may imply.
- **Acceptance oracle:** with mdbook 0.5.4 and `RP() { python -I -c "import sys,re,xml.etree.ElementTree as E;r=E.fromstring('<r>'+re.sub(r'<(img|br|hr|input|wbr|col|source)\b([^>]*?)/?>',r'<\1\2/>',re.search(r'<main>(.*)</main>',open(sys.argv[1],encoding='utf-8').read(),re.S).group(1))+'</r>');B=('p','li','td','th','dd','dt','div','blockquote','h1','h2','h3','h4','h5','h6');T=lambda e:[e.text or '']+[x for c in e for x in ([' '] if c.tag in B else [c] if c.tag=='a' else T(c))+[c.tail or '']];S=lambda e:' '.join(''.join(x if isinstance(x,str) else '\ue000'+''.join(x.itertext()) for x in T(e)).split());A=lambda e:[x for x in T(e) if not isinstance(x,str)];Q={'kwavers':('34fa3f47ae07e28d433b10a22c746f2c982e042e',{'acoustic attenuation':r'crates/kwavers-medium/src/absorption/power_law\.rs','optical response':r'crates/kwavers-medium/src/optical\.rs','perfusion':r'crates/kwavers-physics/src/thermal/diffusion/bioheat\.rs'}),'cfdrs':('b76f00a72132087e6805516cac1b5c4a7c7e2ae3',{'fluid rheology':r'crates/cfd-core/src/physics/fluid/non_newtonian/casson\.rs'}),'helios':('ea487343c0ada745402dec54e195d1a28f5acd30',{'photon interaction':r'crates/helios-physics/src/compton\.rs','CT calibration':r'crates/helios-physics/src/ct_calibration\.rs'})};Z=lambda s:[(n,p,s.lower().find(p.lower(),m.start())) for n in Q for m in re.finditer(r'\b'+n+' retains',s,re.I) for p in list(Q[n][1])+(['flow closure'] if n=='cfdrs' else [])];O=lambda e,n:(lambda s,a:(lambda M,f:all(q>=0 and any(q-1<=M[k]<min([z-1 for z in sorted(y[2] for y in f) if z>q]+[len(s)]+[q+x.start() for x in [re.search(r'[.!?](?:\s|$)',s[q:])] if x]) and ' '.join(''.join(a[k].itertext()).split()) and re.fullmatch(r'https://github\.com/ryancinsight/'+n+'/blob/'+Q[n][0]+'/'+Q[n][1][p]+r'(#L\d+(-L\d+)?)?',a[k].get('href',''),re.I) for k in range(len(a))) for m,p,q in f if m==n and p in Q[n][1]))([i for i,c in enumerate(s) if c=='\ue000'],Z(s)))(S(e),A(e));D=lambda n:[e for e in r.iter() if e.tag in B and re.search(r'\b'+n+' retains',S(e),re.I)];C=lambda n,t:len(re.findall(r'\b'+n+' retains',t,re.I));F=sorted(n for n in Q if not (D(n) and all(O(e,n) for e in D(n)) and C(n,' '.join(''.join(r.itertext()).split()))==sum(C(n,S(e)) for e in r.iter() if e.tag in B)));print('PASS' if not F else ['FAIL']+F)" "$1"; }`, `RB() ( D=$(mktemp -d); trap 'python -I -c "import shutil,sys; shutil.rmtree(sys.argv[1])" $D' EXIT; git archive $1 | tar -x -C $D; mdbook build $D/docs/book -d $D/o >/dev/null 2>&1 && RP $D/o/stack_position.html )`, `RD() ( D=$(mktemp -d); trap 'python -I -c "import shutil,sys; shutil.rmtree(sys.argv[1])" $D' EXIT; git show $1:README.md > $D/README.md; printf '# S\n\n[R](README.md)\n' > $D/SUMMARY.md; printf '[book]\nsrc = "."\n' > $D/book.toml; mdbook build $D -d $D/o >/dev/null 2>&1 && RP $D/o/index.html )`: `RB <rev>` (the chapter) and `RD <rev>` (the README as a one-chapter book) each print `PASS`, otherwise `['FAIL', ...]` naming each failing repository. A repository passes when it has at least one rendered block holding "<name> retains", every occurrence of that text on the page lies in a block the program reads, and for every occurrence each clause phrase after it is bound: an anchor with visible text lies from the phrase (an anchor wrapping the phrase counts) to the next clause phrase or the end of its sentence ("flow closure" counts as a phrase), with `href` `https://github.com/ryancinsight/<repo>/blob/<full SHA above>/<that clause's path>` and an optional `#L<n>` or `#L<n>-L<m>`. Links swapped between clauses, or listed after the last clause or its sentence, fail. Both programs remove their temporary directory on exit. At the basis both print `['FAIL', 'cfdrs', 'helios', 'kwavers']`.
- **Rejected (run), both files unless noted:** links swapped within Kwavers (three pairs), within Helios or within CFDrs, across repositories, or with the clause words linked; the rheology link moved onto "flow closure"; links listed after the last clause or after the sentence; a pinned sentence followed by the unpinned original in one block, or the reverse, or followed by a restated unpinned clause; the original left unpinned beside a paragraph that holds every pin; a pinned sentence beside a second unpinned "Kwavers retains" paragraph; an unpinned copy in a code fence; `README.md` or `lib.rs` pins; one clause unpinned; one repository pinned; links in a comment, in code spans or in a nameless paragraph; empty link text; "keeps" for "retains"; a branch name, a zero SHA or another commit for the SHA; the wrong repository name; an `http` URL; `tree/` instead of `blob/`; the sentence deleted; README pinned with the chapter unpinned (`RB` only); links in a nested sub-bullet beneath the unpinned original (`RB` only). Accepted (run): the intended links with or without a "flow closure" link, a "flow closure" link into `cavitation.rs`, three bullets or paragraphs, two pinned sentences in one block, the clause words themselves linked, an extra link, line fragments, the repository name as link text, "KWAVERS retains", the clauses reordered with links following them.
- **Not caught (run), judged by hand:** a clause whose prose is false while its links are right; a link label that names another file while the `href` is right; zero-width link text; line fragments (any `#L` range passes); the "flow closure" link, or its absence; "keeps" in the original with a pinned "retains" paragraph that repeats every clause phrase. Not read: CSS-hidden text and other pages of the book.
- **Next step:** pin each clause in both files with the links above, on or directly after the clause words and keeping the word "retains"; run `RB` and `RD`.

<a id="PROTEUS-RELEASE-MATRIX"></a>
## PROTEUS-RELEASE-MATRIX — proteus-mat 0.3.0 release feature and API matrix [major] — todo
- priority: verification; milestone: M-elastic-contracts; basis: `9bb6e29`.
- Outcome: Complete the release feature and API matrix for the published `proteus-mat 0.3.0`. The two provider lines this item inherits are done and evidenced: the published Aequitas 0.5 and Eunomia 0.12.1 identities are selected, and the published archive plus a fresh registry consumer were verified against the released provider graph after the downstream revisions merged. Only the matrix itself is open.
- Scope: `docs/`, `src/`, `tests/`.
- Acceptance oracle: every critical feature in the published 0.3.0 surface maps to an implemented API and a passing acceptance test on the released package; the matrix names the file and test for each.
- Next step: enumerate the matrix against [the incremental release baseline](gap_audit.md#incremental-release-baseline) and close each missing row.

<a id="PRO-DOC-012"></a>
## PRO-DOC-012 — Define "flow closure" in the stack chapter and README [patch]

- **Status:** todo; **priority:** verification; **milestone:** M-thermophysical-laws; **needs:** PRO-DOC-011.
- **basis:** `3d83811cd39a24a423317f0d2921dd01d6d85c22`. **Class:** [docs] [patch]; spike, effort S. Waits for PRO-DOC-011 only because both edit the same lines. `CC` reads the two files at a revision, never the board, so it runs unchanged before and after that item delivers; it requires that item's rheology link.
- **Outcome:** `README.md` and `docs/book/stack_position.md` say what CFDrs retains without the undefined term "flow closure": either the term is dropped and the rheology clause keeps its one `casson.rs` link, or each closure family meant is named and followed by a pinned link into its own CFDrs file; the delivering PR records the decision and its evidence.
- **Scope:** `README.md:25-27`, `docs/book/stack_position.md:25-27`. Non-goals: editing CFDrs; the evidence files of the other clauses.
- **Question:** what does "CFDrs retains fluid rheology and flow closure" cover? Neither file defines it. The wording is CFDrs's own: `CHANGELOG.md:1219-1220` at CFDrs `b76f00a72132087e6805516cac1b5c4a7c7e2ae3` reads "CFDrs retains rheology and flow closure while the shared thermophysical law has one Atlas owner", beside the change that moved `alpha = k / (rho c_p)` to Proteus.
- **Read at the basis:** "closure" (case-insensitive) occurs in 43 `.rs` files under `crates/` there, many of them Rust closures. The flow senses I read are turbulence closures (`crates/cfd-2d/src/physics/turbulence/constants.rs`, closure coefficients; `crates/cfd-2d/src/lib.rs:129-130`, "Turbulence and transport closures") and cavitation closures (`crates/cfd-3d/src/cavitation.rs:1,32,143`, `crates/cfd-core/src/physics/cavitation/nuclei_transport.rs:33`), so more than one reading is supported and no single file is the evidence.
- **Method:** read the commit behind that changelog entry and the CFDrs fluid-state code it changes; list the closure families it means; name each or drop the phrase. Evidence budget: that commit and the files it touches.
- **Acceptance oracle:** with `CC() { git show $1:$2 | python -I -c "import sys,re;t=' '.join(sys.stdin.read().replace(chr(96),'').split());R=r'\[([^\]]*)\]\(https://github\.com/ryancinsight/CFDrs/blob/([0-9a-f]{40})/(crates/[^)#\s]*)(?:#L\d+(?:-L\d+)?)?\)';K=('b76f00a72132087e6805516cac1b5c4a7c7e2ae3','crates/cfd-core/src/physics/fluid/non_newtonian/casson.rs');X=set('and or the a an of in for to as with also plus e g i other others various remaining related further additional all any some more fluid rheology closure closures model models cfdrs retains'.split());N=lambda u:all((lambda b:b and b.group(1) not in X)(re.search(r'([a-z]+)(?:[ -]models?)?[^a-z]*\Z',u[:m.start()])) for m in re.finditer('closures?',u));W=lambda T:re.fullmatch('c*',re.sub('w+l+',lambda m:'' if m.group().count('w')>=m.group().count('l') else '!',''.join('c' if w==chr(1) else 'l' if w==chr(2) else 'w' for w in T if w not in X))) is not None;ok=lambda s:(lambda P,u:not re.search('flow[ -]closure',s,re.I) and not re.search(r'\]\(',re.sub(R,'',s,flags=re.I)) and P.count(K)==1 and len({p for h,p in P})==len(P) and W(re.findall('[a-z]+|['+chr(1)+chr(2)+']',u)) and N(u))([(h.lower(),p.lower()) for a,h,p in re.findall(R,s,re.I)],re.sub(r'\S+\.rs\b','',re.sub(R,lambda m:m.group(1)+(' '+chr(1)+' ' if (m.group(2).lower(),m.group(3).lower())==K else ' '+chr(2)+' '),s,flags=re.I)).lower());M=[re.split(r'(?<!e\.g)(?<!i\.e)(?:; |\. |\.\Z)',t[m.start():],maxsplit=1)[0] for m in re.finditer('CFDrs retains',t,re.I)];print('PASS' if M and all(map(ok,M)) else ['FAIL',len(M),[ok(s) for s in M]])"; }`: `CC <rev> README.md` and `CC <rev> docs/book/stack_position.md` each print `PASS`, otherwise `['FAIL', clause-count, [verdict per clause]]`. It reads the raw text of every "CFDrs retains" clause (to the next "; " or ". ", not "e.g. " or "i.e. "), backticks removed, and passes when every clause: has no "flow closure" or "flow-closure"; holds exactly one link to `crates/cfd-core/src/physics/fluid/non_newtonian/casson.rs` at CFDrs `b76f00a72132087e6805516cac1b5c4a7c7e2ae3` (PRO-DOC-011's pin) and no link that is not a pinned CFDrs `crates/` link (any letter case, optional `#L` range), each file once; counts words outside a stop list (and, or, the, other, fluid, rheology, model, closure, ...) as family words and, reading left to right, has every run of links other than `casson.rs` directly preceded by a run of family words at least as long (no `casson.rs` link between them) and no family word left without such a link after it; and has no "closure" without a family word directly before it (ignoring "model"). At the basis it prints `['FAIL', 1, [False]]` for both files.
- **Accepted (run):** the term dropped with the one `casson.rs` link; a family named with a link after it ("turbulence-model closure", "cavitation" without the word closure, a family in the link text, families listed before the rheology clause); a link after each of two or three families; both links after both; one shared link after two families ("turbulence and transport closures ([lib.rs](...))", also "turbulence and cavitation closures ([constants.rs](...))" and "turbulence-model closure and cavitation closure ([cavitation.rs](...))", which a link omitted from the first family cannot be told from; whether the file evidences both is judged by hand); "fluid rheology and turbulence closure ([constants.rs](...), [casson.rs](...))"; "(e.g. [constants.rs](...))"; lowercase `cfdrs` URLs; `#L` ranges; two corrected sentences. **Rejected (run):** the term kept, hyphenated or split by a line break; a leftover "and flow"; "closure" or "other closures" with no family word, with or without a link; a family word with no link after it (the last family without its link; a family named with no link at all); a link with no family word before it since the previous link (a bare second link, also after "e.g."; a family's link put in the rheology parenthetical with the family named after it, even when another family's link follows); the `casson.rs` link repeated or at another SHA; the rheology link missing; the corrected clause beside the unchanged original, in either order; the sentence deleted; links that are unpinned, `http`, `tree/`, branch-named, in another repository or outside `crates/`.
- **Not caught (run):** whether a family word describes its file and whether the family list is complete (an arbitrary word such as "stuff", or another family's name, before a `cavitation.rs` link passes); both are judged against the decision recorded in the PR. Fails closed: a heading or sentence "What CFDrs retains" elsewhere in a file; a non-stop word added anywhere in a dropped clause ("Casson fluid rheology", "for blood flow"); a family's link after `casson.rs` in one parenthetical ("turbulence closure ([casson.rs](...), [constants.rs](...))"); bullets without terminal full stops, whose clause runs into the next bullet. The program reads raw text, not rendering.
- **Next step:** after PRO-DOC-011 lands, read the changelog entry's commit, then edit both files, record the decision in the PR and run `CC`.

<a id="PRO-DOC-009"></a>
## PRO-DOC-009 — Link the user manual and a first example from the entry points [patch]

- **Status:** todo; **priority:** verification; **milestone:** M-thermophysical-laws; **needs:** none.
- **basis:** `3d83811cd39a24a423317f0d2921dd01d6d85c22`. **Class:** [docs] [patch]; effort S.
- **Outcome:** the README (the crates.io page) renders links to the manual root and to the Constant Material example, by absolute Pages URL, and the book index renders links to `validated_properties.md` and `examples/constant_material.md`. Manual root and example page answer HTTP 200 (curl, 2026-11-28).
- **Scope:** `README.md`, `docs/book/README.md`. Source work: the commit at `refs/pull/127/head` adds a README manual line and a book "Start here" list and merges cleanly onto the basis; it names no example in the README. `examples/constant_material.rs` exists and `.github/workflows/ci.yml:55` runs it.
- **Acceptance oracle:** with mdbook 0.5.4 (pulldown-cmark) and `A() { python -I -c "import sys,re,xml.etree.ElementTree as E; r=E.fromstring('<r>'+re.sub(r'<(img|br|hr|input|wbr|col|source)\b([^>]*?)/?>',r'<\1\2/>',re.search(r'<main>(.*)</main>',open(sys.argv[1],encoding='utf-8').read(),re.S).group(1))+'</r>'); H=[a.get('href') for a in r.iter('a') if a.find('img') is None and ''.join(a.itertext()).strip()]; print([H.count(t) for t in sys.argv[2:]])" "$@"; }`, `R() { E=$(mktemp -d); git show $1:README.md > $E/README.md; printf '# S\n\n[R](README.md)\n' > $E/SUMMARY.md; printf '[book]\nsrc = "."\n' > $E/book.toml; mdbook build $E -d $E/o >/dev/null 2>&1 && A $E/o/index.html "${@:2}"; }`, `K() { D=$(mktemp -d); git archive $1 | tar -x -C $D; mdbook build $D/docs/book -d $D/o >/dev/null 2>&1 && A $D/o/index.html "${@:2}"; }`: `R <rev> https://ryancinsight.github.io/proteus/ https://ryancinsight.github.io/proteus/examples/constant_material.html` and `K <rev> validated_properties.html examples/constant_material.html` each print `[n, m]` with n and m 1 or more. The counts are of rendered `<a href>` elements inside `<main>` with that exact href and visible text, image anchors excluded. At the basis both print `[0, 0]`.
- **Rejected (run):** README: the rescued commit alone (`[1, 0]`); the example URL in a comment, code span, double-backtick span, backtick, tilde or nested fence, indented block, `title` attribute, `div`, `pre`, `script` or `textarea` block, an `<a>` without `href`, escaped brackets, empty link text, a bare URL, an `http` URL or a path typo (`[1, 0]` each); relative links (`[0, 0]`). Book index: the `examples/` prefix missing, the example commented out or in a code span (`[1, 0]`); the validated link in a tilde fence, `div` or indented block (`[0, 1]`). Accepted (run): reference-style, titled, angle-bracket, list, table, heading and raw-HTML anchors. Not tested: CSS-hidden anchors and zero-width link text pass (`[1, 1]`), and mdbook-only syntax is not excluded; a `./` prefix, a `#fragment` or a GitHub-only bare-URL autolink is rejected though it renders.
- **Next step:** fetch `pull/127/head`, port its commit, add the example link to the README line, run the oracle.

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
