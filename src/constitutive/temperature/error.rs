use core::fmt;

use crate::InvalidProperty;

/// Polynomial coefficient whose validity boundary failed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CoefficientOrder {
    /// First-order coefficient with units K⁻¹.
    Linear,
    /// Second-order coefficient with units K⁻².
    Quadratic,
}

/// Role of a thermodynamic temperature rejected by a constitutive law.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TemperatureRole {
    /// Temperature defining the reference property state.
    Reference,
    /// Temperature supplied for one law evaluation.
    Evaluation,
}

/// Failure constructing a bounded temperature-validity domain.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InvalidTemperatureValidity<T> {
    minimum: T,
    maximum: T,
}

impl<T> InvalidTemperatureValidity<T> {
    pub(super) const fn new(minimum: T, maximum: T) -> Self {
        Self { minimum, maximum }
    }

    /// Borrow the rejected lower bound.
    #[must_use]
    pub const fn minimum(&self) -> &T {
        &self.minimum
    }

    /// Borrow the rejected upper bound.
    #[must_use]
    pub const fn maximum(&self) -> &T {
        &self.maximum
    }
}

impl<T: fmt::Debug> fmt::Display for InvalidTemperatureValidity<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "temperature validity bounds {:?}..={:?} must be finite, positive, and ordered",
            self.minimum, self.maximum
        )
    }
}

impl<T: fmt::Debug> core::error::Error for InvalidTemperatureValidity<T> {}

/// Non-finite temperature-response coefficient.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InvalidTemperatureCoefficient<T> {
    order: CoefficientOrder,
    value: T,
}

impl<T> InvalidTemperatureCoefficient<T> {
    pub(super) const fn new(order: CoefficientOrder, value: T) -> Self {
        Self { order, value }
    }

    /// Return the coefficient order.
    #[must_use]
    pub const fn order(&self) -> CoefficientOrder {
        self.order
    }

    /// Borrow the rejected canonical-SI scalar.
    #[must_use]
    pub const fn value(&self) -> &T {
        &self.value
    }
}

impl<T: fmt::Debug> fmt::Display for InvalidTemperatureCoefficient<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{:?} temperature coefficient {:?} must be finite",
            self.order, self.value
        )
    }
}

impl<T: fmt::Debug> core::error::Error for InvalidTemperatureCoefficient<T> {}

/// Failure evaluating temperature-dependent thermophysical properties.
#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
pub enum TemperatureLawError<T> {
    /// A thermodynamic temperature was non-finite or not strictly positive.
    InvalidTemperature {
        /// Role of the rejected temperature.
        role: TemperatureRole,
        /// Rejected canonical kelvin value.
        value: T,
    },
    /// A temperature lies outside the law's calibrated validity domain.
    OutsideValidityDomain {
        /// Role of the rejected temperature.
        role: TemperatureRole,
        /// Rejected canonical kelvin value.
        value: T,
        /// Inclusive lower calibration bound in kelvin.
        minimum: T,
        /// Inclusive upper calibration bound in kelvin.
        maximum: T,
    },
    /// An evaluated material property violated its domain.
    InvalidProperty(InvalidProperty<T>),
}

impl<T> TemperatureLawError<T> {
    pub(super) const fn invalid_temperature(role: TemperatureRole, value: T) -> Self {
        Self::InvalidTemperature { role, value }
    }
}

impl<T> From<InvalidProperty<T>> for TemperatureLawError<T> {
    fn from(error: InvalidProperty<T>) -> Self {
        Self::InvalidProperty(error)
    }
}

impl<T: fmt::Debug> fmt::Display for TemperatureLawError<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidTemperature { role, value } => {
                write!(
                    formatter,
                    "{role:?} thermodynamic temperature {value:?} K must be finite and positive"
                )
            }
            Self::OutsideValidityDomain {
                role,
                value,
                minimum,
                maximum,
            } => write!(
                formatter,
                "{role:?} thermodynamic temperature {value:?} K is outside the inclusive validity domain [{minimum:?}, {maximum:?}] K"
            ),
            Self::InvalidProperty(error) => error.fmt(formatter),
        }
    }
}

impl<T: fmt::Debug> core::error::Error for TemperatureLawError<T> {}
