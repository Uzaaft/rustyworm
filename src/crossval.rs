use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3::wrap_pymodule;
use ndarray;
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
#[derive(Debug)]
#[pyclass]
struct LeaveOneOut {
    #[pyo3(get, set)]
    value: i32,
}


impl std::fmt::Display for MyStruct {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "LeaveOneOut( n={} )", self.n) // Should perhaps rewrite this to more like the python code.
    }
}

#[pymethods]
impl LeaveOneOut {
    #[new]
    pub fn new(value: i32) -> Self {
        LeaveOneOut { value }
    }
}


impl std::iter::Iterator for LeaveOneOut {
    type Item = (usize, usize);
    fn next(&mut self) -> Option<Self::Item> {
        for i in 0..self.n {
            test_index = ndarray::ArrayBase::zeros(self.n)
            test_index = test_index.mapv(|x| x == 1); ¨
            // ^Less than ideal. Perhaps we can just use 0 and 1 instead of bools
            test_index[i] = true;
            train_index = ndarray::ArrayBase::zeros(self.n)
            train_index = train_index.mapv(|x| x == test_index);
            yield (train_index, test_index);
            }
}
