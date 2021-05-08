use pyo3::prelude::*;

use bio::alignment::pairwise;

#[pyclass]
pub struct Aligner {
    match_score: i32,
    mismatch_score: i32,
    gap_open: i32,
    gap_extend: i32,
}

#[pyclass]
pub struct Alignment {
    x: Box<String>,
    y: Box<String>,
    aligned: bio_types::alignment::Alignment,
}

#[pymethods]
impl Alignment {
    fn __str__(&self) -> PyResult<String> {
        Ok(self.aligned.pretty(self.x.as_bytes(), self.y.as_bytes()))
    }
}

#[pymethods]
impl Aligner {
    #[new]
    fn new(match_score: i32, mismatch_score: i32, gap_open: i32, gap_extend: i32) -> Self {
        Aligner {
            match_score,
            mismatch_score,
            gap_open,
            gap_extend,
        }
    }

    fn align(&self, _py: Python, a: String, b: String) -> PyResult<Alignment> {
        let x = a.as_bytes();
        let y = b.as_bytes();
        let score = |a: u8, b: u8| {
            if a == b {
                self.match_score
            } else {
                self.mismatch_score
            }
        };
        //        let scoring = pairwise::Scoring::new(self.gap_open, self.gap_extend, &score);
        let mut aligner = pairwise::Aligner::with_capacity(
            x.len(),
            y.len(),
            self.gap_open,
            self.gap_extend,
            score,
        );

        Ok(Alignment {
            x: Box::new(a.clone()),
            y: Box::new(b.clone()),
            aligned: aligner.global(x, y),
        })
    }

    //    fn semiglobal(&self, _py: Python, x: String, y: String) -> PyResult<Alignment> {
    //    }

    //    fn local(&self, _py: Python, x: String, y: String) -> PyResult<Alignment> {
    //    }
}
