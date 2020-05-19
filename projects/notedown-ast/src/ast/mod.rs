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

use url::Url;

#[derive(Debug, Clone)]
pub struct TextRange {
    file: Url,
    start: (u64, u64),
    end: (u64, u64),
}

impl Default for TextRange {
    fn default() -> Self {
        Self {
            file: (),
            start: (0, 0),
            end: (0, 0)
        }
    }
}

#[derive(Debug, Clone)]
pub enum AST {
    /// - `None`: It doesn't look like anything to me
    None,
    Newline {
        r: TextRange
    },
    /// - `Statements`
    Statements {
        children: Vec<AST>,
        r: TextRange
    },

    // Blocks
    /// - `Header`: TEXT, level
    Header {
        children: Vec<AST>,
        level: usize,
        r: TextRange
    },
    ///  - `Paragraph`:
    Paragraph {
        children: Vec<AST>,
        r: TextRange
    },
    Highlight {
        inner: Highlighter,
        r: TextRange
    },
    /// - `Math`:
    MathBlock {
        inner:String,
        r: TextRange
    },
    Table{
        inner: TableView,
        r: TextRange
    },
    List{
        inner:ListView,
        r: TextRange
    },
    /// - `Code`:

    // inlined
    Normal{
        inner: String,
        r: TextRange
    },
    Raw{
        inner:String,
        r: TextRange
    },
    /// `` `code` ``
    Code{
        inner: String,
        r: TextRange
    },
    Emphasis{
        children: Vec<AST>,
        r: TextRange
    },
    Strong{
        children:Vec<AST>,
        r: TextRange
    },
    Underline{
        children:Vec<AST>,
        r: TextRange
    },
    Strikethrough {
        children:Vec<AST>,
        r: TextRange
    },
    Undercover {
        children:Vec<AST>,
        r: TextRange
    },

    MathInline{
        inner: String,
        r: TextRange
    },
    MathDisplay{
        inner: String,
        r: TextRange
    },

    Link{
        inner: SmartLink,
        r: TextRange
    },

    Escaped{
        inner:char,
        r: TextRange
    },
    //
    Command{
        inner: Command,
        r: TextRange
    },
}

impl Display for AST {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {

            /*
            AST::Header(a, l) => write!(f, "{} {}", "#".repeat(*l), join_span(a)),

            AST::Statements(e) => {
                let fs: Vec<_> = e.iter().map(|ast| format!("{}", ast)).collect();
                write!(f, "{}", fs.join("\n\n"))
            }

            AST::Paragraph(span) => write!(f, "{}", join_span(span)),

            AST::Raw(s) => write!(f, "{}", s),
            AST::Code(s) => write!(f, "`{}`", s),
            AST::Normal(s) => write!(f, "{}", s),
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
             */
            AST::None => write!(f, ""),
            AST::Newline { .. } => write!(f, "\n"),
            AST::Statements { .. } => {}
            AST::Header { .. } => {}
            AST::Paragraph { .. } => {}
            AST::Highlight { .. } => {}
            AST::MathBlock { .. } => {}
            AST::Table { .. } => {}
            AST::List { .. } => {}
            AST::Normal { .. } => {}
            AST::Raw { .. } => {}
            AST::Code { .. } => {}
            AST::Emphasis { .. } => {}
            AST::Strong { .. } => {}
            AST::Underline { .. } => {}
            AST::Strikethrough { .. } => {}
            AST::Undercover { .. } => {}
            AST::MathInline { .. } => {}
            AST::MathDisplay { .. } => {}
            AST::Link { .. } => {}
            AST::Escaped { .. } => {}
            AST::Command { .. } => {}
        }
    }
}

fn join_span(v: &[AST]) -> String {
    let s: Vec<String> = v.iter().map(|k| format!("{}", k)).collect();
    s.join("")
}
