use pyo3::prelude::*;
use pyo3::py_run;

use scylla::transport::session;
use scylla::SessionBuilder;
use std::sync::Arc;

#[pyclass]
struct Cluster {
    addrs: Vec<String>,
}

#[pyclass]
struct Session {
    session: Arc<session::Session>,
}

#[pymethods]
impl Cluster {
    #[new]
    fn new(addrs: Vec<String>) -> Self {
        Cluster { addrs }
    }

    fn connect_async<'p>(slf: PyRefMut<'p, Self>, py: Python<'p>) -> PyResult<&'p PyAny> {
        let addrs = slf.addrs.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            Ok(Session {
                session: Arc::new(
                    SessionBuilder::new()
                        .known_nodes(&addrs)
                        .build()
                        .await
                        .unwrap(),
                ),
            })
        })
    }
}

#[pymethods]
impl Session {
    fn execute_async<'p>(
        slf: PyRefMut<'p, Self>,
        py: Python<'p>,
        query_str: String,
    ) -> PyResult<&'p PyAny> {
        let session = slf.session.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let res = session.query(query_str, &[]).await.unwrap();
            match res.rows {
                Some(vec) => Ok(vec
                    .iter()
                    .map(|row| format!("{:?}", row))
                    .collect::<Vec<String>>()),
                None => Ok(vec!["OK".to_string()]),
            }
        })
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn scylla(py: Python, m: &PyModule) -> PyResult<()> {
    let cluster_m = PyModule::new(py, "cluster")?;
    cluster_m.add_class::<Cluster>()?;
    cluster_m.add_class::<Session>()?;
    // Hacky solution for being able to import from scylla.cluster.
    // Source: https://github.com/PyO3/pyo3/issues/1517#issuecomment-808664021
    py_run!(py, cluster_m, "import sys; sys.modules['scylla.cluster'] = cluster_m");
    m.add_submodule(cluster_m)?;
    Ok(())
}
