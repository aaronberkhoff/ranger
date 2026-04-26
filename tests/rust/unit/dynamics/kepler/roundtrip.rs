// AI: 2026-04-25 - Initial generation of Kepler propagator round-trip tests (claude-sonnet-4-6)
// AI: 2026-04-25 - Added BCI round-trip tests (claude-sonnet-4-6)

use nalgebra::Vector6;
use ranger::dynamics::kepler::Kepler;
use ranger::frames::ReferenceFrame;
use ranger::state::State;

const MU: f64 = 398_600.441_8;
const TOL: f64 = 1e-6;
const TOL_BCI_POS: f64 = 1e-4; // km
const TOL_BCI_VEL: f64 = 1e-7; // km/s

/// ISS-like COE: [sma km, ecc, inc rad, arg rad, raan rad, mean_anomaly rad]
fn iss_coe(mean_anomaly: f64) -> State {
    State {
        reference_frame: ReferenceFrame::COE,
        vector: Vector6::new(6_778.0, 0.001, 0.9006, 0.5, 0.3, mean_anomaly),
    }
}

fn iss_bci() -> State {
    State {
        reference_frame: ReferenceFrame::BCI,
        vector: Vector6::new(-4_453.783, -5_038.303, -426.948, 3.831, -2.887, -6.019),
    }
}

fn orbital_period(sma: f64, mu: f64) -> f64 {
    std::f64::consts::TAU * (sma.powi(3) / mu).sqrt()
}

fn period_from_bci(bci: &Vector6<f64>, mu: f64) -> f64 {
    let r = bci.fixed_rows::<3>(0).norm();
    let v_sq = bci.fixed_rows::<3>(3).norm_squared();
    let sma = 1.0 / (2.0 / r - v_sq / mu);
    orbital_period(sma, mu)
}

#[test]
fn one_period_returns_to_start() {
    let state = iss_coe(1.0);
    let period = orbital_period(6_778.0, MU);
    let propagator = Kepler { mu: MU };

    let propagated = propagator.propagate_analytic(&state, period).unwrap();

    // mean anomaly advances by exactly 2π — normalize back to compare
    let original_m = state.vector[5];
    let propagated_m = propagated.vector[5].rem_euclid(std::f64::consts::TAU);
    assert!(
        (propagated_m - original_m).abs() < TOL,
        "mean anomaly after one period: got {propagated_m}, expected {original_m}"
    );
}

#[test]
fn dt_zero_is_identity() {
    let state = iss_coe(1.0);
    let propagator = Kepler { mu: MU };

    let propagated = propagator.propagate_analytic(&state, 0.0).unwrap();

    for index in 0..6 {
        assert!(
            (propagated.vector[index] - state.vector[index]).abs() < TOL,
            "element[{index}] changed with dt=0"
        );
    }
}

#[test]
fn output_frame_matches_coe_input() {
    let state = iss_coe(1.0);
    let propagated = Kepler { mu: MU }.propagate_analytic(&state, 60.0).unwrap();
    assert_eq!(propagated.reference_frame, ReferenceFrame::COE);
}

// --- BCI ---

#[test]
fn one_period_returns_to_start_bci() {
    let state = iss_bci();
    let period = period_from_bci(&state.vector, MU);
    let propagated = Kepler { mu: MU }
        .propagate_analytic(&state, period)
        .unwrap();

    for index in 0..3 {
        assert!(
            (propagated.vector[index] - state.vector[index]).abs() < TOL_BCI_POS,
            "position[{index}] after one period: got {}, expected {}",
            propagated.vector[index],
            state.vector[index]
        );
    }
    for index in 3..6 {
        assert!(
            (propagated.vector[index] - state.vector[index]).abs() < TOL_BCI_VEL,
            "velocity[{index}] after one period: got {}, expected {}",
            propagated.vector[index],
            state.vector[index]
        );
    }
}

#[test]
fn dt_zero_is_identity_bci() {
    let state = iss_bci();
    let propagated = Kepler { mu: MU }.propagate_analytic(&state, 0.0).unwrap();

    for index in 0..6 {
        assert!(
            (propagated.vector[index] - state.vector[index]).abs() < TOL,
            "element[{index}] changed with dt=0"
        );
    }
}

#[test]
fn output_frame_matches_bci_input() {
    let state = iss_bci();
    let propagated = Kepler { mu: MU }.propagate_analytic(&state, 60.0).unwrap();
    assert_eq!(propagated.reference_frame, ReferenceFrame::BCI);
}
