use pyo3::prelude::*;

fn main() -> PyResult<()> {
    // Example Python source to transform
    let source = r#"
import math

def greet(name: str) -> None:
    print(f"Hello, {name}")

def add(x, y):
    return x + y
"#;

    Python::with_gil(|py| -> PyResult<()> {
        // Ensure current directory is in sys.path (so we can import libcst_helper.py)
        let sys = py.import("sys")?;
        let path: &pyo3::types::PyList = sys.getattr("path")?.downcast()?;
        // insert at position 0 so local module resolves first
        path.insert(0, ".")?;

        // Import our helper Python module
        let helper = py.import("libcst_helper")?;

        // Call transform_code(source, comment_text)
        let func = helper.getattr("transform_code")?;
        let transformed: String = func.call1((source, "# PREPENDED VIA RUST"))?.extract()?;

        println!("--- Transformed code ---\n{}\n", transformed);

        Ok(())
    })
}
