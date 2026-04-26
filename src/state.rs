use nalgebra::Vector6;
use pyo3::prelude::*;
use crate::frames::{ReferenceFrame, transforms};

#[pyclass]
#[derive(Debug, Clone)]
pub struct State {
    pub reference_frame: ReferenceFrame,
    pub vector: Vector6<f64>,
}

#[pymethods]
impl State {
    #[new]
    pub fn new(vector: Vec<f64>, frame: ReferenceFrame) -> PyResult<Self> {
        if vector.len() != 6 {
            return Err(pyo3::exceptions::PyValueError::new_err(
                format!("vector must have 6 elements, got {}", vector.len()),
            ));
        }
        Ok(State {
            reference_frame: frame,
            vector: Vector6::new(vector[0], vector[1], vector[2], vector[3], vector[4], vector[5]),
        })
    }

    #[getter]
    pub fn vector(&self) -> Vec<f64> {
        self.vector.as_slice().to_vec()
    }

    #[getter]
    pub fn reference_frame(&self) -> ReferenceFrame {
        self.reference_frame
    }

    #[pyo3(name = "transform")]
    pub fn py_transform(&self, frame: ReferenceFrame, mu: f64) -> PyResult<State> {
        self.transform(frame, mu)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e))
    }

    pub fn __repr__(&self) -> String {
        format!("State(frame={:?}, vector={:?})", self.reference_frame, self.vector.as_slice())
    }
}

impl State {
    pub fn transform(&self, new_frame: ReferenceFrame, mu: f64) -> Result<State, String> {
        let new_vector = match (&self.reference_frame, &new_frame) {
            (ReferenceFrame::BCI, ReferenceFrame::COE) => {
                transforms::bci::bci_to_coe(self.vector, mu)
            }
            (ReferenceFrame::COE, ReferenceFrame::BCI) => {
                transforms::coe::coe_to_bci(self.vector, mu)
            }
            (from, to) if from == to => self.vector,
            (from, to) => return Err(format!("unsupported conversion: {from:?} -> {to:?}")),
        };

        Ok(State {
            reference_frame: new_frame,
            vector: new_vector,
        })
    }
}
