# Isotropic elastic range repair (0.1.1)

## Contract and baseline

Baseline: `1681e9a48ce8de6837e34492412a06d114eb1184`.
`IsotropicModuli<T>` retains one finite Lame pair, with `mu > 0` and exact
`K = lambda + 2mu/3 > 0`. Negative lambda is admissible. The public API,
canonical pair, provider dependencies and typed nonfinite input errors remain.
There is no saturation, widened intermediate or configurable domain tolerance.

Original independent evidence has eight failing cases and five controls per
profile (debug/release/no-default), while the 44 existing tests pass. For
`s=2^126` (f32) or `s=2^1022` (f64), `lambda=-3s, mu=3s` is accepted even though
`K=-s`: the old `2mu` intermediate is infinity. Accepted states also lose
representable results:

- `lambda=0, mu=2^64` (f32) or `2^512` (f64): E should be `2mu`, not infinity.
- `lambda=mu=s`: nu should be `1/4`, not zero.
- `lambda=-s, mu=9s/4`: K should be `s/2` and M `7s/2`, both finite.

## Native arithmetic argument

Positive lambda and positive mu imply positive bulk without arithmetic.
For negative lambda, `K>0` is equivalent to `3lambda+2mu>0`. Finite `2mu` is an
exact binary scaling; a native FMA therefore rounds the complete linear
combination once. A nonzero exact combination of finite binary inputs is at
least one subnormal unit, so its sign cannot round to zero. If doubling mu
overflows, use the same-sign `3lambda/2+mu`: the coefficient 1.5 is exact and
the relevant cancellation operands are normal. No rounded 2/3 predicate is used.

For negative lambda, the fused numerator also evaluates K and E. Divide before
restoring a removed factor of two or multiplying by mu. On the admitted domain
`lambda+mu>mu/3`; the E/mu factor lies in (0,2), avoiding the old quadratic
product. For nonnegative lambda, E is `2mu + mu*lambda/(lambda+mu)`, using FMA
to retain small terms at the true overflow edge. A positive overflowing sum is
halved to compute the bounded ratio. Nu similarly halves its numerator before
division when its doubled denominator overflows, retaining subnormal outputs.
M uses one native FMA `2mu+lambda`, so cancellation can recover a finite result
even when doubling alone would overflow. Truly unrepresentable results retain
native IEEE overflow/underflow; no new Result error, clamp or sentinel is added.

Only exceptional overflow branches are outlined. Auxetic material is a normal
case and is not marked cold. There is no per-precision algorithm fork.

## Verification and scope

The 23 added tests preserve the original bodies and add exact/adjacent bulk
boundaries, rounded-two-thirds distinctions, subnormal and small-product cases,
ordinary identities, typed input guards, genuine overflow and an actual typed
compressional/shear-speed consumer of the recovered moduli. A source-baseline
replay runs the final test bodies against the unchanged implementation.

An external exact-rational oracle covers 20,148 pairs across both scalar types,
including exponent-spanning, boundary-adjacent, random, signed-zero and invalid
inputs. It interprets native inputs exactly, evaluates the domain/formulas with
Python Fraction and rounds reference results with integer nearest-even rules.
O2/O3 default and no-default API outputs agree. All constructor decisions match;
E/nu/K differ by at most one ULP in this corpus and M matches exact rounding.
The predeclared bound was eight ULP with no finite/nonfinite mismatch allowed.
This is bounded evidence, not exhaustive correct-rounding or platform coverage.

## Ordinary cost and resources

One ARM64 Mac, Rust 1.97, native f32/f64, standard O2/O3 and unchanged overflow
checks. Actual baseline/candidate public APIs are timed in runtime direct,
opaque indirect and 1024-input streaming contexts: constructor, each algebraic
accessor and constructor plus all four accessors. Setup and validation are
excluded; complete Results/typed outputs are blackboxed. Each profile has
2,016 samples, seven alternating samples/context, with 200,000 calls or
256x1024 stream calls. No task compilation/testing overlaps timing.

The single outlining hypothesis retains its source, binaries, assembly and raw
before/after CSV. Before outlining, ordinary deltas span -0.263 to +3.166 ns.
The final ordinary matrix spans -0.359 to +1.390 ns/call across APIs/workflows.
Not every context improves. Recovered construct-and-derive cases cost
1.788-4.374 ns in the sampled contexts; the baseline's wrong output is not a
throughput target. No optimality or universal timing claim is made.

Every sampled region requests zero heap allocation calls/bytes. State sizes
stay 8/16 bytes and Result sizes 12/24 bytes for f32/f64. These object/request
measurements are not peak-stack, RSS/cache, no_std-performance or solver bounds.

## Review and release holds

Source-baseline public API comparison, strict source gates and native feature/
profile tests accompany the draft. Exact-head independent review and explicit
performance/resource acceptance remain separate. The checked public sparse
index returns 404 for `proteus-mat`: first-publication identity, authentication,
registry dependency graph and publication scope require lifecycle verification.
No merge, tag, publish, authentication/settings change or service deployment is
part of this draft. Iris #80's denied-read hold and all protected holds remain.
