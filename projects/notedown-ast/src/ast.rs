use crate::Value;
use std::{
    collections::HashMap,
    fmt::{self, Display, Formatter},
};

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

    Code(String),
    Raw(String),
    /// - `Math`:
    MathInline(String),
    MathDisplay(String),

    Table {
        head: Vec<AST>,
        align: Vec<u8>,
        terms: Vec<Vec<AST>>,
        column: usize,
    },
    Quote {
        body: Vec<AST>,
        style: String,
    },
    Ordered(Vec<AST>),
    Orderless(Vec<AST>),

    /// - `Code`:
    Command(String, Vec<Value>, HashMap<String, Value>),

    Escaped(String),
}

impl Display for AST {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            AST::None => write!(f, ""),
            AST::Space => write!(f, " "),
            AST::Newline => write!(f, "\n"),

            AST::Header(a, l) => write!(f, "{} {}", "#".repeat(*l as usize), a),

            AST::Statements(e) => {
                let fs: Vec<String> = e.iter().map(|ast| format!("{}", ast)).collect();
                write!(f, "{}", fs.join(""))
            }

            AST::Paragraph(t) => write!(f, "{}\n", t),

            AST::Text(t) => {
                let fs: Vec<String> = t.iter().map(|k| format!("{}", k)).collect();
                write!(f, "{}", fs.join(""))
            }
            AST::Raw(s) => write!(f, "{}", s),
            AST::Code(s) => write!(f, "`{}`", s),
            AST::String(s) => write!(f, "{}", s),
            AST::Italic(s) => write!(f, "*{}*", s),
            AST::Bold(s) => write!(f, "**{}**", s),
            AST::Underline(s) => write!(f, "~{}~", s),
            AST::Strikethrough(s) => write!(f, "~~{}~~", s),
            AST::Undercover(s) => write!(f, "~~~{}~~~", s),

            AST::MathInline(s) => write!(f, "${}$", s),
            AST::MathDisplay(s) => write!(f, "$${}$$", s),

            AST::Quote { body, .. } => {
                let s: Vec<_> = body.iter().map(|a| format!("> {}", a)).collect();
                write!(f, "{}", s.join("\n"))
            }
            AST::Orderless(v) => {
                let s: Vec<_> = v.iter().map(|a| format!("- {}", a)).collect();
                write!(f, "{}", s.join("\n"))
            }
            AST::Command(s, args, kvs) => {
                let a: Vec<String> = args.iter().map(|v| format!("{}", v)).collect();
                let kv: Vec<String> = kvs.iter().map(|(k, v)| format!("{} = {}", k, v)).collect();

                write!(f, "\\{}{{{}}}", s, [&a[..], &kv[..]].concat().join(", "))
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

impl From<String> for AST {
    fn from(s: String) -> Self {
        AST::String(s)
    }
}
