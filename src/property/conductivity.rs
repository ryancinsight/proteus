use aequitas::systems::si::quantities::ThermalConductivity as Quantity;
use eunomia::RealField;

use super::{InvalidProperty, PropertyKind, validation};

/// Finite, non-negative isotropic thermal conductivity.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ThermalConductivity<T>(Quantity<T>);

impl<T: RealField> ThermalConductivity<T> {
    /// Validate a dimensional thermal-conductivity quantity.
    ///
    /// # Errors
    ///
    /// Returns [`InvalidProperty`] for a negative or non-finite SI value.
    pub fn new(value: Quantity<T>) -> Result<Self, InvalidProperty<T>> {
        validation::non_negative(PropertyKind::ThermalConductivity, value.into_base())
            .map(|valid| Self(Quantity::from_base(valid)))
    }
}

impl<T> ThermalConductivity<T> {
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
