# Python Parsing and Transformation in Rust

This project demonstrates three different methods for parsing and manipulating Python code in a Rust application.

## 1. `tree-sitter` for Concrete Syntax Trees (CST)

The `tree-sitter` crate provides a robust and efficient parser that generates a Concrete Syntax Tree (CST). A CST is a detailed representation of the source code, including all whitespace, comments, and punctuation. This makes it ideal for applications that require a lossless representation of the code, such as formatters, linters, and refactoring tools.

### Example

The `main` binary demonstrates how to use `tree-sitter` to parse a Python file and inspect the resulting CST:

```rust
use tree_sitter::Parser;
use tree_sitter_python::language;

fn main() -> anyhow::Result<()> {
    let mut parser = Parser::new();
    parser.set_language(language())?;

    let source = std::fs::read_to_string("sample.py")?;
    let tree = parser.parse(&source, None).expect("parse failed");
    let root = tree.root_node();

    println!("root kind: {}", root.kind());
    for child in root.children(&mut root.walk()) {
        println!("child: {}  span: {}..{}", child.kind(), child.start_byte(), child.end_byte());
    }

    Ok(())
}
```

## 2. `rustpython-parser` for Abstract Syntax Trees (AST)

The `rustpython-parser` crate is part of the RustPython project and provides a Python-compliant parser that generates an Abstract Syntax Tree (AST). An AST is a more abstract representation of the code that focuses on the syntactic structure and meaning, omitting details like whitespace and comments. This makes it well-suited for applications that analyze the code's logic, such as compilers, interpreters, and static analysis tools.

### Example

The `rustpython_parser_example` binary shows how to use `rustpython-parser` to parse a Python string and generate an AST:

```rust
use rustpython_parser::{Parse, ast};

fn main() {
    let src = "def add(x, y):\n    return x + y\n";
    match ast::Suite::parse(src, "<embedded>") {
        Ok(suite) => {
            // `suite` is an AST::Suite (module body) — inspect nodes
            println!("Parsed {} statements", suite.len());
            // walk nodes (pattern-match on rustpython AST node enums)
        }
        Err(e) => eprintln!("parse error: {:?}", e),
    }
}
```

## 3. `PyO3` and `LibCST` for Advanced Code Transformation

The `PyO3` crate allows for seamless interoperability between Rust and Python. By leveraging `PyO3`, we can directly call Python libraries like `LibCST` from within a Rust application. This approach is powerful because it gives us access to the full `LibCST` API for round-trip parsing and modification, while still writing our main application in Rust.

This method requires a Python environment with `LibCST` installed. We use a Python helper module to define the transformation logic, which is then called from Rust.

### Setup

Before running the example, you need to set up a Python virtual environment and install the necessary dependencies:

```bash
# Create and activate a Python virtual environment
python3 -m venv .venv
source .venv/bin/activate

# Install LibCST
pip install libcst
```

### Python Helper: `libcst_helper.py`

This file contains the core `LibCST` transformation logic. We define a `CSTTransformer` that prepends a comment to every function definition.

```python
# libcst_helper.py
import libcst as cst
from typing import Tuple

class PrependCommentTransformer(cst.CSTTransformer):
    def __init__(self, comment_text: str = "# Auto-prepended comment"):
        self.comment_text = comment_text

    def leave_FunctionDef(
        self,
        original_node: cst.FunctionDef,
        updated_node: cst.FunctionDef
    ) -> cst.FunctionDef:
        # Make a new EmptyLine with the comment and prepend it to leading_lines
        new_leading = (cst.EmptyLine(comment=cst.Comment(self.comment_text)),) + tuple(original_node.leading_lines)
        return updated_node.with_changes(leading_lines=new_leading)

def transform_code(source: str, comment_text: str = "# Auto-prepended comment") -> str:
    """
    Parse source, run transformer that prepends `comment_text` before each function,
    and return the transformed (round-tripped) source code.
    """
    module = cst.parse_module(source)
    transformer = PrependCommentTransformer(comment_text=comment_text)
    new_module = module.visit(transformer)
    return new_module.code
```

### Rust Example: `libcst_pyo3_example.rs`

The Rust binary imports the `libcst_helper.py` module and calls the `transform_code` function to perform the transformation.

```rust
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
```
