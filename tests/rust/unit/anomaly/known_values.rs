// AI: 2026-04-25 - Initial generation of anomaly known-value tests (claude-sonnet-4-6)

use ranger::frames::transforms::anomaly::{eccentric_to_true, mean_to_eccentric, mean_to_true};
use ranger::typing::{FullCircle, ZeroToOne};

const TOL: f64 = 1e-12;

fn angle(radians: f64) -> FullCircle {
    FullCircle::from_radians(radians)
}

fn ecc(value: f64) -> ZeroToOne {
    ZeroToOne::new(value).unwrap()
}

/// At periapsis M=0 → E=0 → ν=0 for any eccentricity
#[test]
fn periapsis_mean_to_eccentric_is_zero() {
    let eccentric_anomaly = mean_to_eccentric(angle(0.0), ecc(0.5));
    assert!(eccentric_anomaly.radians().abs() < TOL);
}

#[test]
fn periapsis_eccentric_to_true_is_zero() {
    let true_anomaly = eccentric_to_true(angle(0.0), ecc(0.5));
    assert!(true_anomaly.radians().abs() < TOL);
}

#[test]
fn periapsis_mean_to_true_is_zero() {
    let true_anomaly = mean_to_true(angle(0.0), ecc(0.5));
    assert!(true_anomaly.radians().abs() < TOL);
}

/// At apoapsis M=π → E=π → ν=π for any eccentricity
#[test]
fn apoapsis_mean_to_eccentric_is_pi() {
    let eccentric_anomaly = mean_to_eccentric(angle(std::f64::consts::PI), ecc(0.5));
    assert!(
        (eccentric_anomaly.radians() - std::f64::consts::PI).abs() < TOL,
        "got {}",
        eccentric_anomaly.radians()
    );
}

#[test]
fn apoapsis_eccentric_to_true_is_pi() {
    let true_anomaly = eccentric_to_true(angle(std::f64::consts::PI), ecc(0.5));
    assert!(
        (true_anomaly.radians() - std::f64::consts::PI).abs() < TOL,
        "got {}",
        true_anomaly.radians()
    );
}

/// For a circular orbit (e=0): M = E = ν
#[test]
fn circular_orbit_mean_equals_eccentric() {
    let mean_anomaly = angle(1.234);
    let eccentric_anomaly = mean_to_eccentric(mean_anomaly, ecc(0.0));
    assert!(
        (eccentric_anomaly.radians() - mean_anomaly.radians()).abs() < TOL,
        "got {}",
        eccentric_anomaly.radians()
    );
}

#[test]
fn circular_orbit_eccentric_equals_true() {
    let eccentric_anomaly = angle(1.234);
    let true_anomaly = eccentric_to_true(eccentric_anomaly, ecc(0.0));
    assert!(
        (true_anomaly.radians() - eccentric_anomaly.radians()).abs() < TOL,
        "got {}",
        true_anomaly.radians()
    );
}
