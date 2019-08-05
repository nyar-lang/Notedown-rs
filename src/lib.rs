#![feature(custom_attribute)]
#![feature(structural_match)]
#![feature(rustc_attrs)]
#![feature(core_intrinsics)]

extern crate pest;
#[macro_use]
extern crate pest_derive;
#[cfg(feature = "colored")]
extern crate colored;

pub mod ast;
mod parser;
pub mod traits;
pub mod utils;

pub use ast::AST as NotedownAST;
pub use parser::{NotedownParser, NotedownRule};
pub use traits::{HTMLConfig, MarkdownConfig, ToHTML, ToMarkdown};
