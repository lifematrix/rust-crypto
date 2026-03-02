use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use std::collections::HashMap;
use marnd::{MPCfg, MPRng, MRndErr};

fn mrnderr_to_py(e: MRndErr) -> PyErr {
    PyValueError::new_err(e.to_string())
}

#[pyclass]
pub struct PyMPRng {
    inner: MPRng,
}

#[pyfunction]
pub fn from_config(py: Python<'_>, cfg: &Bound<'_, PyAny>) -> PyResult<Py<PyMPRng>> {
    let map: HashMap<String, String> = cfg.extract().map_err(|e| {
        PyValueError::new_err(format!("cfg must be dict[str, str]: {e}"))
    })?;   

    let mp_cfg = MPCfg{ c: map };
    let inner = MPRng::build(&mp_cfg).map_err(mrnderr_to_py)?;
    // Ok(PyMPRng { inner })
    Py::new(py ,PyMPRng { inner })
}

fn build_rng_with_schema(py: Python<'_>, schema: &str, seed: Option<u64>) -> PyResult<Py<PyMPRng>> {
    let mut c = HashMap::<String, String>::new();
    c.insert("schema".into(), schema.into());

    if let Some(s) = seed {
        c.insert("seed".into(), s.to_string());
    }

    let mp_cfg = MPCfg { c };
    let inner = MPRng::build(&mp_cfg).map_err(mrnderr_to_py)?;

    Py::new(py, PyMPRng { inner })
}

#[pyfunction]
#[pyo3(signature=(seed=None))]
pub fn rng_dk(py: Python<'_>, seed: Option<u64>) -> PyResult<Py<PyMPRng>> {
    build_rng_with_schema(py, "Lcg64::DK", seed)
}

#[pyfunction]
#[pyo3(signature=(seed=None))]
pub fn rng_sv(py: Python<'_>, seed: Option<u64>) -> PyResult<Py<PyMPRng>> {
    build_rng_with_schema(py, "Lcg64::SV", seed)
}

#[pyfunction]
#[pyo3(signature=(seed=None))]
pub fn rng_pcg64(py: Python<'_>, seed: Option<u64>) -> PyResult<Py<PyMPRng>> {
    build_rng_with_schema(py, "Lcg64::PCG64", seed)
}

#[pyfunction]
#[pyo3(signature=(seed=None))]
pub fn default_rng(py: Python<'_>, seed: Option<u64>) -> PyResult<Py<PyMPRng>> {
    rng_dk(py, seed)
}

#[pymethods]
impl PyMPRng {
    pub fn next_u64(&mut self) -> u64 {
        self.inner.next_u64()
    }
} 

pub fn bind_submodule(m: &Bound<'_, PyModule>) -> PyResult<()>{
    let sm = PyModule::new_bound(m.py(), "random")?;
    sm.add_function(wrap_pyfunction!(from_config, &sm)?)?;
    sm.add_function(wrap_pyfunction!(rng_dk, &sm)?)?;
    sm.add_function(wrap_pyfunction!(rng_sv, &sm)?)?;
    sm.add_function(wrap_pyfunction!(rng_pcg64, &sm)?)?;
    sm.add_function(wrap_pyfunction!(default_rng, &sm)?)?;
    sm.add_class::<PyMPRng>()?;
    m.add_submodule(&sm)?;
    m.add("random", &sm)?;
    Ok(())
}