use aequitas::systems::si::quantities::{
    Dimensionless, ReciprocalTemperature, ReciprocalTemperatureSquared, ThermodynamicTemperature,
};
use eunomia::{NumericElement, RealField};

use super::{CoefficientOrder, InvalidTemperatureCoefficient};

/// Static response mapping a temperature difference to a property multiplier.
#[diagnostic::on_unimplemented(
    message = "this type does not implement a Proteus temperature response",
    note = "use ConstantResponse, LinearResponse<T>, or QuadraticResponse<T>"
)]
pub trait TemperatureResponse<T: RealField> {
    /// Evaluate the dimensionless multiplier at `delta_temperature`.
    fn factor(&self, delta_temperature: ThermodynamicTemperature<T>) -> Dimensionless<T>;
}

/// Zero-sized temperature-invariant property response.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ConstantResponse;

impl<T: RealField> TemperatureResponse<T> for ConstantResponse {
    #[inline]
    fn factor(&self, _delta_temperature: ThermodynamicTemperature<T>) -> Dimensionless<T> {
        Dimensionless::from_base(<T as NumericElement>::ONE)
    }
}

/// Relative linear response `f(ΔT) = 1 + β₁ΔT`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LinearResponse<T> {
    coefficient: ReciprocalTemperature<T>,
}

impl<T: RealField> LinearResponse<T> {
    /// Construct from a finite first-order coefficient.
    ///
    /// # Errors
    ///
    /// Returns [`InvalidTemperatureCoefficient`] when the canonical K⁻¹ value
    /// is NaN or infinite.
    pub fn new(
        coefficient: ReciprocalTemperature<T>,
    ) -> Result<Self, InvalidTemperatureCoefficient<T>> {
        let value = *coefficient.as_base();
        if value.is_finite() {
            Ok(Self { coefficient })
        } else {
            Err(InvalidTemperatureCoefficient::new(
                CoefficientOrder::Linear,
                value,
            ))
        }
    }
}

impl<T> LinearResponse<T> {
    /// Borrow the first-order coefficient.
    #[must_use]
    pub const fn coefficient(&self) -> &ReciprocalTemperature<T> {
        &self.coefficient
    }
}

impl<T: RealField> TemperatureResponse<T> for LinearResponse<T> {
    #[inline]
    fn factor(&self, delta_temperature: ThermodynamicTemperature<T>) -> Dimensionless<T> {
        Dimensionless::from_base(
            self.coefficient
                .as_base()
                .scalar_fmadd(*delta_temperature.as_base(), <T as NumericElement>::ONE),
        )
    }
}

/// Relative quadratic response `f(ΔT) = 1 + β₁ΔT + β₂ΔT²`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct QuadraticResponse<T> {
    linear: ReciprocalTemperature<T>,
    quadratic: ReciprocalTemperatureSquared<T>,
}

impl<T: RealField> QuadraticResponse<T> {
    /// Construct from finite first- and second-order coefficients.
    ///
    /// # Errors
    ///
    /// Returns [`InvalidTemperatureCoefficient`] naming the first non-finite
    /// canonical coefficient.
    pub fn new(
        linear: ReciprocalTemperature<T>,
        quadratic: ReciprocalTemperatureSquared<T>,
    ) -> Result<Self, InvalidTemperatureCoefficient<T>> {
        let linear_value = *linear.as_base();
        if !linear_value.is_finite() {
            return Err(InvalidTemperatureCoefficient::new(
                CoefficientOrder::Linear,
                linear_value,
            ));
        }
        let quadratic_value = *quadratic.as_base();
        if !quadratic_value.is_finite() {
            return Err(InvalidTemperatureCoefficient::new(
                CoefficientOrder::Quadratic,
                quadratic_value,
            ));
        }
        Ok(Self { linear, quadratic })
    }
}

impl<T> QuadraticResponse<T> {
    /// Borrow the first-order coefficient.
    #[must_use]
    pub const fn linear_coefficient(&self) -> &ReciprocalTemperature<T> {
        &self.linear
    }

    /// Borrow the second-order coefficient.
    #[must_use]
    pub const fn quadratic_coefficient(&self) -> &ReciprocalTemperatureSquared<T> {
        &self.quadratic
    }
}

impl<T: RealField> TemperatureResponse<T> for QuadraticResponse<T> {
    #[inline]
    fn factor(&self, delta_temperature: ThermodynamicTemperature<T>) -> Dimensionless<T> {
        let delta = *delta_temperature.as_base();
        let linear_factor = self
            .linear
            .as_base()
            .scalar_fmadd(delta, <T as NumericElement>::ONE);
        let factor = (*self.quadratic.as_base() * delta).scalar_fmadd(delta, linear_factor);
        Dimensionless::from_base(factor)
    }
}
