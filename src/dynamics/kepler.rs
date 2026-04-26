// AI: 2026-04-25 - Added propagate_analytic with Keplerian two-body propagation (claude-sonnet-4-6)

use nalgebra::Vector6;
use pyo3::prelude::*;
use crate::frames::ReferenceFrame;
use crate::state::State;

/// COE convention: [sma, ecc, inc, arg, raan, mean_anomaly]
#[pyclass]
pub struct Kepler {
    pub mu: f64,
}

#[pymethods]
impl Kepler {
    #[new]
    pub fn new(mu: f64) -> Self {
        Kepler { mu }
    }

    #[getter]
    pub fn mu(&self) -> f64 {
        self.mu
    }

    #[pyo3(name = "propagate_analytic")]
    pub fn py_propagate_analytic(&self, state: &State, dt: f64) -> PyResult<State> {
        self.propagate_analytic(state, dt)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e))
    }
}

impl Kepler {
    pub fn propagate_analytic(&self, state: &State, dt: f64) -> Result<State, String> {
        let coe_state = match state.reference_frame {
            ReferenceFrame::COE => state.clone(),
            ReferenceFrame::BCI => state.transform(ReferenceFrame::COE, self.mu)?,
            _ => return Err(format!("expected BCI or COE, got {:?}", state.reference_frame)),
        };

        let (sma, ecc, inc, arg, raan, mean_anomaly) = (
            coe_state.vector[0],
            coe_state.vector[1],
            coe_state.vector[2],
            coe_state.vector[3],
            coe_state.vector[4],
            coe_state.vector[5],
        );

        let mean_motion = (self.mu / sma.powi(3)).sqrt();
        let new_mean_anomaly = mean_anomaly + mean_motion * dt;

        let propagated = State {
            reference_frame: ReferenceFrame::COE,
            vector: Vector6::new(sma, ecc, inc, arg, raan, new_mean_anomaly),
        };

        match state.reference_frame {
            ReferenceFrame::COE => Ok(propagated),
            ReferenceFrame::BCI => propagated.transform(ReferenceFrame::BCI, self.mu),
            _ => unreachable!(),
        }
    }
}