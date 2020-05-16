#[cfg(test)]
#[macro_use]
extern crate quote;
extern crate pest;
#[cfg(test)]
extern crate pest_generator;
#[cfg(test)]
extern crate proc_macro;

#[cfg(test)]
mod pre_build;

pub mod utils;
mod config;
mod note_down;
mod parser;
pub use config::ParserConfig;
// mod note_text;

pub use note_down::{NoteDownParser, Rule as NoteDownRule};
pub use notedown_ast::AST;
// pub use note_text::{NoteTextParser, Rule as NoteTextRule};

pub fn parse(s:&str) -> AST {
    let mut cfg = ParserConfig::default();
    cfg.parse(s)
}