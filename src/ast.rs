use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone)]
pub enum NotedownAST {
    /// - Header(level, `AST`)
    Header(u8, Box<NotedownAST>),

    /// - Node(`AST`)
    Node(Box<NotedownAST>),
}

impl Display for NotedownAST {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            NotedownAST::Header(ref level, ref s) => write!(f, "h{}", level),
            _ => write!(f, "unknown"),
        }
    }
}
