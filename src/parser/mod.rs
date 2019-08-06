#[cfg(not(test))]
pub use self::derive::{Parser as NotedownParser, Rule as NotedownRule};
#[cfg(test)]
pub use self::result::{Parser as NotedownParser, Rule as NotedownRule};
pub use self::{Rule as TextModeRule, TextMode as TextModeParser};

pub mod derive;
#[allow(unused_imports, unused_attributes)]
pub mod result;

#[derive(Parser)]
#[grammar = "text_mode.pest"]
#[allow(dead_code)]
pub struct TextMode;

use crate::{NotedownAST as AST, ToAST};

impl ToAST for NotedownParser {
    fn to_ast(&self) -> AST {
        unimplemented!()
    }
}
