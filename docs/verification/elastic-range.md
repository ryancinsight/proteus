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
even when doubling alone would overflow. Results past the IEEE rounding range retain
native overflow/underflow; no new Result error, clamp or sentinel is added.

Only exceptional overflow branches are outlined. Auxetic material is a normal
case and is not marked cold. There is no per-precision algorithm fork.

### Overflow midpoint continuation

Independent review of `d7ed3f8` demonstrates two remaining P2 failures: a
rounded Young factor crosses the final multiplication's overflow threshold,
and rounding mu/3 before adding bulk crosses the IEEE overflow midpoint.
The latter exact K is MAX + ulp(MAX)/6: it exceeds MAX as a real number but
correctly rounds to finite MAX, below MAX + ulp(MAX)/2. The former also has
positive-lambda cases that return finite E when exact rounding is infinity.
The historical 20,148-case oracle misses these boundaries.

The repair routes only large derived-value evaluations to a private native-T
residual kernel. E < 3mu, and nonnegative bulk with both inputs <= MAX/4 is
at most 5MAX/12. Thus an overflowing 4*mu (or, for bulk, 4*lambda) selects a
superset of all possible overflow cases. Negative-lambda bulk and the exact
constructor predicate retain their validated implementation.

Use provider-native binary exponent/scaling to obtain S=2^emax, with at most
two binade advances from the large input. Provider MAX_VALUE is infinity,
a reduction identity; it is not used as a finite bound. Put l=lambda/S and
m=mu/S. Young's normalized numerator and denominator are
N=3lm+2m^2 and D=l+m>m/3>1/6; bulk uses N=3l+2m and D=3.
All products are bounded. FMA TwoProduct residuals and grow-expansion TwoSum
preserve these polynomials as at most twelve native terms in sixteen stack
slots. The last nonzero expansion term determines the exact sign.

For upper overflow boundary b=2-eps/2, compare exact N-bD with zero; equality
rounds to infinity because the next binade has the even significand. The lower
MAX rounding boundary is 2-3eps/2; strict inequality selects the interior MAX
rounding interval. Returning MAX inside that interval is IEEE rounding, not
saturation. Below it, compute q=RN(estimate(N)/RN(D)) and retain the exact
residual N-qD; one quotient correction supplies the finite value before
restoring S. Summing a nonoverlapping expansion has a relative gamma_n bound;
with n<=12, the first quotient error is below 16u and the corrected error is
bounded by u+512u^2, where u=eps/2. This is below the existing eight-ULP limit
for binary32/64. Classification is decided by the exact sign, not this bound.

If a normalized contributing term has magnitude below eps/16, its Young
correction is below eps/15 (m>1/2), or its bulk correction is below eps/24.
The native grid near 2 remains at least eps/2 from overflow's midpoint. Such
a term cannot change classification and changes the result by at most one
ULP. This analytically bounded path also excludes normalization underflow
from the exact expansion. Otherwise all nonzero relevant products/residuals stay
normal for the provider's binary32/64 RealField implementations. There is no
widened scalar, heap allocation, numeric tolerance in domain/classification,
or change to typed state or input errors.

## Verification and scope

The original 23 added tests preserve the original bodies and add exact/adjacent bulk
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
This is historical bounded evidence, not exhaustive correct-rounding or
platform coverage; it does not establish overflow midpoint correctness.

The continuation adds the four independent regressions and two generic
fixture runners (73 total tests, 29 elastic-range tests). The 336 committed
boundary cases include each reviewed input's neighbors, both lambda signs,
true overflow, tiny-term/path-activation neighbors, and exact bulk overflow
ties in both formats. Run `python3 scripts/elastic_range_reference.py` to
check every E/K reference with integer/Fraction nearest-even rounding.
All six added tests fail against d7ed3f8 at O2/O3 with both feature modes.

An expanded external oracle runs 78,163 deduplicated input pairs, retaining
all 27,594 independent-review inputs. It adds neighborhoods of the exact
lower/upper overflow midpoints and MAX for 1,024 seeded mu samples per format,
as well as scaling-underflow and activation edges. All four O2/O3 and
default/no-default combinations match the domain and finite classification;
E/nu/K stay within one ULP, M matches exact rounding. The original acceptance
bound remains eight ULP with zero finite/nonfinite mismatches. This remains
bounded binary32/64 evidence; renewed exact-head review is required.

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
The historical d7ed3f8 ordinary matrix spans -0.359 to +1.390 ns/call across APIs/workflows.
Not every context improves. Recovered construct-and-derive cases cost
1.788-4.374 ns in the sampled contexts; the baseline's wrong output is not a
throughput target. No optimality or universal timing claim is made.

Every historical sampled region requests zero heap allocation calls/bytes. State sizes
stay 8/16 bytes and Result sizes 12/24 bytes for f32/f64. These object/request
measurements are not peak-stack, RSS/cache, no_std-performance or solver bounds.

The repaired-head matrix repeats all 2,016 samples/profile against both the
original 1681e9a baseline and reviewed d7ed3f8, with identical context counts,
input streams, validation, flags and alternating order. Ordinary deltas versus
the original baseline span -0.140 to +1.410 ns/call; versus d7ed3f8 they span
-0.253 to +0.600 ns/call. The largest median for sampled large-value repaired
workflows is 133.985 ns/call. The exact residual work adds cost in those paths;
it is required to repair the prior boundary misclassification. These samples
do not establish a universal cost bound or acceptance.

All 8,064 continuation timing regions request zero heap allocations/bytes.
State and Result sizes remain 8/16 and 12/24 bytes respectively. The private
sixteen-T expansion has bounded stack storage; peak total stack, RSS and
solver-level effects are not measured. No build/test process overlaps timing.
Independent performance/resource acceptance remains open.

## Review and release holds

Source-baseline public API comparison, strict source gates and native feature/
profile tests accompany the draft. Exact-head independent review and explicit
performance/resource acceptance remain separate. The checked public sparse
index returns 404 for `proteus-mat`: first-publication identity, authentication,
registry dependency graph and publication scope require lifecycle verification.
No merge, tag, publish, authentication/settings change or service deployment is
part of this draft. Iris #80's denied-read hold and all protected holds remain.
