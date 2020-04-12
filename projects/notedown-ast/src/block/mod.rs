use crate::Command;
use std::borrow::Cow;
mod from;

#[derive(Debug, Clone)]
pub enum Block<'a> {
    /// - `None`: It doesn't look like anything to me
    None,
    Space,
    Newline,
    /// - `Statements`
    Statements(Vec<Block<'a>>),

    /// - `Header`: TEXT, level
    Header(Box<Block<'a>>, u8),

    ///  - `Paragraph`:
    Paragraph(Box<Block<'a>>),
    Text(Vec<Block<'a>>),
    /// - `String`: Normal string with no style
    String(Cow<'a, str>),
    /// - `Bold`:
    Bold(Box<Block<'a>>),
    Italic(Box<Block<'a>>),
    /// - `Underline`:
    Underline(Box<Block<'a>>),
    Strikethrough(Box<Block<'a>>),
    Undercover(Box<Block<'a>>),

    Code(Cow<'a, str>),
    Raw(Cow<'a, str>),
    /// - `Math`:
    MathInline(Cow<'a, str>),
    MathDisplay(Cow<'a, str>),

    Table {
        head: Vec<Block<'a>>,
        align: Vec<u8>,
        terms: Vec<Vec<Block<'a>>>,
        column: usize,
    },
    Quote {
        body: Vec<Block<'a>>,
        style: String,
    },
    Ordered(Vec<Block<'a>>),
    Orderless(Vec<Block<'a>>),
    /// - `Code`:
    Command(Command<'a>),
    Escaped(String),
}


use std::fmt::{Display, Formatter};
use std::fmt;

impl<'a> Display for Block<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Block::None => write!(f, ""),
            Block::Space => write!(f, " "),
            Block::Newline => write!(f, "\n"),

            Block::Header(a, l) => write!(f, "{} {}", "#".repeat(*l as usize), a),

            Block::Statements(e) => {
                let fs: Vec<String> = e.iter().map(|ast| format!("{}", ast)).collect();
                write!(f, "{}", fs.join(""))
            }

            Block::Paragraph(t) => write!(f, "{}\n", t),

            Block::Text(t) => {
                let fs: Vec<String> = t.iter().map(|k| format!("{}", k)).collect();
                write!(f, "{}", fs.join(""))
            }
            Block::Raw(s) => write!(f, "{}", s),
            Block::Code(s) => write!(f, "`{}`", s),
            Block::String(s) => write!(f, "{}", s),
            Block::Italic(s) => write!(f, "*{}*", s),
            Block::Bold(s) => write!(f, "**{}**", s),
            Block::Underline(s) => write!(f, "~{}~", s),
            Block::Strikethrough(s) => write!(f, "~~{}~~", s),
            Block::Undercover(s) => write!(f, "~~~{}~~~", s),

            Block::MathInline(s) => write!(f, "${}$", s),
            Block::MathDisplay(s) => write!(f, "$${}$$", s),

            Block::Quote { body, .. } => {
                let s: Vec<_> = body.iter().map(|a| format!("> {}", a)).collect();
                write!(f, "{}", s.join("\n"))
            }
            Block::Orderless(v) => {
                let s: Vec<_> = v.iter().map(|a| format!("- {}", a)).collect();
                write!(f, "{}", s.join("\n"))
            }
            Block::Command(c) => write!(f, "{}", c),
            _ => {
                let a = format!("unimplemented AST::{:?}", self);
                write!(f, "{}", a.split("(").next().unwrap_or("Unknown"))
            }
        }
    }
}

