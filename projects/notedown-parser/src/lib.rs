extern crate pest;
#[macro_use]
extern crate pest_derive;
#[cfg(feature = "colored")]
extern crate colored;

mod ast;
mod parser;
#[macro_use]
mod traits;
#[allow(dead_code)]
pub mod utils;

pub use ast::AST as NotedownAST;
pub use parser::{MathModeParser, NotedownParser, TextModeParser};
pub use parser::{MathModeRule, NotedownRule, TextModeRule};
pub use traits::{HTMLConfig, MarkdownConfig, ToHTML, ToMarkdown};
