# Python Parsing in Rust

This project demonstrates two different methods for parsing Python code in a Rust application.

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
