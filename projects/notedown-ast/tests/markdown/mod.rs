use markdown::tokenize;
use notedown_ast::AST;
use std::{fs::File, io::Write};

#[test]
pub fn markdown_codegen() {
    let out = AST::from(tokenize(include_str!("readme.md")));
    let mut file = File::create("tests/markdown/readme.note").unwrap();
    file.write_all(format!("{}", out).as_bytes()).unwrap();
}
