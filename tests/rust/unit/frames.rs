// AI: 2026-04-25 - Initial generation of ReferenceFrame enum tests (claude-sonnet-4-6)

use ranger::frames::ReferenceFrame;

#[test]
fn variant_equals_itself() {
    assert_eq!(ReferenceFrame::BCI, ReferenceFrame::BCI);
    assert_eq!(ReferenceFrame::COE, ReferenceFrame::COE);
}

#[test]
fn variants_are_not_equal_to_each_other() {
    assert_ne!(ReferenceFrame::BCI, ReferenceFrame::COE);
    assert_ne!(ReferenceFrame::LVLH, ReferenceFrame::RIC);
}

#[test]
fn clone_produces_equal_variant() {
    let frame = ReferenceFrame::BCRF;
    assert_eq!(frame.clone(), frame);
}

#[test]
fn copy_does_not_move() {
    let frame = ReferenceFrame::ENU;
    let copy = frame;
    assert_eq!(frame, copy); // both usable — frame was copied not moved
}

#[test]
fn debug_output_is_nonempty() {
    let debug = format!("{:?}", ReferenceFrame::BCI);
    assert!(!debug.is_empty());
}
