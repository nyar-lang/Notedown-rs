mod ast;
mod commands;
mod parser;
mod traits;
pub mod utils;

pub use ast::AST;
pub use commands::Value;
pub use parser::Context;
pub use traits::{HTMLConfig, MarkdownConfig, ToHTML, ToMarkdown};

pub fn parse_file(path_from: &str, path_to: &str) -> Result<Context, std::io::Error> {
    let c = Context::default();
    c.parse_file(path_from, path_to)
}

pub fn parse(text: &str) -> Context {
    let c = Context::default();
    c.parse(text)
}
