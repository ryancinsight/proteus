use eunomia::{NumericElement, RealField};

use super::{InvalidProperty, PropertyConstraint, PropertyKind};

#[inline]
pub(super) fn non_negative<T: RealField>(
    kind: PropertyKind,
    value: T,
) -> Result<T, InvalidProperty<T>> {
    if value.is_finite() && value >= <T as NumericElement>::ZERO {
        Ok(value)
    } else {
        Err(InvalidProperty::new(
            kind,
            value,
            PropertyConstraint::FiniteNonNegative,
        ))
    }
}

#[inline]
pub(super) fn positive<T: RealField>(
    kind: PropertyKind,
    value: T,
) -> Result<T, InvalidProperty<T>> {
    if value.is_finite() && value > <T as NumericElement>::ZERO {
        Ok(value)
    } else {
        Err(InvalidProperty::new(
            kind,
            value,
            PropertyConstraint::FinitePositive,
        ))
    }
}
