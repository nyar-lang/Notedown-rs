mod ast;
mod commands;
mod parser;
mod traits;
pub mod utils;

pub use ast::AST;
pub use commands::Value;
pub use parser::Context;
use std::fs::read_to_string;
pub use traits::{HTMLConfig, MarkdownConfig, ToHTML, ToMarkdown};

pub fn parse_file(path_from: &str) -> Result<Context, std::io::Error> {
    let r = read_to_string(path_from)?;
    let a = parse(&r);
    return Ok(a);
}
pub fn parse(text: &str) -> Context {
    let mut c = Context::default();
    c.parse(text);
    return c;
}
