use aequitas::systems::si::quantities::MassDensity as Quantity;
use eunomia::RealField;

use super::{InvalidProperty, PropertyKind, validation};

validated_quantity_newtype! {
    /// Finite, non-negative mass density.
    ///
    /// Zero represents vacuum or a calibrated voxel below the material floor.
    /// Cohesive continuum-property bundles impose strict positivity.
    MassDensity,
    Quantity<T>,
    PropertyKind::MassDensity,
    validation::non_negative,
}
