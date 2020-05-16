mod command;
mod highlighter;
mod link;
mod list;
mod table;

use std::fmt::{self, Display, Formatter};

pub use crate::Value;
pub use command::{Command, CommandKind};
pub use highlighter::Highlighter;
pub use link::SmartLink;
pub use list::ListView;
pub use table::TableView;

#[derive(Debug, Clone)]
pub enum AST {
    /// - `None`: It doesn't look like anything to me
    None,
    Newline,
    /// - `Statements`
    Statements(Vec<AST>),
    // Blocks
    /// - `Header`: TEXT, level
    Header(Vec<AST>, usize),
    ///  - `Paragraph`:
    Paragraph(Vec<AST>),
    Highlight(Highlighter),
    /// - `Math`:
    Math(String),
    Table(TableView),
    List(ListView),
    /// - `Code`:

    // inlined

    /// normal text
    Text(String),
    Raw(String),
    Code(String),
    Emphasis(Vec<AST>),
    Strong(Vec<AST>),
    Underline(Vec<AST>),
    Strikethrough(Vec<AST>),
    Undercover(Vec<AST>),

    MathInline(String),
    MathDisplay(String),

    Link(SmartLink),

    Escaped(char),
    //
    Command(Command),
}

impl Display for AST {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            AST::None => write!(f, ""),
            AST::Newline => write!(f, "\n"),

            AST::Header(a, l) => write!(f, "{} {}", "#".repeat(*l), join_span(a)),

            AST::Statements(e) => {
                let fs: Vec<_> = e.iter().map(|ast| format!("{}", ast)).collect();
                write!(f, "{}", fs.join("\n\n"))
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

            AST::Escaped(c) => write!(f, "{}", c),
        }
    }
}

fn join_span(v: &[AST]) -> String {
    let s: Vec<String> = v.iter().map(|k| format!("{}", k)).collect();
    s.join("")
}
