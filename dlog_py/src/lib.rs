use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use dlog_core::models::Priority;
use dlog_core::transforms::Transforms;

#[pyclass]
struct PythonLogger {
    core: dlog_core::Logger,
}

#[pymethods]
impl PythonLogger {
    #[new]
    fn __new__(api_key: String, email_sanitizer: bool, credit_card_sanitizer: bool) -> PyResult<Self> {
        let mut transforms = Transforms::new();
        transforms.add_credit_card_sanitizer(credit_card_sanitizer);
        transforms.add_email_sanitizer(email_sanitizer);

        match dlog_core::Logger::new(api_key, transforms) {
            Err(err) => Err(PyValueError::new_err(err)),
            Ok(val) => Ok(Self { core: val }),
        }
    }

    fn log(&self, level: i32, message: String) -> PyResult<()> {
        match self.core.log(convert_priority(level), message) {
            Err(err) => Err(PyValueError::new_err(err)),
            Ok(_) => Ok(()),
        }
    }

    fn flush(&self) -> PyResult<()> {
        match self.core.flush() {
            Err(err) => Err(PyValueError::new_err(err)),
            Ok(_) => Ok(()),
        }
    }

    fn clean_up(&self) {
        self.core.clean_up()
    }
}

fn convert_priority(level: i32) -> Priority {
    match level {
        50 => Priority::Critical,
        40 => Priority::Error,
        30 => Priority::Warning,
        20 => Priority::Info,
        _ => Priority::Debug
    }
}

#[pymodule]
fn dlog_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PythonLogger>()?;

    Ok(())
}
