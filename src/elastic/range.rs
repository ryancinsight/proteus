//! Native-precision evaluation of isotropic elastic identities.

use eunomia::{NumericElement, RealField};

use super::overflow_boundary;

#[inline]
fn two<T: RealField>() -> T {
    <T as NumericElement>::ONE + <T as NumericElement>::ONE
}

#[inline]
fn three<T: RealField>() -> T {
    two::<T>() + <T as NumericElement>::ONE
}

/// Return `(3lambda + 2mu) / scale` and its exact power-of-two scale.
///
/// Requires finite `lambda < 0`, finite `mu > 0`. A finite `2mu` is exact,
/// so FMA preserves the sign even at cancellation and subnormal boundaries.
/// If doubling overflows, both inputs relevant to cancellation are normal:
/// `3lambda/2 + mu` has the same sign, with no overflowing product or rounded
/// `2/3` coefficient. The scale is restored only after division.
#[inline]
pub(super) fn bulk_numerator<T: RealField>(lambda: T, mu: T) -> (T, T) {
    let twice_mu = two::<T>() * mu;
    if twice_mu.is_finite() {
        (
            lambda.scalar_fmadd(three::<T>(), twice_mu),
            <T as NumericElement>::ONE,
        )
    } else {
        (large_bulk_numerator(lambda, mu), two::<T>())
    }
}

#[inline]
pub(super) fn bulk<T: RealField>(lambda: T, mu: T) -> T {
    if lambda < <T as NumericElement>::ZERO {
        let (numerator, scale) = bulk_numerator(lambda, mu);
        (numerator / three::<T>()) * scale
    } else if !(lambda * (two::<T>() + two::<T>())).is_finite()
        || !(mu * (two::<T>() + two::<T>())).is_finite()
    {
        overflow_boundary::bulk(lambda, mu)
    } else {
        lambda + (two::<T>() * mu) / three::<T>()
    }
}

#[inline]
pub(super) fn young<T: RealField>(lambda: T, mu: T) -> T {
    // E < 3mu: only this upper range can approach IEEE overflow.
    if !(mu * (two::<T>() + two::<T>())).is_finite() {
        return overflow_boundary::young(lambda, mu);
    }
    let sum = lambda + mu;
    if lambda < <T as NumericElement>::ZERO {
        let (numerator, scale) = bulk_numerator(lambda, mu);
        // On the admitted domain, sum > mu/3 and this factor is in (0, 2).
        // Divide before multiplying: the old product was quadratic in mu.
        mu * ((numerator / sum) * scale)
    } else {
        let ratio = if sum.is_finite() {
            lambda / sum
        } else {
            // A positive overflowing sum has normal inputs at this scale;
            // halving is exact and keeps the bounded ratio in [0, 1].
            large_sum_ratio(lambda, mu)
        };
        // Preserve a small lambda contribution at the overflow edge instead
        // of rounding it away when forming the factor 2 + ratio first.
        mu.scalar_fmadd(ratio, two::<T>() * mu)
    }
}

#[inline]
pub(super) fn poisson<T: RealField>(lambda: T, mu: T) -> T {
    let sum = lambda + mu;
    let twice_sum = two::<T>() * sum;
    if twice_sum.is_finite() {
        lambda / twice_sum
    } else {
        large_poisson(lambda, mu, sum)
    }
}

#[cold]
#[inline(never)]
fn large_bulk_numerator<T: RealField>(lambda: T, mu: T) -> T {
    lambda.scalar_fmadd(three::<T>() / two::<T>(), mu)
}

#[cold]
#[inline(never)]
fn large_sum_ratio<T: RealField>(lambda: T, mu: T) -> T {
    let half_lambda = lambda / two::<T>();
    half_lambda / (half_lambda + mu / two::<T>())
}

#[cold]
#[inline(never)]
fn large_poisson<T: RealField>(lambda: T, mu: T, sum: T) -> T {
    if sum.is_finite() {
        // Halve the numerator first, avoiding a second rounding of a tiny
        // ratio. Any numerator lost by halving already gives zero here.
        (lambda / two::<T>()) / sum
    } else {
        (lambda / (two::<T>() + two::<T>())) / (lambda / two::<T>() + mu / two::<T>())
    }
}
