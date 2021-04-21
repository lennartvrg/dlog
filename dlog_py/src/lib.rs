use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;

use dlog_core::models::Priority;

#[pyclass]
struct PythonLogger {
    native: native::Logger
}

#[pymethods]
impl PythonLogger {
    #[new]
    fn __new__(api_key: String) -> PyResult<Self> {
        match native::Logger::new(api_key) {
            Err(err) => Err(PyValueError::new_err(err)),
            Ok(val) => Ok(Self {
                native: val,
            })
        }
    }

    fn log(&self, level: i32, message: String) -> PyResult<()> {
        match self.native.log(convert_priority(level), message) {
            Err(err) => Err(PyValueError::new_err(err)),
            Ok(_) => Ok(())
        }
    }

    fn flush(&self) -> PyResult<()> {
        match self.native.flush() {
            Err(err) => Err(PyValueError::new_err(err)),
            Ok(_) => Ok(())
        }
    }

    fn clean_up(&self) -> PyResult<()> {
        Ok(self.native.clean_up())
    }
}

fn convert_priority(level: i32) -> Priority {
    match level {
        50 => Priority::Critical,
        40 => Priority::Error,
        30 => Priority::Warning,
        20 => Priority::Informational,
        10 => Priority::Debug,
        _ => Priority::None,
    }
}

#[pymodule]
fn dlog_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PythonLogger>()?;

    Ok(())
}
