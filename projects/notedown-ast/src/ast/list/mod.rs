use crate::AST;
use std::fmt::{Display, Formatter};
use std::fmt;

#[derive(Clone, Debug)]
pub enum ListView<'a> {
    Quote {
        style: &'static str,
        body: Vec<AST<'a>>,
    },
    Ordered {
        head: usize,
        body: Vec<AST<'a>>,
    },
    Orderless {
        body: Vec<AST<'a>>
    },
}

impl<'a> Display for ListView<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            ListView::Quote { style, body } => {
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