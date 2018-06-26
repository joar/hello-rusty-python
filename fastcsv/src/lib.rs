#![feature(proc_macro, specialization)]

extern crate csv;
extern crate pyo3;

use pyo3::prelude::*;
use pyo3::py::class as pyclass;
use pyo3::py::methods as pymethods;
use pyo3::py::modinit as pymodinit;
use pyo3::py::proto as pyproto;
use std::fs::File;
use std::io;


#[pyclass]
struct FastCSVReader<'r> {
    token: PyToken,
    iter: Box<Iterator<Item=csv::StringRecord> + 'r>,
//    iter: &'r csv::StringRecordsIter<'r, io::Read>,
}


fn records_iterator(
    path: String
) -> Result<Box<Iterator<Item=csv::StringRecord>>, Box<csv::Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b'\x01')
        .comment(Some(b'#'))
        .flexible(true)
        .has_headers(false)
        .terminator(csv::Terminator::Any(b'\n'))
        .from_path(path)?;

    let iter = rdr.into_records().into();
    return Ok(Box::new(iter));
}


#[pymethods]
impl<'r> FastCSVReader<'r> {
    #[new]
    fn __new__(
        obj: &PyRawObject,
        path: String,
    ) -> PyResult<()> {
//        let buf = match File::open(path) {
//            Ok(buf) => buf,
//            Err(err) => {
//                return Err(PyErr::from(err.into()))
//            }
//        };
        let iter = match records_iterator(path) {
            Ok(rdr) => rdr,
            Err(err) => {
                return Err(PyErr::new(""));
            }
        };

        let mut iter: csv::StringRecordsIter<File> = rdr.records();
        obj.init(|token| {
            FastCSVReader {
                token,
                iter: &iter,
            }
        })
    }
}


//#[pyproto]
impl<'r> PyIterProtocol<'r> for FastCSVReader<'r> {
    fn __iter__(&mut self) -> PyResult<PyObject> {
        Ok(self.into())
    }

    fn __next__(&mut self) -> PyResult<Option<PyObject>> {
        match self.iter.next() {
            Ok(record) => {
                println!("{:?}", record);
                let mut output: Vec<String> = Vec::new();
                output.try_reserve(record.len());
                output.extend(record.iter());
                return Ok(Some(output.into()));
            }
            Err(err) => {
                println!("error reading CSV from <stdin>: {}", err);
                return Ok(None);
            }
        }
    }
}


// Add bindings to the generated python module
// N.B: names: "libfastcsv2" must be the name of the `.so` or `.pyd` file
/// This module is implemented in Rust.
#[pymodinit(libfastcsv2)]
fn init_mod(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<FastCSVReader>()?;
    Ok(())
}
