// AI: 2026-04-25 - Initial generation of transform known-value tests (claude-sonnet-4-6)

use nalgebra::Vector6;
use ranger::frames::transforms::bci::bci_to_coe;

const MU: f64 = 398_600.441_8;

fn iss_bci() -> Vector6<f64> {
    Vector6::new(-4_453.783, -5_038.303, -426.948, 3.831, -2.887, -6.019)
}

#[test]
fn sma_is_positive() {
    let coe = bci_to_coe(iss_bci(), MU);
    assert!(coe[0] > 0.0, "sma must be positive, got {}", coe[0]);
}

#[test]
fn eccentricity_is_in_range() {
    let coe = bci_to_coe(iss_bci(), MU);
    assert!(
        coe[1] >= 0.0 && coe[1] < 1.0,
        "eccentricity out of range: {}",
        coe[1]
    );
}

#[test]
fn inclination_is_in_range() {
    let coe = bci_to_coe(iss_bci(), MU);
    assert!(
        coe[2] >= 0.0 && coe[2] <= std::f64::consts::PI,
        "inclination out of range: {}",
        coe[2]
    );
}

#[test]
fn angles_are_in_range() {
    let coe = bci_to_coe(iss_bci(), MU);
    for index in 3..6 {
        assert!(
            coe[index] >= 0.0 && coe[index] < std::f64::consts::TAU,
            "angle coe[{index}] out of [0, 2π): {}",
            coe[index]
        );
    }
}

#[test]
fn iss_sma_is_near_leo() {
    let coe = bci_to_coe(iss_bci(), MU);
    let sma = coe[0];
    // ISS orbits at ~400 km altitude; Earth radius ~6371 km
    assert!(
        sma > 6_700.0 && sma < 6_900.0,
        "sma not in LEO range: {sma}"
    );
}
