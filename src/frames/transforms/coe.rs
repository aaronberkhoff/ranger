// AI: 2026-04-25 - Initial generation of coe_to_bci transform (claude-sonnet-4-6)

use nalgebra::Vector6;

/// Converts classical orbital elements [sma, ecc, inc, arg, raan, true_anomaly]
/// to a cartesian state vector [x, y, z, vx, vy, vz].
pub fn coe_to_bci(coe: Vector6<f64>, mu: f64) -> Vector6<f64> {
    let (sma, ecc, inc, arg, raan, nu) = (coe[0], coe[1], coe[2], coe[3], coe[4], coe[5]);

    let (sin_nu, cos_nu) = nu.sin_cos();
    let (sin_inc, cos_inc) = inc.sin_cos();
    let (sin_arg, cos_arg) = arg.sin_cos();
    let (sin_raan, cos_raan) = raan.sin_cos();

    let p = sma * (1.0 - ecc * ecc);
    let r = p / (1.0 + ecc * cos_nu);
    let sqrt_mu_p = (mu / p).sqrt();

    // perifocal state (z = 0 by definition)
    let rx = r * cos_nu;
    let ry = r * sin_nu;
    let vx = -sqrt_mu_p * sin_nu;
    let vy = sqrt_mu_p * (ecc + cos_nu);

    // only the first two columns of the rotation matrix are needed since z = 0
    let t00 = cos_raan * cos_arg - sin_raan * sin_arg * cos_inc;
    let t10 = sin_raan * cos_arg + cos_raan * sin_arg * cos_inc;
    let t20 = sin_arg * sin_inc;

    let t01 = -cos_raan * sin_arg - sin_raan * cos_arg * cos_inc;
    let t11 = -sin_raan * sin_arg + cos_raan * cos_arg * cos_inc;
    let t21 = cos_arg * sin_inc;

    Vector6::new(
        t00 * rx + t01 * ry,
        t10 * rx + t11 * ry,
        t20 * rx + t21 * ry,
        t00 * vx + t01 * vy,
        t10 * vx + t11 * vy,
        t20 * vx + t21 * vy,
    )
}
