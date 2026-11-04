//! Native scaling for the thermophysical quotient.

use aequitas::systems::si::quantities::{
    MassDensity, SpecificHeatCapacity, ThermalConductivity, ThermalDiffusivity,
};
use eunomia::RealField;

/// The sealed provider's binary32/binary64 encodings have an odd exponent
/// bias. One and one-half differ in the least exponent bit, whose encoding
/// is the minimum normal. The expression also covers transparent wrappers.
#[inline]
pub(super) fn minimum_normal<T: RealField>() -> T {
    T::ONE.bitxor(T::ONE / (T::ONE + T::ONE))
}

/// Split a finite scalar exactly into a significand and a binary exponent.
/// Zero has significand zero and exponent zero; its sign is retained.
fn split<T: RealField>(value: T) -> (T, i32) {
    let exponent = value.binary_exponent().unwrap_or(0);
    (value.scale_binary(-exponent), exponent)
}

/// All positive significands lie in `[1,2)`, so the capacity product lies in
/// `[1,4)` and a positive quotient in `(1/4,2)`. Only the final scaling can
/// underflow or overflow; dimensional algebra remains the original law.
#[cold]
#[inline(never)]
pub(super) fn diffusivity<T: RealField>(
    density: MassDensity<T>,
    heat_capacity: SpecificHeatCapacity<T>,
    conductivity: ThermalConductivity<T>,
) -> ThermalDiffusivity<T> {
    let (density, density_exponent) = split(density.into_base());
    let (heat_capacity, heat_exponent) = split(heat_capacity.into_base());
    let (conductivity, conductivity_exponent) = split(conductivity.into_base());
    if conductivity == T::ZERO {
        return ThermalDiffusivity::from_base(conductivity);
    }
    let exponent = conductivity_exponent - density_exponent - heat_exponent;
    let quotient = ThermalConductivity::from_base(conductivity)
        / (MassDensity::from_base(density) * SpecificHeatCapacity::from_base(heat_capacity));
    let approximation = quotient.into_base().scale_binary(exponent);
    ThermalDiffusivity::from_base(classify_boundaries(
        conductivity,
        density,
        heat_capacity,
        exponent,
        approximation,
    ))
}

/// Normalization bounds the value error, but rounding a quotient before its
/// final scaling can cross the zero or overflow midpoint. Compare the exact
/// normalized rational value at those two boundaries, without wider scalars.
fn classify_boundaries<T: RealField>(k: T, rho: T, cp: T, exponent: i32, value: T) -> T {
    let minimum = minimum_normal::<T>();
    let minimum_exponent = minimum.binary_exponent().unwrap_or(0);
    let subnormal_exponent = minimum_exponent + T::EPSILON.binary_exponent().unwrap_or(0);
    let least = minimum * T::EPSILON;
    if value <= least {
        // k/(rho*cp) * 2^exponent > 2^(subnormal_exponent-1).
        // Scaling the numerator makes the midpoint a product comparison.
        // Any nonzero difference has a normal magnitude, so FMA preserves
        // its sign. Equality rounds to zero, whose significand is even.
        let numerator = k.scale_binary(exponent - subnormal_exponent + 1);
        return if (-rho).scalar_fmadd(cp, numerator) <= T::ZERO {
            T::ZERO
        } else if value == T::ZERO {
            least
        } else {
            value
        };
    }

    let maximum_exponent = 1 - minimum_exponent;
    let largest_power = T::ONE.scale_binary(maximum_exponent);
    if value >= largest_power {
        // The overflow midpoint is (2-eps/2)*2^emax. Divide its normalized
        // comparison by two: a - rho*cp + (eps/4)*rho*cp.
        let two = T::ONE + T::ONE;
        let offset = T::EPSILON / (two + two);
        let numerator = k.scale_binary(exponent - maximum_exponent - 1);
        let residual = (-rho).scalar_fmadd(cp, numerator);
        let product = rho * cp;
        let product_error = rho.scalar_fmadd(cp, -product);
        // Near cancellation the FMA residual is exact: its magnitude is
        // about eps*product/4 while the product's least bit is 2^(2-2p).
        // Sterbenz makes residual + offset*product exact at the decision
        // boundary; the final product-error term determines its exact sign.
        let difference = (residual + offset * product) + offset * product_error;
        if difference >= T::ZERO {
            return T::INFINITY;
        }
        if !value.is_finite() {
            return (two - T::EPSILON) * largest_power;
        }
    }
    value
}
