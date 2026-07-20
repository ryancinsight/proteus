use aequitas::systems::si::quantities::MassDensity as Quantity;
use eunomia::RealField;

use super::{InvalidProperty, PropertyKind, validation};

/// Finite, non-negative mass density.
///
/// Zero represents vacuum or a calibrated voxel below the material floor.
/// Cohesive continuum-property bundles impose strict positivity.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MassDensity<T>(Quantity<T>);

impl<T: RealField> MassDensity<T> {
    /// Validate a dimensional mass-density quantity.
    ///
    /// # Errors
    ///
    /// Returns [`InvalidProperty`] for a negative or non-finite SI value.
    pub fn new(value: Quantity<T>) -> Result<Self, InvalidProperty<T>> {
        validation::non_negative(PropertyKind::MassDensity, value.into_base())
            .map(|valid| Self(Quantity::from_base(valid)))
    }
}

impl<T> MassDensity<T> {
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
