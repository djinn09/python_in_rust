use pyo3::prelude::*;
use pyo3::types::PyDict;

fn main() -> PyResult<()> {
    let source = r#"
def foo():
    return 1
"#;

    Python::with_gil(|py| -> PyResult<()> {
        // Put current dir on sys.path so Python package imports resolve (if needed)
        let sys = py.import("sys")?;
        let path: &pyo3::types::PyList = sys.getattr("path")?.downcast()?;
        path.insert(0, ".")?;

        // small Python script that defines transformer and a function to call it
        let code = r#"
import libcst as cst

class PrependCommentTransformer(cst.CSTTransformer):
    def __init__(self, comment_text):
        self.comment_text = comment_text

    def leave_FunctionDef(self, original_node, updated_node):
        new_leading = (cst.EmptyLine(comment=cst.Comment(self.comment_text)),) + tuple(original_node.leading_lines)
        return updated_node.with_changes(leading_lines=new_leading)

def transform_code(src, comment_text):
    module = cst.parse_module(src)
    transformer = PrependCommentTransformer(comment_text)
    new_module = module.visit(transformer)
    return new_module.code
"#;

        // Execute the code in a fresh module context
        let locals = PyDict::new(py);
        // Pass `locals` for both globals and locals to ensure `cst` is in scope
        py.run(code, Some(locals), Some(locals))?;

        // Extract the defined function and call it
        let transform = locals.get_item("transform_code").expect("transform_code not found");
        let transformed: String = transform.call1((source, "# PREPENDED INLINE"))?.extract()?;

        println!("--- Transformed (inline) ---\n{}\n", transformed);
        Ok(())
    })
}