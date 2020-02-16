mod ast;
mod parser;
mod traits;
mod commands;
pub mod utils;

pub use ast::AST;
pub use traits::{HTMLConfig, ToHTML, ToMarkdown, MarkdownConfig};
pub use parser::Context;
pub use commands::Value;


pub fn parse_file(path_from: &str, path_to: &str) -> Result<Context, std::io::Error> {
    let c = Context::default();
    c.parse_file(path_from, path_to)
}

pub fn parse(text: &str) -> Context {
    let c = Context::default();
    c.parse(text)
}