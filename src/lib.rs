extern crate bio;

mod pairwise;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use crate::pairwise::{levenshtein, Aligner, Alignment, Scoring};

#[pymodule]
fn pyrustbio(_py: Python, m: &PyModule) -> PyResult<()> {
    let pairwise_module = PyModule::new(_py, "pairwise")?;
    pairwise_module.add_class::<Aligner>()?;
    pairwise_module.add_class::<Alignment>()?;
    pairwise_module.add_class::<Scoring>()?;
    //    pairwise_module.add_wrapped(wrap_pyfunction!(levenshtein))?;
    Ok(())
}
