#[macro_use]
extern crate quote;
extern crate pest_generator;
extern crate proc_macro;

use notedown_ast::AST;
use notedown_parser::parse;

mod pre_build;
// mod easy;
// mod normal;

#[test]
fn ready() {
    println!("it, works!")
}

#[test]
fn readme() {
    let ast = parse(include_str!("readme.md"));
    println!("{:#?}", ast)
}

#[test]
#[ignore]
fn gen_parser() {
    pre_build::gen_note_down();
}
