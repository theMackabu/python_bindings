mod runtime;
use pyo3::prelude::*;

#[pyfunction]
fn run_javascript(script: String) {
    let runtime = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();
    if let Err(error) = runtime.block_on(runtime::run_js(Box::leak(script.into_boxed_str()))) {
        eprintln!("error: {error}");
    }
}

#[pymodule]
#[pyo3(name = "rust")]
fn python_bindings(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(run_javascript, m)?)?;
    Ok(())
}
