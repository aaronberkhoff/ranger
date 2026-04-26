use ranger::typing::{FullCircle, ZeroToOne};
use std::f64::consts::TAU;

// --- FullCircle ---

#[test]
fn full_circle_accepts_zero() {
    assert!(FullCircle::new(0.0).is_ok());
}

#[test]
fn full_circle_accepts_value_below_tau() {
    assert!(FullCircle::new(TAU - 0.001).is_ok());
}

#[test]
fn full_circle_rejects_tau() {
    assert!(FullCircle::new(TAU).is_err());
}

#[test]
fn full_circle_rejects_negative() {
    assert!(FullCircle::new(-0.1).is_err());
}

#[test]
fn full_circle_from_radians_wraps_tau() {
    let angle = FullCircle::from_radians(TAU);
    assert_eq!(angle.radians(), 0.0);
}

#[test]
fn full_circle_from_radians_wraps_negative() {
    let angle = FullCircle::from_radians(-0.1);
    assert!((angle.radians() - (TAU - 0.1)).abs() < 1e-12);
}

#[test]
fn full_circle_from_radians_wraps_greater_than_tau() {
    let angle = FullCircle::from_radians(TAU + 1.0);
    assert!((angle.radians() - 1.0).abs() < 1e-12);
}

#[test]
fn full_circle_radians_roundtrips() {
    let value = 1.234;
    let angle = FullCircle::new(value).unwrap();
    assert_eq!(angle.radians(), value);
}

// --- ZeroToOne ---

#[test]
fn zero_to_one_accepts_zero() {
    assert!(ZeroToOne::new(0.0).is_ok());
}

#[test]
fn zero_to_one_accepts_value_below_one() {
    assert!(ZeroToOne::new(0.999).is_ok());
}

#[test]
fn zero_to_one_rejects_one() {
    assert!(ZeroToOne::new(1.0).is_err());
}

#[test]
fn zero_to_one_rejects_negative() {
    assert!(ZeroToOne::new(-0.1).is_err());
}

#[test]
fn zero_to_one_value_roundtrips() {
    let ecc = 0.7;
    let result = ZeroToOne::new(ecc).unwrap();
    assert_eq!(result.value(), ecc);
}
