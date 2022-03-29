use pyo3::prelude::*;

use lazy_static::lazy_static;
use tokio::runtime::Runtime;

use scylla::transport::session::Session;
use scylla::SessionBuilder;

lazy_static! {
    pub static ref RUNTIME: Runtime = Runtime::new().unwrap();
}

async fn connect_and_print_table_info() -> PyResult<String> {
    let session: Session = SessionBuilder::new()
        .known_node("127.0.0.1:9042".to_string())
        .build()
        .await
        .unwrap();
    let res = session
        .query(
            "SELECT keyspace_name, table_name FROM system_schema.tables",
            &[],
        )
        .await
        .unwrap();
    Ok(format!("{:?}", res))
}

/// Formats the sum of two numbers as string.
#[pyfunction]
fn smoke_test<'p>(py: Python<'p>) -> PyResult<&'p PyAny> {
    pyo3_asyncio::tokio::future_into_py(py, async move { connect_and_print_table_info().await })
}

/// A Python module implemented in Rust.
#[pymodule]
fn better_python_driver(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(smoke_test, m)?)?;
    Ok(())
}
