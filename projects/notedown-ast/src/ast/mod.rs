mod command;
mod span;
mod value;
mod link;
mod list;

use std::fmt::{Display, Formatter};
use std::fmt;
use std::borrow::Cow;
pub use command::{Command, CommandKind};
pub use span::Span;
pub use value::{Value};
pub use link::SmartLink;
pub use list::ListView;

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
    List(ListView<'a>),
    /// - `Code`:
    Command(Command<'a>),
    // inlined






    /// normal text
    Text(Cow<'a, str>),
    Raw(Cow<'a, str>),
    Emphasis(Vec<AST<'a>>),
    Strong(Vec<AST<'a>>),
    Underline(Vec<AST<'a>>),
    Strikethrough(Vec<AST<'a>>),
    Undercover(Vec<AST<'a>>),

    MathInline(Cow<'a, str>),
    MathDisplay(Cow<'a, str>),

    Link(SmartLink<'a>),

    Escaped(char),
}


impl<'a> Display for AST<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            AST::None => write!(f, ""),
            AST::Newline => write!(f, "\n"),

            AST::Header(a, l) => write!(f, "{} {}", "#".repeat(*l as usize), a),

            AST::Statements(e) => {
                let fs: Vec<String> = e.iter().map(|ast| format!("{}", ast)).collect();
                write!(f, "{}", fs.join("\n\n"))
            }

            AST::Paragraph(t) => {
                let fs: Vec<String> = t.iter().map(|k| format!("{}", k)).collect();
                write!(f, "{}", fs.join(""))
            }
            AST::Raw(s) => write!(f, "{}", s),
            AST::Code(s) => write!(f, "`{}`", s),
            AST::Text(s) => write!(f, "{}", s),
            AST::Emphasis(s) => write!(f, "*{}*", "s"),
            AST::Strong(s) => write!(f, "**{}**", "s"),
            AST::Underline(s) => write!(f, "~{}~", "s"),
            AST::Strikethrough(s) => write!(f, "~~{}~~", "s"),
            AST::Undercover(s) => write!(f, "~~~{}~~~", "s"),

            AST::MathInline(m) => write!(f, "${}$", m),
            AST::MathDisplay(m) => write!(f, "$${}$$", m),
            AST::MathBlock(m) => write!(f, "$${}$$", m),

            AST::Link(link) => write!(f, "{}", link),
            AST::List(list) => write!(f, "{}", list),
            AST::Table { .. } => {unimplemented!()}
            AST::Command(cmd) => write!(f, "{}", cmd),


            AST::Escaped(_) => {unimplemented!()}
        }
    }
}

