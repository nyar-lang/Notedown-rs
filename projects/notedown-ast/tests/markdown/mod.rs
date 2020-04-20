use markdown::{tokenize};
use notedown_ast::AST;

#[test]
pub fn markdown_codegen() {
    let out = AST::from(tokenize(include_str!("readme.md")));
    println!("{}", out)
}