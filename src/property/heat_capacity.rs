use aequitas::systems::si::quantities::SpecificHeatCapacity as Quantity;
use eunomia::RealField;

use super::{InvalidProperty, PropertyKind, validation};

validated_quantity_newtype! {
    /// Finite, strictly positive specific heat capacity.
    SpecificHeatCapacity,
    Quantity<T>,
    PropertyKind::SpecificHeatCapacity,
    validation::positive,
}
