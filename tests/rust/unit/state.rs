use nalgebra::Vector6;
use ranger::frames::ReferenceFrame;
use ranger::state::State;

fn iss_bci() -> State {
    State {
        reference_frame: ReferenceFrame::BCI,
        vector: Vector6::new(
            -4_453.783, // x  km
            -5_038.303, // y  km
            -426.948,   // z  km
            3.831,      // vx km/s
            -2.887,     // vy km/s
            -6.019,     // vz km/s
        ),
    }
}

#[test]
fn state_fields_are_accessible() {
    let state = iss_bci();
    assert_eq!(state.reference_frame, ReferenceFrame::BCI);
    assert_eq!(state.vector[0], -4_453.783);
}

#[test]
fn state_clone_is_independent() {
    let original = iss_bci();
    let mut cloned = original.clone();
    cloned.vector[0] = 0.0;
    assert_eq!(original.vector[0], -4_453.783);
}

#[test]
fn transform_same_frame_is_identity() {
    let state = iss_bci();
    let result = state.transform(ReferenceFrame::BCI, 398_600.441_8).unwrap();
    assert_eq!(result.vector, state.vector);
}

#[test]
fn transform_unsupported_frame_returns_err() {
    let state = iss_bci();
    let result = state.transform(ReferenceFrame::RIC, 398_600.441_8);
    assert!(result.is_err());
}
