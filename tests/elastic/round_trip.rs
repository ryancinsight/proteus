//! Round-trip theorems: every conversion chain returns its input.

use super::*;

fn assert_engineering_round_trip<T: RealField>(young: f64, poisson: f64) {
    let state = moduli::<T>(young, poisson);
    let condition = round_trip_condition(poisson);
    assert_close_scaled(
        state.youngs_modulus().into_base(),
        T::from_f64(young),
        condition,
        "E round-trip",
    );
    assert_close_scaled(
        state.poissons_ratio().into_base(),
        T::from_f64(poisson),
        condition,
        "nu round-trip",
    );
}

#[test]
fn engineering_pair_round_trips_for_every_supported_real_scalar() {
    for &(young, poisson) in &[
        (200e9, 0.30),
        (68.9e9, 0.33),
        (1e6, 0.0),
        (5e9, -0.25),
        (3e10, 0.49),
    ] {
        assert_engineering_round_trip::<f32>(young, poisson);
        assert_engineering_round_trip::<f64>(young, poisson);
    }
}

fn assert_wave_speed_round_trip<T: RealField>(c_p: f64, c_s: f64, rho: f64) {
    let density = DensityQuantity::from_base(T::from_f64(rho));
    let state = IsotropicModuli::from_wave_speeds(
        Velocity::from_base(T::from_f64(c_p)),
        Velocity::from_base(T::from_f64(c_s)),
        density,
    )
    .expect("speeds satisfy the positive-definite condition");

    assert_close(
        state
            .compressional_wave_speed(density)
            .expect("positive density")
            .into_base(),
        T::from_f64(c_p),
        "c_p round-trip",
    );
    assert_close(
        state
            .shear_wave_speed(density)
            .expect("positive density")
            .into_base(),
        T::from_f64(c_s),
        "c_s round-trip",
    );
}

#[test]
fn wave_speeds_round_trip_for_every_supported_real_scalar() {
    for &(c_p, c_s, rho) in &[
        (5960.0, 3200.0, 8000.0),
        (1500.0, 80.0, 1000.0),
        (6070.0, 3100.0, 4430.0),
    ] {
        assert_wave_speed_round_trip::<f32>(c_p, c_s, rho);
        assert_wave_speed_round_trip::<f64>(c_p, c_s, rho);
    }
}
