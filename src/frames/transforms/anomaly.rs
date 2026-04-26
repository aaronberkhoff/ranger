use crate::typing::{FullCircle, ZeroToOne};

pub fn mean_to_eccentric(mean_anomaly: FullCircle, eccentricity: ZeroToOne) -> FullCircle {
    let mean = mean_anomaly.radians();
    let ecc = eccentricity.value();
    let mut eccentric_anomaly = mean;
    for _ in 0..50 {
        let delta = (eccentric_anomaly - ecc * eccentric_anomaly.sin() - mean)
            / (1.0 - ecc * eccentric_anomaly.cos());
        eccentric_anomaly -= delta;
        if delta.abs() < 1e-12 {
            break;
        }
    }
    FullCircle::from_radians(eccentric_anomaly)
}

pub fn eccentric_to_true(eccentric_anomaly: FullCircle, eccentricity: ZeroToOne) -> FullCircle {
    let half_e = eccentric_anomaly.radians() / 2.0;
    let ecc = eccentricity.value();
    let true_anomaly = 2.0 * ((1.0 + ecc).sqrt() * half_e.sin())
        .atan2((1.0 - ecc).sqrt() * half_e.cos());
    FullCircle::from_radians(true_anomaly)
}

pub fn mean_to_true(mean_anomaly: FullCircle, eccentricity: ZeroToOne) -> FullCircle {
    let eccentric_anomaly = mean_to_eccentric(mean_anomaly, eccentricity);
    eccentric_to_true(eccentric_anomaly, eccentricity)
}
