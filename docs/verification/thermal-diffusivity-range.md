# Thermal-diffusivity range

## Contract and source

`ThermophysicalProperties::thermal_diffusivity` evaluates
`alpha = k / (rho * c_p)` for finite `rho,c_p > 0` and finite `k >= 0`.
The locked Eunomia real fields are binary32/binary64 and their transparent
`F32`/`F64` wrappers. Arithmetic remains in that native field, under round to
nearest, ties to even, with gradual underflow. Aequitas quantities retain the
compile-time `L^2/T` dimension in both evaluation paths.

At default `bf850b12e857f8d423117b322adfa73ec3e5c865`, public calls fail:

- `rho=c_p=k=2^-100` in binary32 or `2^-600` in binary64 return infinity,
  although the exact results `2^100` and `2^600` are representable.
- The corresponding positive exponents return zero instead of the finite
  reciprocal. Zero conductivity with the small properties returns NaN.
- When `rho*c_p` is exactly three quarters of the minimum subnormal and `k`
  is the minimum subnormal, the old result is 1 rather than 4/3.

The eight failures, one ordinary control and unchanged production identity
are retained in the repair evidence. The public tests instantiate both native
precisions separately. Later exact-reference tests also reproduce final-range
misclassification when the capacity product itself is normal.

The repaired law preserves signed zero conductivity and never produces NaN
from an admitted bundle. Finite results are within four representable steps
of the exact rational result rounded to the native format. Final zero versus
nonzero and finite versus infinity classifications follow the exact quotient.
This is not a claim that every finite result is correctly rounded.

## Interior and scaling

The original expression and its bits remain when its capacity product is
normal and its result lies strictly between the least subnormal and the
largest finite power of two. Zero conductivity with a normal capacity also
keeps its original signed-zero result. Values outside those conditions enter
the range path; keeping every normal-product result bit would preserve the
demonstrated final-boundary failures.

Write each positive input exactly as `m * 2^e`, `1 <= m < 2`, using the
provider's `binary_exponent` and `scale_binary`. The normalized capacity lies
in `[1,4)` and the exact normalized quotient in `(1/4,2)`. These operations
stay normal; one final binary scaling restores `e_k-e_rho-e_cp`. Zero is
handled separately without losing its sign. There is no allocation, scalar
widening or temporary material representation.

For precision `p`, let `u=2^-p`. The product/division relative error is at
most `2u/(1-u)`. Binary scaling to a normal result is exact; final subnormal
rounding adds at most half a least-subnormal unit. Comparing with a correctly
rounded reference adds at most another half unit. Four representable steps
cover these bounds for both precisions, including binade transitions.
Classification corrections below do not widen that value bound.

## Exact final classification

All following inputs `k,r,c` are normalized significands. Let `D=r*c` be
their exact product and `E` the combined exponent. The least product bit is
at least `2^(2-2p)`. `P=round(D)` and `err=fma(r,c,-P)` retain `D=P+err`
exactly. Their nonzero residuals are normal in binary32 and binary64.

### Zero midpoint

The least subnormal is `s=2^esub`; the zero midpoint is `s/2`. When the
provisional result is zero or `s`, compare `k*2^(E-esub+1)` with `D` using
one FMA. Near equality, the scaling is exact and every nonzero difference
is normal, so FMA preserves its sign. When scaling loses a tiny numerator,
that numerator is already below `D`; it cannot change the decision. Equality
rounds to even zero. A positive difference returns at least `s`.

### Overflow midpoint

Let `emax` be the exponent of the largest finite power of two and
`eps=2u`. The overflow midpoint is `(2-eps/2)*2^emax`. For a provisional
result at least `2^emax`, compare

`a - D + (eps/4)*D`, where `a=k*2^(E-emax-1)`.

The code forms `residual=fma(-r,c,a)` and
`(residual + (eps/4)*P) + (eps/4)*err`. At the deciding cancellation,
`|a-D|` is about `(eps/4)*D < 2^(1-p)`. Its least bit is at least
`2^(2-2p)`, so it fits in `p` significand bits and the FMA is exact. If
`residual` and the positive offset are within a factor of two, Sterbenz's
lemma makes their sum exact. Outside that interval the dominant term fixes
the sign; the product-error term is too small to reverse it. The last term
is also exact power-of-two scaling. Any nonzero final difference is at least
`2^(1-3p)`, still normal for both formats, so its rounded sign is exact.
An overflowing scaled numerator already establishes a positive difference.

Nonnegative difference selects infinity, including the theoretical midpoint
tie. Negative difference preserves the finite approximation or replaces a
false infinity with finite `MAX`; this replacement is justified by the exact
comparison, rather than saturation of legitimate overflow.

The public interior guard routes all possible misclassifications here: a
normal product/division cannot move a value near overflow below `2^emax`,
or move a value near `s/2` above `s`, under the error bound above.

## Verification and limits

`scripts/thermophysical-oracle.py` generates 744 deterministic fixtures using
Python integer fractions and an explicit nearest-even binary encoder. It
includes numerator neighbors around both final midpoints, normal and
exceptional capacity products, full-range inputs and exact powers of two.
The Rust integration test requires exact classification and at most four
representable steps of finite error. The original 424 cases remain present
when the normal-product extension is added.

Separate controls cover signed zeros, midpoint ties, true final overflow,
minimum subnormals, all four real scalar representations, ordinary legacy
bits and allocation-free public evaluation. The full existing elastic suite
remains unchanged. Final native, no-default, release, documentation, API,
package, performance, normal-hook and independent-review receipts accompany
delivery; the fixture corpus alone is not a whole-domain proof or release
acceptance.
