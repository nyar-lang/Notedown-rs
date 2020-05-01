mod command;
mod table;
mod value;
mod link;
mod list;
mod highlighter;

use std::fmt::{Display, Formatter};
use std::fmt;
use std::borrow::Cow;
use joinery::JoinableIterator;
use lazy_format::lazy_format;

pub use command::{Command, CommandKind};
pub use table::TableView;
pub use value::{Value};
pub use link::SmartLink;
pub use list::ListView;
pub use highlighter::Highlighter;

#[derive(Debug, Clone)]
pub enum AST<'a> {
    /// - `None`: It doesn't look like anything to me
    None,
    Newline,
    /// - `Statements`
    Statements(Vec<AST<'a>>),
    // Blocks






    /// - `Header`: TEXT, level
    Header(Vec<AST<'a>>, usize),
    ///  - `Paragraph`:
    Paragraph(Vec<AST<'a>>),
    Highlight(Highlighter<'a>),
    /// - `Math`:
    Math(Cow<'a, str>),
    Table(TableView<'a>),
    List(ListView<'a>),
    /// - `Code`:

    // inlined






    /// normal text
    Text(Cow<'a, str>),
    Raw(Cow<'a, str>),
    Code(Cow<'a, str>),
    Emphasis(Vec<AST<'a>>),
    Strong(Vec<AST<'a>>),
    Underline(Vec<AST<'a>>),
    Strikethrough(Vec<AST<'a>>),
    Undercover(Vec<AST<'a>>),

    MathInline(Cow<'a, str>),
    MathDisplay(Cow<'a, str>),

    Link(SmartLink<'a>),

    Escaped(char),
    //


    Command(Command<'a>),
}


impl<'a> Display for AST<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            AST::None => write!(f, ""),
            AST::Newline => write!(f, "\n"),

            AST::Header(a, l) => write!(f, "{} {}", "#".repeat(*l), join_span(a)),

            AST::Statements(e) => {
                let fs = e.iter().map(|ast| lazy_format!("{}", ast));
                write!(f, "{}", fs.join_with("\n\n").to_string())
            }

            AST::Paragraph(span) => write!(f, "{}", join_span(span)),


            AST::Raw(s) => write!(f, "{}", s),
            AST::Code(s) => write!(f, "`{}`", s),
            AST::Text(s) => write!(f, "{}", s),
            AST::Emphasis(s) => write!(f, "*{}*", join_span(s)),
            AST::Strong(s) => write!(f, "**{}**", join_span(s)),
            AST::Underline(s) => write!(f, "~{}~", join_span(s)),
            AST::Strikethrough(s) => write!(f, "~~{}~~", join_span(s)),
            AST::Undercover(s) => write!(f, "~~~{}~~~", join_span(s)),

            AST::MathInline(m) => write!(f, "${}$", m),
            AST::MathDisplay(m) => write!(f, "$${}$$", m),
            AST::Math(m) => write!(f, "$${}$$", m),

            AST::Link(link) => write!(f, "{}", link),
            AST::List(list) => write!(f, "{}", list),
            AST::Table(table) => write!(f, "{}", table),
            AST::Highlight(code) => write!(f, "{}", code),
            AST::Command(cmd) => write!(f, "{}", cmd),

            AST::Escaped(c) => { write!(f, "{}", c) }

        }
    }
}

fn join_span(v: &[AST]) -> String {
    let s = v.iter().map(|k| lazy_format!("{}", k));
    s.join_with("").to_string()
}