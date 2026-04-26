// AI: 2026-04-25 - Initial generation of anomaly round-trip tests (claude-sonnet-4-6)

use ranger::frames::transforms::anomaly::{eccentric_to_true, mean_to_eccentric, mean_to_true};
use ranger::typing::{FullCircle, ZeroToOne};

const TOL: f64 = 1e-12;

fn angle(radians: f64) -> FullCircle {
    FullCircle::from_radians(radians)
}

fn ecc(value: f64) -> ZeroToOne {
    ZeroToOne::new(value).unwrap()
}

/// Verify Kepler's equation is satisfied: E - e*sin(E) = M
#[test]
fn mean_to_eccentric_satisfies_keplers_equation() {
    let mean_anomaly = angle(1.2);
    let eccentricity = ecc(0.3);
    let eccentric_anomaly = mean_to_eccentric(mean_anomaly, eccentricity);

    let residual = eccentric_anomaly.radians()
        - eccentricity.value() * eccentric_anomaly.radians().sin()
        - mean_anomaly.radians();

    assert!(
        residual.abs() < TOL,
        "Kepler residual too large: {residual}"
    );
}

#[test]
fn mean_to_eccentric_satisfies_keplers_equation_high_eccentricity() {
    let mean_anomaly = angle(2.5);
    let eccentricity = ecc(0.85);
    let eccentric_anomaly = mean_to_eccentric(mean_anomaly, eccentricity);

    let residual = eccentric_anomaly.radians()
        - eccentricity.value() * eccentric_anomaly.radians().sin()
        - mean_anomaly.radians();

    assert!(
        residual.abs() < TOL,
        "Kepler residual too large: {residual}"
    );
}

/// Verify eccentric_to_true is invertible: tan(E/2) = sqrt((1-e)/(1+e)) * tan(ν/2)
#[test]
fn eccentric_to_true_is_consistent_with_inverse() {
    let eccentric_anomaly = angle(1.0);
    let eccentricity = ecc(0.4);
    let true_anomaly = eccentric_to_true(eccentric_anomaly, eccentricity);

    let half_nu = true_anomaly.radians() / 2.0;
    let half_e = eccentric_anomaly.radians() / 2.0;
    let recovered_tan =
        ((1.0 - eccentricity.value()) / (1.0 + eccentricity.value())).sqrt() * half_nu.tan();

    assert!(
        (recovered_tan - half_e.tan()).abs() < TOL,
        "inverse check failed: {recovered_tan} vs {}",
        half_e.tan()
    );
}

#[test]
fn mean_to_true_output_is_in_full_circle_range() {
    for mean_degrees in [0, 45, 90, 135, 180, 225, 270, 315] {
        let mean_anomaly = angle(mean_degrees as f64 * std::f64::consts::PI / 180.0);
        let eccentricity = ecc(0.3);
        let true_anomaly = mean_to_true(mean_anomaly, eccentricity);
        assert!(
            true_anomaly.radians() >= 0.0 && true_anomaly.radians() < std::f64::consts::TAU,
            "true anomaly out of [0, 2π) at M={mean_degrees}°: {}",
            true_anomaly.radians()
        );
    }
}
