mod runtime;
use pyo3::prelude::*;
use std::io::Write;

#[pyfunction]
fn stdout(arg: String) { write!(&mut std::io::stdout(), "{arg}").expect("Unable to write to stderr (file handle closed?)"); }

#[pyfunction]
fn stdout_ln(arg: String) { write!(&mut std::io::stdout(), "{arg}\n").expect("Unable to write to stderr (file handle closed?)"); }

#[pyfunction]
fn fetch(url: String) -> PyResult<String> {
    let client = reqwest::blocking::Client::new();
    let body = client.get(url).send().unwrap();

    Ok(body.text().unwrap())
}

#[pyfunction]
fn eval_javascript(script: String) -> PyResult<String> { Ok(runtime::eval_js(Box::leak(script.into_boxed_str())).unwrap()) }

#[pyfunction]
fn run_javascript(script: String) {
    let runtime = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();
    if let Err(error) = runtime.block_on(runtime::run_js(Box::leak(script.into_boxed_str()))) {
        eprintln!("error: {error}");
    }
}

#[pymodule]
#[pyo3(name = "rust")]
fn python_bindings(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(stdout, m)?)?;
    m.add_function(wrap_pyfunction!(stdout_ln, m)?)?;
    m.add_function(wrap_pyfunction!(fetch, m)?)?;
    m.add_function(wrap_pyfunction!(eval_javascript, m)?)?;
    m.add_function(wrap_pyfunction!(run_javascript, m)?)?;
    Ok(())
}
