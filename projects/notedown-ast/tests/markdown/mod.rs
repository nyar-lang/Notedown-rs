use markdown::{tokenize};
use notedown_ast::AST;

#[test]
pub fn markdown_codegen() {
    let out = AST::from(tokenize("# 2"));
    println!("{}", out)
}