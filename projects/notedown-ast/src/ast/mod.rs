mod command;
mod span;
mod value;

pub use command::{Command, CommandKind};
pub use span::Span;
pub use value::{Value};

#[derive(Debug, Clone)]
pub enum AST<'a> {
    /// - `None`: It doesn't look like anything to me
    None,
    Newline,
    /// - `Statements`
    Statements(Vec<AST<'a>>),
    // Blocks






    /// - `Header`: TEXT, level
    Header(Box<AST<'a>>, u8),
    ///  - `Paragraph`:
    Paragraph(Vec<AST<'a>>),
    Code(Cow<'a, str>),
    /// - `Math`:
    MathBlock(Cow<'a, str>),
    Table {
        head: Vec<AST<'a>>,
        align: Vec<u8>,
        terms: Vec<Vec<AST<'a>>>,
        column: usize,
    },
    Quote {
        body: Vec<AST<'a>>,
        style: String,
    },
    Ordered(Vec<AST<'a>>),
    Orderless(Vec<AST<'a>>),
    /// - `Code`:
    Command(Command<'a>),
    // inlined






    /// normal text
    Text(Cow<'a, str>),
    Raw(Cow<'a, str>),
    Italic(Cow<'a, str>),
    Bold(Cow<'a, str>),
    Underline(Cow<'a, str>),
    Strikethrough(Cow<'a, str>),
    Undercover(Cow<'a, str>),

    MathInline(Cow<'a, str>),
    MathDisplay(Cow<'a, str>),

    Escaped(String),
}


use std::fmt::{Display, Formatter};
use std::fmt;
use std::borrow::Cow;

impl<'a> Display for AST<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            AST::None => write!(f, ""),
            //AST::Space => write!(f, " "),
            AST::Newline => write!(f, "\n"),

            AST::Header(a, l) => write!(f, "{} {}", "#".repeat(*l as usize), a),

            AST::Statements(e) => {
                let fs: Vec<String> = e.iter().map(|ast| format!("{}", ast)).collect();
                write!(f, "{}", fs.join(""))
            }

            AST::Paragraph(t) => {                let fs: Vec<String> = t.iter().map(|k| format!("{}", k)).collect();                write!(f, "{}", fs.join(""))            }
            AST::Raw(s) => write!(f, "{}", s),
            AST::Code(s) => write!(f, "`{}`", s),
            AST::Text(s) => write!(f, "{}", s),
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
            AST::Command(c) => write!(f, "{}", c),
            _ => {
                let a = format!("unimplemented AST::{:?}", self);
                write!(f, "{}", a.split("(").next().unwrap_or("Unknown"))
            }
        }
    }
}

