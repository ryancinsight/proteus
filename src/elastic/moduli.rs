use aequitas::systems::si::quantities::{
    Dimensionless, MassDensity as DensityQuantity, Pressure, Velocity,
};
use eunomia::{NumericElement, RealField};

use super::{ElasticConstraint, ElasticQuantity, InvalidElasticModuli};

/// Validated isotropic linear-elastic moduli.
///
/// The canonical state is the Lame pair `(lambda, mu)`; every other isotropic
/// description is derived from it, so the type carries one representation and
/// no redundant fields that could disagree.
///
/// # Validity domain
///
/// Construction admits exactly the positive-definite isotropic states:
///
/// ```text
/// mu > 0    and    K = lambda + 2mu/3 > 0
/// ```
///
/// This is equivalent to `E > 0` with `nu` in the open interval `(-1, 1/2)`,
/// and to `c_s > 0` with `c_p^2 > (4/3) c_s^2`. Auxetic solids (`nu < 0`,
/// hence `lambda < 0`) lie inside the domain: negative `lambda` is physical,
/// and rejecting it would exclude real materials.
///
/// Because `lambda > -2mu/3` on this domain, `lambda + mu > mu/3 > 0`, so the
/// derived [`youngs_modulus`](Self::youngs_modulus) and
/// [`poissons_ratio`](Self::poissons_ratio) divisions are total and every
/// derived accessor is infallible.
///
/// # Examples
///
/// ```
/// use aequitas::systems::si::quantities::{Dimensionless, Pressure};
/// use proteus::IsotropicModuli;
///
/// let steel = IsotropicModuli::from_young_poisson(
///     Pressure::from_base(200e9_f64),
///     Dimensionless::from_base(0.3),
/// )?;
///
/// // mu = E / (2 (1 + nu))
/// let expected = 200e9 / (2.0 * 1.3);
/// assert!((*steel.shear_modulus().as_base() - expected).abs() <= expected * 1e-12);
/// # Ok::<(), proteus::InvalidElasticModuli<f64>>(())
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IsotropicModuli<T> {
    lame_lambda: Pressure<T>,
    shear_modulus: Pressure<T>,
}

impl<T: RealField> IsotropicModuli<T> {
    #[inline]
    fn two() -> T {
        <T as NumericElement>::ONE + <T as NumericElement>::ONE
    }

    #[inline]
    fn three() -> T {
        Self::two() + <T as NumericElement>::ONE
    }

    /// Construct from the Lame parameters `(lambda, mu)`.
    ///
    /// # Errors
    ///
    /// Returns [`InvalidElasticModuli`] when `lambda` is non-finite, when `mu`
    /// is not finite and positive, or when the bulk modulus
    /// `K = lambda + 2mu/3` is not positive.
    pub fn from_lame(
        lame_lambda: Pressure<T>,
        shear_modulus: Pressure<T>,
    ) -> Result<Self, InvalidElasticModuli<T>> {
        let lambda = lame_lambda.into_base();
        let mu = shear_modulus.into_base();
        let zero = <T as NumericElement>::ZERO;

        if !lambda.is_finite() {
            return Err(InvalidElasticModuli::new(
                ElasticQuantity::LameLambda,
                lambda,
                ElasticConstraint::Finite,
            ));
        }
        if !mu.is_finite() || mu <= zero {
            return Err(InvalidElasticModuli::new(
                ElasticQuantity::ShearModulus,
                mu,
                ElasticConstraint::FinitePositive,
            ));
        }

        let bulk = lambda + Self::two() * mu / Self::three();
        if bulk <= zero {
            return Err(InvalidElasticModuli::new(
                ElasticQuantity::LameLambda,
                lambda,
                ElasticConstraint::PositiveDefinite,
            ));
        }

        Ok(Self {
            lame_lambda: Pressure::from_base(lambda),
            shear_modulus: Pressure::from_base(mu),
        })
    }

    /// Construct from the engineering pair `(E, nu)`.
    ///
    /// # Theorem
    ///
    /// `mu = E / (2(1 + nu))` and `lambda = E nu / ((1 + nu)(1 - 2nu))`. For
    /// `E > 0` and `nu` in `(-1, 1/2)` both denominators are positive, so
    /// `mu > 0`, and `K = E / (3(1 - 2nu)) > 0`. Every accepted argument pair
    /// therefore lands inside the validity domain.
    ///
    /// # Errors
    ///
    /// Returns [`InvalidElasticModuli`] when `E` is not finite and positive or
    /// when `nu` lies outside `(-1, 1/2)`.
    pub fn from_young_poisson(
        youngs_modulus: Pressure<T>,
        poissons_ratio: Dimensionless<T>,
    ) -> Result<Self, InvalidElasticModuli<T>> {
        let young = youngs_modulus.into_base();
        let nu = poissons_ratio.into_base();
        let one = <T as NumericElement>::ONE;
        let zero = <T as NumericElement>::ZERO;

        if !young.is_finite() || young <= zero {
            return Err(InvalidElasticModuli::new(
                ElasticQuantity::YoungsModulus,
                young,
                ElasticConstraint::FinitePositive,
            ));
        }
        if !nu.is_finite() || nu <= -one || nu >= one / Self::two() {
            return Err(InvalidElasticModuli::new(
                ElasticQuantity::PoissonsRatio,
                nu,
                ElasticConstraint::PoissonRatioRange,
            ));
        }

        let mu = young / (Self::two() * (one + nu));
        let lambda = young * nu / ((one + nu) * (one - Self::two() * nu));

        Self::from_lame(Pressure::from_base(lambda), Pressure::from_base(mu))
    }

    /// Construct from isotropic wave speeds and density.
    ///
    /// # Theorem
    ///
    /// `mu = rho c_s^2` and `lambda = rho (c_p^2 - 2 c_s^2)` invert
    /// `c_s = sqrt(mu / rho)` and `c_p = sqrt((lambda + 2mu) / rho)`. The
    /// positive-definite condition becomes `c_s > 0` and
    /// `c_p^2 > (4/3) c_s^2`, because `K = rho (c_p^2 - (4/3) c_s^2)`.
    ///
    /// # Errors
    ///
    /// Returns [`InvalidElasticModuli`] when density or either speed is not
    /// finite and positive, or when the speeds violate the positive-definite
    /// condition.
    pub fn from_wave_speeds(
        compressional_wave_speed: Velocity<T>,
        shear_wave_speed: Velocity<T>,
        density: DensityQuantity<T>,
    ) -> Result<Self, InvalidElasticModuli<T>> {
        let rho = density.into_base();
        let c_p = compressional_wave_speed.into_base();
        let c_s = shear_wave_speed.into_base();
        let zero = <T as NumericElement>::ZERO;

        if !rho.is_finite() || rho <= zero {
            return Err(InvalidElasticModuli::new(
                ElasticQuantity::MassDensity,
                rho,
                ElasticConstraint::FinitePositive,
            ));
        }
        if !c_p.is_finite() || c_p <= zero {
            return Err(InvalidElasticModuli::new(
                ElasticQuantity::CompressionalWaveSpeed,
                c_p,
                ElasticConstraint::FinitePositive,
            ));
        }
        if !c_s.is_finite() || c_s <= zero {
            return Err(InvalidElasticModuli::new(
                ElasticQuantity::ShearWaveSpeed,
                c_s,
                ElasticConstraint::FinitePositive,
            ));
        }

        let mu = rho * c_s * c_s;
        let lambda = rho * (c_p * c_p - Self::two() * c_s * c_s);

        Self::from_lame(Pressure::from_base(lambda), Pressure::from_base(mu))
    }

    /// Derive Young's modulus `E = mu (3 lambda + 2 mu) / (lambda + mu)`.
    #[must_use]
    pub fn youngs_modulus(&self) -> Pressure<T> {
        let lambda = *self.lame_lambda.as_base();
        let mu = *self.shear_modulus.as_base();
        Pressure::from_base(mu * (Self::three() * lambda + Self::two() * mu) / (lambda + mu))
    }

    /// Derive Poisson's ratio `nu = lambda / (2 (lambda + mu))`.
    #[must_use]
    pub fn poissons_ratio(&self) -> Dimensionless<T> {
        let lambda = *self.lame_lambda.as_base();
        let mu = *self.shear_modulus.as_base();
        Dimensionless::from_base(lambda / (Self::two() * (lambda + mu)))
    }

    /// Derive the bulk modulus `K = lambda + 2 mu / 3`.
    #[must_use]
    pub fn bulk_modulus(&self) -> Pressure<T> {
        let lambda = *self.lame_lambda.as_base();
        let mu = *self.shear_modulus.as_base();
        Pressure::from_base(lambda + Self::two() * mu / Self::three())
    }

    /// Derive the P-wave (longitudinal) modulus `M = lambda + 2 mu`.
    #[must_use]
    pub fn p_wave_modulus(&self) -> Pressure<T> {
        let lambda = *self.lame_lambda.as_base();
        let mu = *self.shear_modulus.as_base();
        Pressure::from_base(lambda + Self::two() * mu)
    }

    /// Derive the compressional wave speed `c_p = sqrt((lambda + 2 mu) / rho)`.
    ///
    /// The radicand is positive on the validity domain because
    /// `lambda + 2mu = K + 4mu/3`.
    ///
    /// # Errors
    ///
    /// Returns [`InvalidElasticModuli`] when density is not finite and
    /// positive.
    pub fn compressional_wave_speed(
        &self,
        density: DensityQuantity<T>,
    ) -> Result<Velocity<T>, InvalidElasticModuli<T>> {
        let rho = Self::positive_density(density)?;
        Ok(Velocity::from_base(
            (self.p_wave_modulus().into_base() / rho).sqrt(),
        ))
    }

    /// Derive the shear wave speed `c_s = sqrt(mu / rho)`.
    ///
    /// # Errors
    ///
    /// Returns [`InvalidElasticModuli`] when density is not finite and
    /// positive.
    pub fn shear_wave_speed(
        &self,
        density: DensityQuantity<T>,
    ) -> Result<Velocity<T>, InvalidElasticModuli<T>> {
        let rho = Self::positive_density(density)?;
        Ok(Velocity::from_base(
            (*self.shear_modulus.as_base() / rho).sqrt(),
        ))
    }

    #[inline]
    fn positive_density(density: DensityQuantity<T>) -> Result<T, InvalidElasticModuli<T>> {
        let rho = density.into_base();
        if rho.is_finite() && rho > <T as NumericElement>::ZERO {
            Ok(rho)
        } else {
            Err(InvalidElasticModuli::new(
                ElasticQuantity::MassDensity,
                rho,
                ElasticConstraint::FinitePositive,
            ))
        }
    }
}

impl<T> IsotropicModuli<T> {
    /// Borrow Lame's first parameter `lambda`.
    #[must_use]
    pub const fn lame_lambda(&self) -> &Pressure<T> {
        &self.lame_lambda
    }

    /// Borrow the shear modulus `mu`, Lame's second parameter.
    #[must_use]
    pub const fn shear_modulus(&self) -> &Pressure<T> {
        &self.shear_modulus
    }
}
