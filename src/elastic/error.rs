use core::fmt;

/// Elastic quantity whose validity boundary failed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ElasticQuantity {
    /// Young's modulus `E`.
    YoungsModulus,
    /// Poisson's ratio `nu`.
    PoissonsRatio,
    /// Lame's first parameter `lambda`.
    LameLambda,
    /// Shear modulus `mu`, Lame's second parameter.
    ShearModulus,
    /// Compressional (P) wave speed `c_p`.
    CompressionalWaveSpeed,
    /// Shear (S) wave speed `c_s`.
    ShearWaveSpeed,
    /// Mass density `rho`.
    MassDensity,
}

/// Constraint imposed on an elastic quantity or on the moduli pair.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ElasticConstraint {
    /// The value must be finite and greater than zero.
    FinitePositive,
    /// The value must be finite.
    Finite,
    /// Poisson's ratio must lie in the open interval `(-1, 1/2)`.
    PoissonRatioRange,
    /// The moduli must be positive definite: `mu > 0` and `K = lambda + 2mu/3 > 0`.
    ///
    /// Expressed in wave speeds this is `c_s > 0` and `c_p^2 > (4/3) c_s^2`.
    PositiveDefinite,
}

/// Typed failure at an isotropic-elastic validity boundary.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InvalidElasticModuli<T> {
    quantity: ElasticQuantity,
    value: T,
    constraint: ElasticConstraint,
}

impl<T> InvalidElasticModuli<T> {
    pub(crate) const fn new(
        quantity: ElasticQuantity,
        value: T,
        constraint: ElasticConstraint,
    ) -> Self {
        Self {
            quantity,
            value,
            constraint,
        }
    }

    /// Return the rejected elastic quantity.
    #[must_use]
    pub const fn quantity(&self) -> ElasticQuantity {
        self.quantity
    }

    /// Borrow the rejected canonical-SI scalar.
    #[must_use]
    pub const fn value(&self) -> &T {
        &self.value
    }

    /// Return the violated constraint.
    #[must_use]
    pub const fn constraint(&self) -> ElasticConstraint {
        self.constraint
    }
}

impl<T: fmt::Debug> fmt::Display for InvalidElasticModuli<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{:?} value {:?} violates {:?}",
            self.quantity, self.value, self.constraint
        )
    }
}

impl<T: fmt::Debug> core::error::Error for InvalidElasticModuli<T> {}
