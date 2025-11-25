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
