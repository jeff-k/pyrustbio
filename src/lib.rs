extern crate bio;

mod pairwise;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use bio::alignment::distance;

use crate::pairwise::{Aligner, Alignment};

#[pyfunction]
fn levenshtein(a: &str, b: &str) -> PyResult<u32> {
    Ok(distance::levenshtein(a.as_bytes(), b.as_bytes()))
}

#[pymodule]
fn pyrustbio(_py: Python, m: &PyModule) -> PyResult<()> {
    let pairwise_module = PyModule::new(_py, "pairwise")?;
    pairwise_module.add_class::<Aligner>()?;
    pairwise_module.add_class::<Alignment>()?;

    let distance_module = PyModule::new(_py, "distance")?;
    distance_module
        .add_function(wrap_pyfunction!(levenshtein, distance_module)?)
        .unwrap();
    m.add_submodule(pairwise_module)?;
    m.add_submodule(distance_module)?;
    Ok(())
}
