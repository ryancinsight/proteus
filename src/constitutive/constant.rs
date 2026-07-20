use core::convert::Infallible;

use eunomia::RealField;

use crate::{ConstitutiveLaw, ThermophysicalProperties};

/// Zero-sized state for a condition-independent constitutive law.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct NoState;

/// Condition-independent thermophysical constitutive law.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ConstantLaw<T> {
    properties: ThermophysicalProperties<T>,
}

impl<T> ConstantLaw<T> {
    /// Construct from validated properties.
    #[must_use]
    pub const fn new(properties: ThermophysicalProperties<T>) -> Self {
        Self { properties }
    }

    /// Borrow the constant properties.
    #[must_use]
    pub const fn value(&self) -> &ThermophysicalProperties<T> {
        &self.properties
    }
}

impl<T: RealField> ConstitutiveLaw<T> for ConstantLaw<T> {
    type State<'a>
        = NoState
    where
        Self: 'a,
        T: 'a;
    type Error = Infallible;

    #[inline]
    fn properties<'a>(
        &'a self,
        NoState: Self::State<'a>,
    ) -> Result<ThermophysicalProperties<T>, Self::Error>
    where
        T: 'a,
    {
        Ok(self.properties)
    }
}
