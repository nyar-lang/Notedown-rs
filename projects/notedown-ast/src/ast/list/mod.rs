use crate::AST;
use std::{
    fmt,
    fmt::{Display, Formatter},
};

#[derive(Clone, Debug)]
pub enum ListView {
    Quote { style: Option<String>, body: Vec<AST> },
    Ordered { head: usize, body: Vec<AST> },
    Orderless { body: Vec<AST> },
}
/*
impl Display for ListView {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            ListView::Quote { style: _, body } => {
                let s: Vec<_> = body.iter().map(|a| format!("> {}", a)).collect();
                write!(f, "{}", s.join("\n"))
            }
            ListView::Ordered { head, body } => {
                let s: Vec<_> = body.iter().map(|a| format!("{}. {}", head, a)).collect();
                write!(f, "{}", s.join("\n"))
            }
            ListView::Orderless { body } => {
                let s: Vec<_> = body.iter().map(|a| format!("- {}", a)).collect();
                write!(f, "{}", s.join("\n"))
            }
        }
    }
}
*/