# Guarded residual recovery: domain and error proof

The guard applies to the locked Eunomia 0.10 real-field cohort: native binary32,
binary64 and their transparent F32/F64 wrappers. Git and normalized-registry
sources implement exactly these four RealField types; FloatElement is sealed.
The model is provider-native IEEE nearest-even arithmetic with gradual
underflow and one-rounding FMA, under the existing Rust build flags. A future
scalar/backend/cohort change must revalidate these assumptions.

All IsotropicModuli constructors reach from_lame; the fields are private.
Accepted states have finite lambda, finite mu>0, and exact 3lambda+2mu>0.
No constructor predicate, public representation, error policy or ordinary
arithmetic changes. The guard is a derived error bound for selecting recovery,
not a tolerance on the material domain.

## Normal intermediates, including auxetic cancellation

The existing outer Young branch is entered only when 4mu rounds to infinity.
For native binary32/64 the discrete input grid implies mu>=S/2, where
S=2^emax is the largest finite binary power. Exact scaling gives
m=mu/S in[1/2,2). The already-reviewed tiny-lambda branch is retained. Outside
it, l=lambda/S is exact and |l|>=eps/16.

For l>=0, N=3l+2m>=1 and D=l+m>=1/2 are normal. For l<0 the admitted domain
gives |l|<2m/3, so D>m/3>=1/6. If |l|<=m/2, N>=m/2>=1/4. Otherwise
|l| is in(1/4,4/3), where both3l and2m are integer multiples of eps/4.
The positive exact N is therefore at least eps/4. Its fused rounding remains
positive and normal. N/D>=eps/8 and m*(N/D)>=eps/16, also normal for both
precisions. The fused numerator retains cancellation information; a rounded
separate3l+2m expression would not justify this proof.

Thus the four rounded operations—fused numerator, denominator sum, division
and final product—obey the normal relative-error model. Put u=eps/2 and
gamma4=4u/(1-4u). The approximation qhat to the exact normalized Young value q
satisfies |qhat-q|<=gamma4*q. The constitutive identity gives 0<q<3m<6, so
|qhat-q|<6gamma4<25u: the last inequality holds whenever u<1/100, in
particular for p=24 or 53. It does not depend on the tested input sample.

For nonnegative bulk, the existing tiny-component branch remains. Otherwise
l,m>=eps/16, one is at least 1/2, and the normalized value q=l+2m/3 lies
in[1/3,10/3). Division by 3 and the final sum are rounded; doubling is exact.
Its relative error is bounded by gamma2, smaller than the common Young bound.
All intermediate values are normal and there is no cancellation.

## Classification and value accuracy

The normalized overflow midpoint is b=2-u. The guard thresholds 2+/-16eps
are exactly representable and equal 2+/-32u. For qhat<2-32u, q<2-7u<b;
returning qhat*S is finite, with no overflowing or underflowing scaling.
For qhat>2+32u, q>2+7u>b, so infinity is the legitimate IEEE result.
All other values, including threshold equality, retain the unchanged exact
polynomial comparisons, midpoint tie handling and quotient correction.
In particular every true value near b must enter that exact recovery region.
No clamping or shortcut is introduced inside it.

For a finite fast-path result, restoring S is an exact exponent shift.
Let h be the smallest normal spacing between qhat and RN(q). At most one
binade boundary can be crossed. If the approximation crosses downward,
h>=u*q*(1-gamma4); otherwise h>=u*q. Also |RN(q)-q|<=u*q. The representable
step distance is bounded by (gamma4/u+1)/(1-gamma4)<6 for the locked
precisions. This is inside the existing eight-ULP limit. Bulk has a smaller
bound. scripts/verify_elastic_guard.py checks the exact rational constants
and the independently rounded public-input fixture; the proof covers the
whole admitted numeric domain, not only those fixture points.

## Preservation and evidence

Ordinary Young/bulk entry branches, Poisson/P-wave arithmetic and all state
constructors remain byte-identical. Differential public-API comparisons cover
ordinary outputs against the prior reviewed implementation, while the exact
oracle checks all changed large-value outputs against the unchanged eight-ULP
and zero-classification-mismatch policy. The four repaired boundary regressions,
336 existing overflow cases and all earlier tests remain intact.

An additional 305 exact-rational public cases span both sides of the guard,
both lambda signs and neighboring native inputs. Transparent-wrapper tests
cover these and the existing overflow fixture. Source/actual normalized-package
feature/profile gates, current timing including both boundary families and
mixed workloads, and generated native stack-frame observations accompany the
new exact-head packet. Total peak stack/RSS and owner performance/resource
acceptance remain open. Prior-head review does not approve this integration.

## Refreshed implementation measurements

One ARM64 Mac/Rust 1.97, native O2/O3 and unchanged overflow checks. The paired
matrix uses full typed public outputs, seven alternating samples, direct and
opaque callers plus 1024-input streaming, and excludes setup/validation. The
large benchmark states are valid and away from overflow; both versions give
correct outputs. Both actual repaired boundary families are timed separately.
Exact 1024-input mixed populations are510ordinary/490auxetic/12large/6Young-boundary/
6bulk-boundary and400ordinary/400auxetic/104large/60Young-boundary/60bulk-boundary.
All three mixed caller contexts use the same 1024 inputs; the labels are nominal,
not unqualified workload percentages. Full figures accompany the review receipt.

All 8,064 timing regions request zero heap allocation calls/bytes. State sizes
remain 8/16 bytes and Result sizes 12/24 bytes. Generated ARM64 CFI local frames
are 32 bytes for the guarded large wrappers, versus 144/208 previously. Exact
recovery retains 144/208-byte local frames and is tail-called after the wrapper
epilogue. Calls to libm scaling and surrounding callers are not bounded by
those local figures; total peak process stack/RSS remains unmeasured. No
universal timing or owner resource acceptance is inferred.
