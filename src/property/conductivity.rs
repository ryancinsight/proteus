use aequitas::systems::si::quantities::ThermalConductivity as Quantity;
use eunomia::RealField;

use super::{InvalidProperty, PropertyKind, validation};

validated_quantity_newtype! {
    /// Finite, non-negative isotropic thermal conductivity.
    ThermalConductivity,
    Quantity<T>,
    PropertyKind::ThermalConductivity,
    validation::non_negative,
}
