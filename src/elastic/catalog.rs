use aequitas::systems::si::quantities::{
    Dimensionless, MassDensity as DensityQuantity, Pressure, ReciprocalTemperature,
    SpecificHeatCapacity as HeatCapacityQuantity, ThermalConductivity as ConductivityQuantity,
};
use eunomia::RealField;

use super::{InvalidElasticModuli, IsotropicModuli};
use crate::{ThermophysicalProperties, property::MassDensity};

/// Cohesive isotropic-solid state: elastic moduli plus mass density.
///
/// Density is carried alongside the moduli because the wave-speed identities
/// need it; the moduli themselves are density-free.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IsotropicSolid<T> {
    moduli: IsotropicModuli<T>,
    density: MassDensity<T>,
}

impl<T: RealField> IsotropicSolid<T> {
    /// Compose validated moduli with a validated density.
    #[must_use]
    pub const fn new(moduli: IsotropicModuli<T>, density: MassDensity<T>) -> Self {
        Self { moduli, density }
    }
}

impl<T> IsotropicSolid<T> {
    /// Borrow the elastic moduli.
    #[must_use]
    pub const fn moduli(&self) -> &IsotropicModuli<T> {
        &self.moduli
    }

    /// Borrow the mass density.
    #[must_use]
    pub const fn density(&self) -> &MassDensity<T> {
        &self.density
    }
}

/// Reference isotropic solids with published room-temperature constants.
///
/// Each variant names one specific alloy or grade, never a material family:
/// "steel" alone is not a material, and merging distinct grades under one
/// entry would silently substitute their constants. Consumers that need a
/// grade absent here construct [`IsotropicModuli`] directly rather than
/// approximating with a neighbouring entry.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum NamedIsotropicSolid {
    /// Plain carbon steel, typical structural grade.
    ///
    /// `rho = 7850 kg/m^3`, `E = 200 GPa`, `nu = 0.30`,
    /// `k = 50 W/(m K)`, `c_p = 460 J/(kg K)`, `alpha = 12e-6 /K`.
    CarbonSteel,
    /// Austenitic stainless steel 316L (ASTM F138/F139 implant grade).
    ///
    /// `rho = 8000 kg/m^3`, `E = 193 GPa`, `nu = 0.30`,
    /// `k = 16 W/(m K)`, `c_p = 500 J/(kg K)`, `alpha = 16e-6 /K`.
    StainlessSteel316L,
    /// Wrought aluminium alloy 6061-T6.
    ///
    /// `rho = 2700 kg/m^3`, `E = 68.9 GPa`, `nu = 0.33`,
    /// `k = 167 W/(m K)`, `c_p = 896 J/(kg K)`, `alpha = 23.6e-6 /K`.
    ///
    /// The modulus is the published 6061-T6 value, 10 000 ksi. The two
    /// consumer catalogs this replaces had drifted to `70e9` and `69e9`
    /// respectively; neither is the grade's value, so consolidation resolves
    /// the disagreement to the source rather than promoting either side.
    Aluminium6061,
    /// Titanium alloy Ti-6Al-4V (Grade 5).
    ///
    /// `rho = 4430 kg/m^3`, `E = 114 GPa`, `nu = 0.34`,
    /// `k = 7.4 W/(m K)`, `c_p = 560 J/(kg K)`, `alpha = 8.6e-6 /K`.
    TitaniumGrade5,
}

/// Canonical-SI constants for one catalog entry, in base units.
struct Constants {
    density: f64,
    youngs_modulus: f64,
    poissons_ratio: f64,
    thermal_conductivity: f64,
    specific_heat_capacity: f64,
    thermal_expansion: f64,
}

impl NamedIsotropicSolid {
    const fn constants(self) -> Constants {
        match self {
            Self::CarbonSteel => Constants {
                density: 7850.0,
                youngs_modulus: 200e9,
                poissons_ratio: 0.30,
                thermal_conductivity: 50.0,
                specific_heat_capacity: 460.0,
                thermal_expansion: 12e-6,
            },
            Self::StainlessSteel316L => Constants {
                density: 8000.0,
                youngs_modulus: 193e9,
                poissons_ratio: 0.30,
                thermal_conductivity: 16.0,
                specific_heat_capacity: 500.0,
                thermal_expansion: 16e-6,
            },
            Self::Aluminium6061 => Constants {
                density: 2700.0,
                youngs_modulus: 68.9e9,
                poissons_ratio: 0.33,
                thermal_conductivity: 167.0,
                specific_heat_capacity: 896.0,
                thermal_expansion: 23.6e-6,
            },
            Self::TitaniumGrade5 => Constants {
                density: 4430.0,
                youngs_modulus: 114e9,
                poissons_ratio: 0.34,
                thermal_conductivity: 7.4,
                specific_heat_capacity: 560.0,
                thermal_expansion: 8.6e-6,
            },
        }
    }

    /// Build the entry's validated isotropic-solid state.
    ///
    /// # Errors
    ///
    /// Returns [`InvalidElasticModuli`] only if the target scalar type cannot
    /// represent the published constants; every catalog entry is inside the
    /// positive-definite domain at `f32` and `f64`.
    pub fn solid<T: RealField>(self) -> Result<IsotropicSolid<T>, InvalidElasticModuli<T>> {
        let constants = self.constants();
        let moduli = IsotropicModuli::from_young_poisson(
            Pressure::from_base(T::from_f64(constants.youngs_modulus)),
            Dimensionless::from_base(T::from_f64(constants.poissons_ratio)),
        )?;
        let density = MassDensity::new(DensityQuantity::from_base(T::from_f64(constants.density)))
            .map_err(|invalid| {
                InvalidElasticModuli::new(
                    super::ElasticQuantity::MassDensity,
                    *invalid.value(),
                    super::ElasticConstraint::FinitePositive,
                )
            })?;
        Ok(IsotropicSolid::new(moduli, density))
    }

    /// Build the entry's validated thermophysical bundle.
    ///
    /// # Errors
    ///
    /// Returns the property failure if the target scalar type cannot represent
    /// the published constants.
    pub fn thermophysical<T: RealField>(
        self,
    ) -> Result<ThermophysicalProperties<T>, crate::property::InvalidProperty<T>> {
        let constants = self.constants();
        ThermophysicalProperties::try_from_quantities(
            DensityQuantity::from_base(T::from_f64(constants.density)),
            HeatCapacityQuantity::from_base(T::from_f64(constants.specific_heat_capacity)),
            ConductivityQuantity::from_base(T::from_f64(constants.thermal_conductivity)),
        )
    }

    /// Return the entry's linear thermal-expansion coefficient `alpha`.
    #[must_use]
    pub fn thermal_expansion<T: RealField>(self) -> ReciprocalTemperature<T> {
        ReciprocalTemperature::from_base(T::from_f64(self.constants().thermal_expansion))
    }

    /// Return the entry's canonical name.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::CarbonSteel => "carbon steel",
            Self::StainlessSteel316L => "stainless steel 316L",
            Self::Aluminium6061 => "aluminium 6061-T6",
            Self::TitaniumGrade5 => "Ti-6Al-4V",
        }
    }
}
