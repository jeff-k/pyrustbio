use pyo3::prelude::*;

use bio::alignment::distance;
use bio::alignment::pairwise;

#[pyclass]
pub struct Scoring {
    gap_open: i32,
    gap_extend: i32,
    //    match_fn: (),
    //    match_scores: (),
    xclip_prefix: i32,
    xclip_suffix: i32,
    yclip_prefix: i32,
    yclip_suffix: i32,
}

#[pyclass]
pub struct Aligner {
    scoring: Scoring,
}

#[pyclass]
pub struct Alignment {
    score: i32,
    xstart: i32,
    xend: i32,
    ystart: i32,
    yend: i32,
    //    operations: (),
}

#[pymethods]
impl Alignment {
    fn str(&self) -> PyResult<String> {
        Ok("".to_string())
    }
}

#[pymethods]
impl Aligner {
    #[new]
    fn new(gap_open: i32, gap_extend: i32) -> Self {
        Aligner {
            scoring: Scoring {
                gap_open: 0,
                gap_extend: 0,
                xclip_prefix: 0,
                xclip_suffix: 0,
                yclip_prefix: 0,
                yclip_suffix: 0,
            },
        }
    }

    fn global(&self, _py: Python, x: String, y: String) -> PyResult<Alignment> {
        Ok(Alignment {
            score: 0,
            xstart: 0,
            xend: 0,
            ystart: 0,
            yend: 0,
        })
    }

    fn semiglobal(&self, _py: Python, x: String, y: String) -> PyResult<Alignment> {
        Ok(Alignment {
            score: 0,
            xstart: 0,
            xend: 0,
            ystart: 0,
            yend: 0,
        })
    }

    fn local(&self, _py: Python, x: String, y: String) -> PyResult<Alignment> {
        Ok(Alignment {
            score: 0,
            xstart: 0,
            xend: 0,
            ystart: 0,
            yend: 0,
        })
    }
}

#[pyfunction]
pub fn levenshtein(a: &str, b: &str) -> PyResult<u32> {
    Ok(distance::levenshtein(a.as_bytes(), b.as_bytes()))
}
