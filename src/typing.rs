#[derive(Debug, Clone, Copy)]
pub struct FullCircle(f64);

impl FullCircle {
    pub fn new(radians: f64) -> Result<Self, String> {
        if radians >= 0.0 && radians < std::f64::consts::TAU {
            Ok(Self(radians))
        } else {
            Err(format!("{radians} is not in [0, 2π)"))
        }
    }

    pub fn from_radians(radians: f64) -> Self {
        Self(radians.rem_euclid(std::f64::consts::TAU))
    }

    pub fn radians(self) -> f64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ZeroToOne(f64);

impl ZeroToOne {
    pub fn new(value: f64) -> Result<Self, String> {
        if value >= 0.0 && value < 1.0 {
            Ok(Self(value))
        } else {
            Err(format!("{value} is not in [0, 1)"))
        }
    }

    pub fn value(self) -> f64 {
        self.0
    }
}
