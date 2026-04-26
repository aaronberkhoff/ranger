pub mod dynamics;
pub mod frames;
pub mod state;
pub mod typing; 
use pyo3::prelude::*;

/// Root Python module. Add functions, classes, and submodules here.
#[pymodule]
fn ranger(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<frames::ReferenceFrame>()?;
    m.add_class::<state::State>()?;
    m.add_class::<dynamics::kepler::Kepler>()?;
    Ok(())
}
