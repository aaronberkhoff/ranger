// AI: 2026-04-25 - Initial generation of bci_to_coe transform (claude-sonnet-4-6)

use nalgebra::{Vector3, Vector6};

/// Returns [sma, ecc, inc, arg, raan, true_anomaly] in radians/km
pub fn bci_to_coe(bci: Vector6<f64>, mu: f64) -> Vector6<f64> {
    let r_vec = bci.fixed_rows::<3>(0).into_owned();
    let v_vec = bci.fixed_rows::<3>(3).into_owned();

    let r = r_vec.norm();
    let v_sq = v_vec.norm_squared();

    let h = r_vec.cross(&v_vec);
    let h_norm = h.norm();

    let rdotv = r_vec.dot(&v_vec);
    let e_vec = (v_sq - mu / r) / mu * r_vec - rdotv / mu * v_vec;
    let ecc = e_vec.norm();

    let p = h_norm * h_norm / mu;
    let semi_major_axis = p / (1.0 - ecc * ecc);

    let inc = (h[2] / h_norm).clamp(-1.0, 1.0).acos();

    // cross([0,0,1], h) = [-h[1], h[0], 0] — simplified analytically
    let node_vec = Vector3::new(-h[1], h[0], 0.0);
    let ahat = node_vec / node_vec.norm();

    // dot([1,0,0], ahat) = ahat[0]
    let mut raan = ahat[0].clamp(-1.0, 1.0).acos();
    if ahat[1] < 0.0 {
        raan = std::f64::consts::TAU - raan;
    }

    let mut arg = (ahat.dot(&e_vec) / ecc).clamp(-1.0, 1.0).acos();
    if e_vec[2] < 0.0 {
        arg = std::f64::consts::TAU - arg;
    }

    let mut true_anomaly = (e_vec.dot(&r_vec) / (ecc * r)).clamp(-1.0, 1.0).acos();
    if rdotv < 0.0 {
        true_anomaly = std::f64::consts::TAU - true_anomaly;
    }

    Vector6::new(semi_major_axis, ecc, inc, arg, raan, true_anomaly)
}
