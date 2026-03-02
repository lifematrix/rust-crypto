use pyo3::prelude::*;
use marcore::pkg_version;

pub mod random;

#[pymodule]
fn marcrypto(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("__version__", pkg_version!())?;
    m.add("__author__", "lifematrix")?;
    random::bind_submodule(m)?;
    Ok(())
}
