#[cfg(not(test))]
pub use self::derive::{Parser as NotedownParser, Rule as NotedownRule};
#[cfg(test)]
pub use self::result::{Parser as NotedownParser, Rule as NotedownRule};

pub mod derive;
#[allow(unused_imports, unused_attributes)]
pub mod result;

use crate::{ToAST, NotedownAST as AST};

impl ToAST for NotedownParser {
    fn to_ast(&self) -> AST {
        unimplemented!()
    }
}
