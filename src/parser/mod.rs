#[cfg(not(test))]
pub use self::derive::{Parser as NotedownParser, Rule as NotedownRule};
pub use self::note_text::{Parser as TextModeParser, Rule as TextModeRule};
#[cfg(test)]
pub use self::result::{Parser as NotedownParser, Rule as NotedownRule};

mod derive;
#[allow(unused_imports, unused_attributes)]
mod result;

mod note_text;

use crate::{NotedownAST as AST, ToAST};
