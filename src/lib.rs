#![feature(custom_attribute)]
#![feature(structural_match)]
#![feature(rustc_attrs)]
#![feature(core_intrinsics)]

extern crate pest;
#[macro_use]
extern crate pest_derive;
#[cfg(feature = "colored")]
extern crate colored;

mod parser;
pub mod utils;

pub use parser::{NotedownParser, NotedownRule};

pub mod ast;

pub use ast::NotedownAST;
