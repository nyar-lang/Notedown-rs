use crate::{HTMLConfig, ToHTML};
use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub enum AST {
    /// - `None`: It doesn't look like anything to me
    None,
    /// - ``
    Statements(Vec<AST>),

    /// - `Header`: TEXT, level, args
    Header(Box<AST>, u8, HashMap<String, String>),

    /// - `String`: Normal string with no style
    String(String),
    /// - `Bold`:
    Bold(Box<AST>, u8),
    /// - `Italic`:
    Italic(Box<AST>, u8),
    /// - `Underline`:
    Underline(Box<AST>, u8),
    /// - `Font`:
    Font(Box<AST>, HashMap<String, String>),

    /// - `Math`:
    MathInline(String),
    MathDisplay(String),
    /// - `Code`:
    Code(String, HashMap<String, String>),

    /// - `Text`: For inline style
    Text(Box<AST>),
    /// -
    Word(String),
    Escaped(String),
    Newline,
    ///  - `Paragraph`:
    Paragraph(Box<AST>),
    /// - `Function`: input, args, kvs
    Function(String, Vec<AST>, HashMap<String, String>),
}

impl Display for AST {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            AST::Statements(ref e) => {
                let fs: Vec<String> = e.iter().map(|ast| format!("{}", ast)).collect();
                write!(f, "{}", fs.join(""))
            }
            AST::Paragraph(ref e) => {
                let fs: Vec<String> = format!("{}", e)
                    .lines()
                    .map(|k| k.trim_end().to_string())
                    .collect();
                write!(f, "{}\n\n", fs.join("\n"))
            }
            AST::Newline => write!(f, "\n"),

            AST::Word(ref s) => write!(f, "{} ", s),

            AST::MathInline(ref s) => write!(f, "${}$ ", s),

            _ => {
                let a = format!("unimplemented AST::{:?}", self);
                println!("{}", a.split("(").next().unwrap_or("Unknown"));
                write!(f, "UnimplementedError")
            }
        }
    }
}

impl From<&str> for AST {
    fn from(s: &str) -> Self {
        AST::String(s.to_string())
    }
}
