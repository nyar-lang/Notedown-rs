use crate::Value;
use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub enum AST {
    /// - `None`: It doesn't look like anything to me
    None,
    Space,
    Newline,
    /// - `Statements`
    Statements(Vec<AST>),

    /// - `Header`: TEXT, level
    Header(Box<AST>, u8),

    ///  - `Paragraph`:
    Paragraph(Box<AST>),
    Text(Vec<AST>),
    /// - `String`: Normal string with no style
    String(String),
    /// - `Bold`:
    Bold(Box<AST>),
    Italic(Box<AST>),
    /// - `Underline`:
    Underline(Box<AST>),
    Strikethrough(Box<AST>),
    Undercover(Box<AST>),
    /// - `Font`:
    Font(Box<AST>, HashMap<String, String>),

    Code(String),
    Raw(String),
    /// - `Math`:
    MathInline(String),
    MathDisplay(String),

    TableAlign(u8),

    /// - `Code`:
    Command(String, Vec<Value>, HashMap<String, Value>),

    Escaped(String),
}

impl Display for AST {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            AST::None => write!(f, ""),
            AST::Space => write!(f, " "),
            AST::Newline => write!(f, "\n"),

            AST::Statements(ref e) => {
                let fs: Vec<String> = e.iter().map(|ast| format!("{}", ast)).collect();
                write!(f, "{}", fs.join(""))
            }

            AST::Paragraph(ref t) => write!(f, "{}\n", t),

            AST::Text(ref t) => {
                let fs: Vec<String> = t.iter().map(|k| format!("{}", k)).collect();
                write!(f, "{}", fs.join(""))
            }
            AST::Raw(ref s) => write!(f, "{}", s),
            AST::Code(ref s) => write!(f, "`{}`", s),
            AST::String(ref s) => write!(f, "{}", s),
            AST::Italic(ref s) => write!(f, "*{}*", s),
            AST::Bold(ref s) => write!(f, "**{}**", s),
            AST::Underline(ref s) => write!(f, "~{}~", s),
            AST::Strikethrough(ref s) => write!(f, "~~{}~~", s),
            AST::Undercover(ref s) => write!(f, "~~~{}~~~", s),

            AST::MathInline(ref s) => write!(f, "${}$", s),
            AST::MathDisplay(ref s) => write!(f, "$${}$$", s),

            AST::Command(ref s, ref args, ref kvs) => {
                let a: Vec<String> = args.iter().map(|v| format!("{}", v)).collect();
                let kv: Vec<String> = kvs.iter().map(|(k, v)| format!("{} = {}", k, v)).collect();

                write!(
                    f,
                    "\\{}{}{:?}",
                    s,
                    format!("[{}]", a.join(", ")),
                    format!("{{{}}}", kv.join(", "))
                )
            }
            _ => {
                let a = format!("unimplemented AST::{:?}", self);
                write!(f, "{}", a.split("(").next().unwrap_or("Unknown"))
            }
        }
    }
}

impl From<&str> for AST {
    fn from(s: &str) -> Self {
        AST::String(s.to_string())
    }
}
