extern crate bio;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use bio::alignment::distance;
//use bio::alignment::pairwise;
//use bio::scores::blosum62::blosum62;

#[pyclass]
struct Aligner {
    b: bool,
}

#[pymethods]
impl Aligner {
    #[new]
    fn new(obj: &PyRawObject) {
        obj.init(Aligner { b: true });
    }

    //    fn affine(&self, _py: Python, query: String) -> PyResult<usize> {
    //        Ok(0)
    //    }
}

#[pyfunction]
fn levenshtein(a: &str, b: &str) -> PyResult<u32> {
    Ok(distance::levenshtein(a.as_bytes(), b.as_bytes()))
}

#[pymodule]
fn pyrustbio(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Aligner>()?;
    m.add_wrapped(wrap_pyfunction!(levenshtein))?;
    Ok(())
}
