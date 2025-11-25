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
