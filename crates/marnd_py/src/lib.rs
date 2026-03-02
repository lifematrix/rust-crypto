use std::collections::HashMap;

use pyo3::exceptions::PyValueError;
use pyo3::prelude::PyErr;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use marcore::pkg_version;
use marnd::{MPCfg, MPRng, MRndErr};

// impl From<MRndErr> for PyErr {
//     fn from(e: MRndErr) -> PyErr {
//         PyValueError::new_err(e.to_string())
//     }
// }

fn mrnderr_to_py(e: MRndErr) -> PyErr {
    PyValueError::new_err(e.to_string())
}

#[pyclass]
pub struct PyMPRng {
    inner: MPRng,
}

#[pyfunction]
pub fn create_rng(cfg: &Bound<'_, PyAny>) -> PyResult<PyMPRng> {
    let map: HashMap<String, String> = cfg.extract().map_err(|e| {
        PyValueError::new_err(format!("cfg must be dict[str, str]: {e}"))
    })?;   

    let mp_cfg = MPCfg{ c: map };
    let inner = MPRng::build(&mp_cfg).map_err(mrnderr_to_py)?;
    Ok(PyMPRng { inner })
}

#[pymethods]
impl PyMPRng {
    // pub fn build(_cls: &Bound<'_, PyType>, cfg: &Bound<'_, PyAny>) -> PyResult<Self> {
    // pub fn build(_cls: &Bound<'_, PyType>, cfg: &Bound<'_, PyAny>) -> PyResult<Self> { 
    //     let map: HashMap<String, String> = cfg.extract().map_err(|e| {
    //         PyValueError::new_err(format!("cfg must be dict[str, str]: {e}"))
    //     })?;
    // #[classmethod]
    // pub fn build(_cls: &Bound<'_, PyType>, cfg: &Bound<'_, PyAny>) -> PyResult<Self> {
    //     let map: HashMap<String, String> = cfg.extract().map_err(|e| {
    //         PyValueError::new_err(format!("cfg must be dict[str, str]: {e}"))
    //     })?;   

    //     let mp_cfg = MPCfg{ c: map };
    //     let inner = MPRng::build(&mp_cfg).map_err(mrnderr_to_py)?;
    //     Ok(Self { inner })
    // }

    pub fn next_u64(&mut self) -> u64 {
        self.inner.next_u64()
    }
} 

#[pymodule]
fn marnd_py(_py: Python<'_>, m: &Bound<'_,PyModule>) -> PyResult<()> {
    m.add("__version__", pkg_version!())?;
    m.add("__author__", "lifematrix")?;
    m.add_function(wrap_pyfunction!(create_rng, m)?)?;
    m.add_class::<PyMPRng>()?;
    Ok(())
}
