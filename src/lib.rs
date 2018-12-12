#![feature(specialization)]

#[macro_use]
extern crate pyo3;
extern crate bio;

use bio::alignment::distance;
use pyo3::prelude::*;
//use bio::alignment::pairwise;
//use bio::scores::blosum62::blosum62;

#[pymodule]
fn pyrustbio(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Aligner>()?;
    m.add_wrapped(wrap_function!(levenshtein))?;
    Ok(())
}

#[pyclass]
struct Aligner {
    b: bool,
}

#[pymethods]
impl Aligner {
    #[new]
    fn __new__(obj: &PyRawObject, reference: String) -> PyResult<()> {
        obj.init(|| Aligner { b: true })
    }

    fn affine(&self, _py: Python, query: String) -> PyResult<usize> {
        Ok(0)
    }
}

#[pyfunction]
fn levenshtein(a: &str, b: &str) -> u32 {
    distance::levenshtein(a.as_bytes(), b.as_bytes())
}
