// AI: 2026-04-25 - Initial generation of Kepler propagator known-value tests (claude-sonnet-4-6)
// AI: 2026-04-25 - Added BCI known-value tests (claude-sonnet-4-6)

use nalgebra::Vector6;
use ranger::dynamics::kepler::Kepler;
use ranger::frames::ReferenceFrame;
use ranger::state::State;

const MU: f64 = 398_600.4418;
const TOL: f64 = 1e-10;

fn iss_coe(mean_anomaly: f64) -> State {
    State {
        reference_frame: ReferenceFrame::COE,
        vector: Vector6::new(6_778.0, 0.001, 0.9006, 0.5, 0.3, mean_anomaly),
    }
}

fn orbital_period(sma: f64, mu: f64) -> f64 {
    std::f64::consts::TAU * (sma.powi(3) / mu).sqrt()
}

/// sma, ecc, inc, arg, raan must be unchanged — only mean_anomaly advances
#[test]
fn orbital_elements_are_constant() {
    let state = iss_coe(0.5);
    let propagated = Kepler { mu: MU }
        .propagate_analytic(&state, 300.0)
        .unwrap();

    for index in 0..5 {
        assert!(
            (propagated.vector[index] - state.vector[index]).abs() < TOL,
            "element[{index}] changed during propagation"
        );
    }
}

#[test]
fn mean_anomaly_advances_by_quarter_period() {
    let state = iss_coe(0.0);
    let quarter_period = orbital_period(6_778.0, MU) / 4.0;
    let propagated = Kepler { mu: MU }
        .propagate_analytic(&state, quarter_period)
        .unwrap();

    let expected = std::f64::consts::FRAC_PI_2; // π/2
    let actual = propagated.vector[5];
    assert!(
        (actual - expected).abs() < TOL,
        "mean anomaly after T/4: got {actual}, expected {expected}"
    );
}

#[test]
fn unsupported_frame_returns_err() {
    let state = State {
        reference_frame: ReferenceFrame::LVLH,
        vector: Vector6::zeros(),
    };
    let result = Kepler { mu: MU }.propagate_analytic(&state, 60.0);
    assert!(result.is_err());
}

// --- BCI ---

fn iss_bci() -> State {
    State {
        reference_frame: ReferenceFrame::BCI,
        vector: Vector6::new(-4_453.783, -5_038.303, -426.948, 3.831, -2.887, -6.019),
    }
}

/// Geometry (sma, ecc, inc, arg, raan) must be unchanged — convert result back to COE to verify
#[test]
fn orbital_elements_are_constant_bci() {
    let state = iss_bci();
    let initial_coe = state.transform(ReferenceFrame::COE, MU).unwrap();

    let propagated = Kepler { mu: MU }
        .propagate_analytic(&state, 300.0)
        .unwrap();
    let propagated_coe = propagated.transform(ReferenceFrame::COE, MU).unwrap();

    for index in 0..5 {
        assert!(
            (propagated_coe.vector[index] - initial_coe.vector[index]).abs() < TOL,
            "element[{index}] changed during BCI propagation"
        );
    }
}

#[test]
fn energy_is_conserved_bci() {
    let state = iss_bci();
    let propagated = Kepler { mu: MU }
        .propagate_analytic(&state, 300.0)
        .unwrap();

    let energy = |v: &Vector6<f64>| {
        let r = v.fixed_rows::<3>(0).norm();
        let v_sq = v.fixed_rows::<3>(3).norm_squared();
        v_sq / 2.0 - MU / r
    };

    let initial_energy = energy(&state.vector);
    let propagated_energy = energy(&propagated.vector);
    assert!(
        (propagated_energy - initial_energy).abs() < 1e-8,
        "energy not conserved: Δε = {}",
        propagated_energy - initial_energy
    );
}
