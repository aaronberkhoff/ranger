// AI: 2026-04-25 - Initial generation of transform round-trip tests (claude-sonnet-4-6)

use nalgebra::Vector6;
use ranger::frames::transforms::{bci::bci_to_coe, coe::coe_to_bci};

const MU: f64 = 398_600.441_8;
const TOL_POS: f64 = 1e-10; // km
const TOL_VEL: f64 = 1e-10; // km/s
const TOL_COE: f64 = 1e-10;

fn iss_bci() -> Vector6<f64> {
    Vector6::new(-4_453.783, -5_038.303, -426.948, 3.831, -2.887, -6.019)
}

#[test]
fn bci_to_coe_to_bci_roundtrip() {
    let original = iss_bci();
    let coe = bci_to_coe(original, MU);
    let recovered = coe_to_bci(coe, MU);

    for index in 0..3 {
        assert!(
            (recovered[index] - original[index]).abs() < TOL_POS,
            "position[{index}]: got {}, expected {}",
            recovered[index],
            original[index]
        );
    }
    for index in 3..6 {
        assert!(
            (recovered[index] - original[index]).abs() < TOL_VEL,
            "velocity[{index}]: got {}, expected {}",
            recovered[index],
            original[index]
        );
    }
}

#[test]
fn coe_to_bci_to_coe_roundtrip() {
    let original = bci_to_coe(iss_bci(), MU);
    let bci = coe_to_bci(original, MU);
    let recovered = bci_to_coe(bci, MU);

    for index in 0..6 {
        assert!(
            (recovered[index] - original[index]).abs() < TOL_COE,
            "coe[{index}]: got {}, expected {}",
            recovered[index],
            original[index]
        );
    }
}
