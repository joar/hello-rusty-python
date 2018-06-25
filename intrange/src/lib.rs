#![feature(proc_macro, specialization)]

extern crate pyo3;

use pyo3::prelude::*;

use pyo3::py::methods as pymethods;
use pyo3::py::class as pyclass;
use pyo3::py::proto as pyproto;
use pyo3::py::modinit as pymodinit;

#[pyclass]
struct IntRange {
    start: u64,
    stop: u64,
    current: Option<u64>,
    token: PyToken,
}

#[pymethods]
impl IntRange {
    #[new]
    fn __new__(obj: &PyRawObject, start: u64, stop: u64) -> PyResult<()> {
        obj.init(|token| {
            IntRange {
                start,
                stop,
                current: None,
                token,
            }
        })
    }
}

#[pyproto]
impl PyIterProtocol for IntRange {
    fn __iter__(&mut self) -> PyResult<PyObject> {
        Ok(self.into())
    }
    fn __next__(&mut self) -> PyResult<Option<u64>> {
        match self.current {
            Some(current) => {
                let next = current + 1;
                if next < self.stop {
                    self.current = Some(next);
                    return Ok(self.current)
                }
                return Ok(None)
            }
            None => {
                self.current = Some(self.start);
                return Ok(self.current)
            }
        }
    }
}

// Add bindings to the generated python module
// N.B: names: "libintrange" must be the name of the `.so` or `.pyd` file
/// This module is implemented in Rust.
#[pymodinit(libintrange)]
fn init_mod(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<IntRange>()?;
    Ok(())
}

