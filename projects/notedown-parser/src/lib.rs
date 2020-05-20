mod config;
mod note_down;
mod parser;
mod error;
// pub mod utils;
pub use config::ParserConfig;
pub use error::{Error, ParserResult};

pub use note_down::{NoteDownParser, Rule as NoteDownRule};
pub use notedown_ast::AST;
// pub use note_text::{NoteTextParser, Rule as NoteTextRule};

pub fn parse(s: &str) -> AST {
    let mut cfg = ParserConfig::default();
    cfg.parse(s).unwrap_or(AST::None)
}
