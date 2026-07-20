use core::fmt;

/// Material property whose validity boundary failed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PropertyKind {
    /// Mass density.
    MassDensity,
    /// Specific heat capacity.
    SpecificHeatCapacity,
    /// Thermal conductivity.
    ThermalConductivity,
}

/// Constraint imposed on a material property.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PropertyConstraint {
    /// The value must be finite and at least zero.
    FiniteNonNegative,
    /// The value must be finite and greater than zero.
    FinitePositive,
}

/// Typed failure at a material-property validity boundary.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InvalidProperty<T> {
    kind: PropertyKind,
    value: T,
    constraint: PropertyConstraint,
}

impl<T> InvalidProperty<T> {
    pub(crate) const fn new(kind: PropertyKind, value: T, constraint: PropertyConstraint) -> Self {
        Self {
            kind,
            value,
            constraint,
        }
    }

    /// Return the rejected property kind.
    #[must_use]
    pub const fn kind(&self) -> PropertyKind {
        self.kind
    }

    /// Borrow the rejected canonical-SI scalar.
    #[must_use]
    pub const fn value(&self) -> &T {
        &self.value
    }

    /// Return the violated constraint.
    #[must_use]
    pub const fn constraint(&self) -> PropertyConstraint {
        self.constraint
    }
}

impl<T: fmt::Debug> fmt::Display for InvalidProperty<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{:?} value {:?} violates {:?}",
            self.kind, self.value, self.constraint
        )
    }
}

impl<T: fmt::Debug> core::error::Error for InvalidProperty<T> {}
