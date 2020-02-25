mod ast;
mod commands;
mod parser;
mod traits;
pub mod utils;

pub use ast::AST;
pub use commands::Value;
pub use traits::{Context, MarkdownConfig, NotedownTarget, NotedownTemplate, Settings, ToHTML, ToMarkdown};

pub fn parse(text: &str) -> Context {
    let mut c = Context::default();
    c.parse(text);
    return c;
}
