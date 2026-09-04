//! The named-material catalog: published constants against the values the
//! conversions produce.

use super::*;

#[test]
#[expect(
    clippy::float_cmp,
    reason = "catalog constants round-trip through from_base/into_base with no arithmetic, so exact equality is the correct assertion"
)]
fn catalog_entries_preserve_their_published_constants() {
    let steel = NamedIsotropicSolid::CarbonSteel
        .solid::<f64>()
        .expect("catalog entry is valid");
    assert_close(
        steel.moduli().youngs_modulus().into_base(),
        200e9,
        "carbon steel E",
    );
    assert_close(
        steel.moduli().poissons_ratio().into_base(),
        0.30,
        "carbon steel nu",
    );
    assert_eq!(steel.density().quantity().into_base(), 7850.0);

    let aluminium = NamedIsotropicSolid::Aluminium6061
        .solid::<f64>()
        .expect("catalog entry is valid");
    assert_close(
        aluminium.moduli().youngs_modulus().into_base(),
        68.9e9,
        "6061 E",
    );
    assert_close(
        aluminium.moduli().poissons_ratio().into_base(),
        0.33,
        "6061 nu",
    );
    assert_eq!(aluminium.density().quantity().into_base(), 2700.0);
}

#[test]
#[expect(
    clippy::float_cmp,
    reason = "catalog constants round-trip through from_base/into_base with no arithmetic, so exact equality is the correct assertion"
)]
fn distinct_steel_grades_are_not_conflated() {
    // The two consumer catalogs both said "steel" while meaning different
    // alloys. Merging them would silently substitute constants.
    let carbon = NamedIsotropicSolid::CarbonSteel
        .solid::<f64>()
        .expect("valid");
    let austenitic = NamedIsotropicSolid::StainlessSteel316L
        .solid::<f64>()
        .expect("valid");
    assert_ne!(
        carbon.density().quantity().into_base(),
        austenitic.density().quantity().into_base()
    );
    assert_ne!(
        carbon.moduli().youngs_modulus().into_base(),
        austenitic.moduli().youngs_modulus().into_base()
    );
}

#[test]
fn every_catalog_entry_is_constructible_at_every_supported_scalar() {
    for entry in [
        NamedIsotropicSolid::CarbonSteel,
        NamedIsotropicSolid::StainlessSteel316L,
        NamedIsotropicSolid::Aluminium6061,
        NamedIsotropicSolid::TitaniumGrade5,
    ] {
        let wide = entry.solid::<f64>().expect("f64 entry");
        let narrow = entry.solid::<f32>().expect("f32 entry");
        assert!(wide.moduli().bulk_modulus().into_base() > 0.0);
        assert!(narrow.moduli().bulk_modulus().into_base() > 0.0);
        assert!(entry.thermophysical::<f64>().is_ok());
        assert!(entry.thermal_expansion::<f64>().into_base() > 0.0);
        assert!(!entry.name().is_empty());
    }
}

#[test]
#[expect(
    clippy::float_cmp,
    reason = "catalog constants round-trip through from_base/into_base with no arithmetic, so exact equality is the correct assertion"
)]
fn catalog_density_agrees_between_the_elastic_and_thermophysical_views() {
    for entry in [
        NamedIsotropicSolid::CarbonSteel,
        NamedIsotropicSolid::StainlessSteel316L,
        NamedIsotropicSolid::Aluminium6061,
        NamedIsotropicSolid::TitaniumGrade5,
    ] {
        let solid = entry.solid::<f64>().expect("valid");
        let thermophysical = entry.thermophysical::<f64>().expect("valid");
        assert_eq!(
            solid.density().quantity().into_base(),
            thermophysical.density().quantity().into_base(),
        );
    }
}
