use pyo3::prelude::*;
use native::models::Priority;
use pyo3::exceptions::PyValueError;

#[pyclass]
struct PythonLogger {
    native: native::Logger
}

#[pymethods]
impl PythonLogger {
    #[new]
    fn __new__(api_key: String) -> Self {
        Self {
            native: native::Logger::new(api_key),
        }
    }

    fn log(&self, level: i32, message: String) -> PyResult<()> {
        match self.native.log(convert_priority(level), message) {
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
fn dlog_python(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PythonLogger>()?;

    Ok(())
}
