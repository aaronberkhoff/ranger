pub mod transforms;

use pyo3::prelude::*;

#[pyclass(eq)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReferenceFrame {
    BCI,
    COE,
    BCRF,
    LVLH,
    RIC,
    ENU,
}

#[pymethods]
impl ReferenceFrame {
    fn __repr__(&self) -> &str {
        match self {
            ReferenceFrame::BCI => "ReferenceFrame.BCI",
            ReferenceFrame::COE => "ReferenceFrame.COE",
            ReferenceFrame::BCRF => "ReferenceFrame.BCRF",
            ReferenceFrame::LVLH => "ReferenceFrame.LVLH",
            ReferenceFrame::RIC => "ReferenceFrame.RIC",
            ReferenceFrame::ENU => "ReferenceFrame.ENU",
        }
    }
}