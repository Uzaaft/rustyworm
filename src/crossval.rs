use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3::wrap_pymodule;

mod submodule;
use submodule::*;

/* Generator returning combinations of items from a sequence <seq>
taken <r> at a time. Order is not significant. If <r> is not given,
the entire sequence is returned.*/
fn combinations<T>(seq: &[T], mut r: Option<size>) {
    let r = r.unwrap_or(None);
    if r == None {
        r = seq.len();
    }
    if r.unwrap() <= 0 {
        return vec![];
    } else {
        for i in 0..seq.len() {
            for cc in combinations(&seq[i + 1..], r - 1) {
                yield vec![seq[i]] + cc;
            }
        }
    }
}

/*
Leave-One-Out cross validation iterator:
Provides train/test indexes to split data in train test sets
*/
#[pyclass]
struct LeaveOneOut {
    #[pyo3(get, set)]
    value: i32,
}

#[pymethods]
impl LeaveOneOut {
    #[new]
    pub fn new(value: i32) -> Self {
        LeaveOneOut { value }
    }
}
