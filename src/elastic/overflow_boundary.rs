//! Overflow-midpoint evaluation with native-precision residuals.
//!
//! This path receives finite admitted Lamé pairs with at least one large
//! component. Scaling by S = 2^emax makes every product bounded. `TwoSum` and
//! FMA retain the exact polynomial numerator, including the terms that decide
//! which side of MAX + ulp(MAX)/2 the real identity lies on. No wider scalar,
//! heap allocation, rounded 2/3 coefficient, or saturation is used.

use core::cmp::Ordering;
use eunomia::RealField;

#[inline]
fn two<T: RealField>() -> T {
    T::ONE + T::ONE
}

#[inline]
fn three<T: RealField>() -> T {
    two::<T>() + T::ONE
}

/// Start in the largest input's binade and advance to the largest finite
/// power of two. The caller's overflowing 4*input guard puts it at most two
/// binades away. Provider `MAX_VALUE` is a reduction identity (infinity), so it
/// must not be used as the largest finite number.
#[inline]
fn scale<T: RealField>(lambda: T, mu: T) -> T {
    let exponent = lambda
        .abs()
        .max_scalar(mu)
        .binary_exponent()
        .expect("finite nonzero admitted large input");
    let mut scale = T::ONE.scale_binary(exponent);
    while (scale * two::<T>()).is_finite() {
        scale *= two::<T>();
    }
    scale
}

#[inline]
fn two_sum<T: RealField>(a: T, b: T) -> (T, T) {
    let sum = a + b;
    let virtual_b = sum - a;
    (sum, (a - (sum - virtual_b)) + (b - virtual_b))
}

/// Nonoverlapping expansion in increasing magnitude. Grow-expansion `TwoSum`
/// preserves the exact sum; its last nonzero component determines its sign.
/// At most 12 scalar terms enter any expression below, so 16 slots suffice.
struct Terms<T: RealField> {
    values: [T; 16],
    len: usize,
}

impl<T: RealField> Terms<T> {
    fn new() -> Self {
        Self {
            values: [T::ZERO; 16],
            len: 0,
        }
    }

    fn add(&mut self, mut value: T) {
        let mut kept = 0;
        for i in 0..self.len {
            let (sum, error) = two_sum(value, self.values[i]);
            if error != T::ZERO {
                self.values[kept] = error;
                kept += 1;
            }
            value = sum;
        }
        if value != T::ZERO {
            self.values[kept] = value;
            kept += 1;
        }
        self.len = kept;
    }

    fn product(&mut self, a: T, b: T) {
        let product = a * b;
        self.add(a.scalar_fmadd(b, -product));
        self.add(product);
    }

    fn estimate(&self) -> T {
        self.values[..self.len]
            .iter()
            .copied()
            .fold(T::ZERO, |a, b| a + b)
    }

    fn sign(&self) -> Ordering {
        if self.len == 0 {
            Ordering::Equal
        } else if self.values[self.len - 1] > T::ZERO {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    }
}

/// Compare an exact numerator with (2 - steps*eps/2) times its denominator.
/// steps=1 is the overflow midpoint; steps=3 is MAX's lower midpoint.
/// Equality at the upper midpoint rounds to infinity (even significand).
fn young_midpoint<T: RealField>(l: T, m: T, steps: T) -> Ordering {
    let mut terms = young_numerator(l, m);
    terms.add(-two::<T>() * l);
    terms.add(-two::<T>() * m);
    let offset = steps * (T::EPSILON / two::<T>());
    terms.product(offset, l);
    terms.product(offset, m);
    terms.sign()
}

fn young_numerator<T: RealField>(l: T, m: T) -> Terms<T> {
    let mut terms = Terms::new();
    let product = l * m;
    let residual = l.scalar_fmadd(m, -product);
    terms.product(three::<T>(), product);
    terms.product(three::<T>(), residual);
    let square = m * m;
    terms.add(two::<T>() * m.scalar_fmadd(m, -square));
    terms.add(two::<T>() * square);
    terms
}

#[cold]
#[inline(never)]
pub(super) fn young<T: RealField>(lambda: T, mu: T) -> T {
    let s = scale(lambda, mu);
    let l = lambda / s;
    let m = mu / s;
    let two_mu = two::<T>() * mu;
    // When |l| < eps/16, |E/S - 2m| < eps/15. Here m > 1/2,
    // so the error is below 1/4 ulp. The native grid for 2m stays at
    // least eps/2 from the overflow midpoint. Thus even normalization
    // underflow cannot change finite/infinite classification.
    let sixteen = (two::<T>() + two::<T>()) * (two::<T>() + two::<T>());
    if l.abs() < T::EPSILON / sixteen {
        return two_mu;
    }
    // Outside the tiny-term case, all products and residuals are normal
    // for binary32/binary64. Admissibility gives l+m > m/3 > 1/6.
    if young_midpoint(l, m, T::ONE) != Ordering::Less {
        return T::INFINITY;
    }
    if young_midpoint(l, m, three::<T>()) == Ordering::Greater {
        return (two::<T>() - T::EPSILON) * s;
    }
    let mut numerator = young_numerator(l, m);
    let denominator = l + m;
    let quotient = numerator.estimate() / denominator;
    // Evaluate N - quotient*(l+m) exactly, including the sum residual.
    numerator.product(-quotient, l);
    numerator.product(-quotient, m);
    let correction = numerator.estimate() / denominator;
    (quotient + correction) * s
}

fn bulk_numerator<T: RealField>(l: T, m: T) -> Terms<T> {
    let mut terms = Terms::new();
    terms.product(three::<T>(), l);
    terms.add(two::<T>() * m);
    terms
}

fn bulk_midpoint<T: RealField>(l: T, m: T, steps: T) -> Ordering {
    let mut terms = bulk_numerator(l, m);
    terms.add(-(three::<T>() * two::<T>()));
    terms.add(three::<T>() * (steps * (T::EPSILON / two::<T>())));
    terms.sign()
}

#[cold]
#[inline(never)]
pub(super) fn bulk<T: RealField>(lambda: T, mu: T) -> T {
    let s = scale(lambda, mu);
    let l = lambda / s;
    let m = mu / s;
    let sixteen = (two::<T>() + two::<T>()) * (two::<T>() + two::<T>());
    // A term below eps/16 cannot bridge the half-ulp distance from
    // lambda's native grid to overflow. If lambda is tiny, K < 2MAX/3.
    // These cases also exclude normalization underflow from exact expansions.
    if l < T::EPSILON / sixteen || m < T::EPSILON / sixteen {
        return lambda + (mu / three::<T>()) * two::<T>();
    }
    if bulk_midpoint(l, m, T::ONE) != Ordering::Less {
        return T::INFINITY;
    }
    if bulk_midpoint(l, m, three::<T>()) == Ordering::Greater {
        return (two::<T>() - T::EPSILON) * s;
    }
    let mut numerator = bulk_numerator(l, m);
    let quotient = numerator.estimate() / three::<T>();
    numerator.product(-quotient, three::<T>());
    (quotient + numerator.estimate() / three::<T>()) * s
}
