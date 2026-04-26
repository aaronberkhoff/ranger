use pyo3::exceptions::PyValueError;
use pyo3::PyErr;

pub struct RangerError(pub String);

impl From<RangerError> for PyErr {
    fn from(error: RangerError) -> PyErr {
        PyValueError::new_err(error.0)
    }
}

impl From<String> for RangerError {
    fn from(message: String) -> Self {
        RangerError(message)
    }
}
