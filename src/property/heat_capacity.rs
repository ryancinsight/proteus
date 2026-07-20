use aequitas::systems::si::quantities::SpecificHeatCapacity as Quantity;
use eunomia::RealField;

use super::{InvalidProperty, PropertyKind, validation};

/// Finite, strictly positive specific heat capacity.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SpecificHeatCapacity<T>(Quantity<T>);

impl<T: RealField> SpecificHeatCapacity<T> {
    /// Validate a dimensional specific-heat-capacity quantity.
    ///
    /// # Errors
    ///
    /// Returns [`InvalidProperty`] for a non-positive or non-finite SI value.
    pub fn new(value: Quantity<T>) -> Result<Self, InvalidProperty<T>> {
        validation::positive(PropertyKind::SpecificHeatCapacity, value.into_base())
            .map(|valid| Self(Quantity::from_base(valid)))
    }
}

impl<T> SpecificHeatCapacity<T> {
    /// Borrow the Aequitas quantity without conversion or copying.
    #[must_use]
    pub const fn quantity(&self) -> &Quantity<T> {
        &self.0
    }

    /// Move out the Aequitas quantity.
    #[must_use]
    pub fn into_quantity(self) -> Quantity<T> {
        self.0
    }
}
