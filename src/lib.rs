#![feature(custom_attribute)]
#![feature(structural_match)]
#![feature(rustc_attrs)]
#![feature(core_intrinsics)]

extern crate pest;
#[macro_use]
extern crate pest_derive;
#[cfg(feature = "colored")]
extern crate colored;

mod ast;
mod parser;
#[macro_use]
mod traits;
pub mod utils;

pub use ast::AST as NotedownAST;
pub use parser::{NotedownParser, NotedownRule, TextModeParser, TextModeRule};
pub use traits::{HTMLConfig, MarkdownConfig, ToAST, ToHTML, ToMarkdown};
