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
use std::collections::HashMap;


#[derive(Debug, Clone)]
pub struct TextRange {
    file: (),
    index: u64,
    start: (u64, u64),
    end: (u64, u64),
}

impl Default for TextRange {
    fn default() -> Self {
        Self {
            file: (),
            index: 0,
            start: (0, 0),
            end: (0, 0),
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
        r: TextRange,
    },
    // Blocks
    /// - `Header`: TEXT, level
    Header {
        children: Vec<AST>,
        level: usize,
        r: TextRange,
    },
    ///  - `Paragraph`:
    Paragraph {
        children: Vec<AST>,
        r: TextRange,
    },
    Highlight {
        inner: Highlighter,
        r: TextRange,
    },
    /// - `Math`:
    MathBlock {
        inner: String,
        r: TextRange,
    },
    Table {
        inner: TableView,
        r: TextRange,
    },
    List {
        inner: ListView,
        r: TextRange,
    },
    /// - `Code`:

    // inlined
    Normal {
        inner: String,
        r: TextRange,
    },
    Raw {
        inner: String,
        r: TextRange,
    },
    /// `` `code` ``
    Code {
        inner: String,
        r: TextRange,
    },
    Emphasis {
        children: Vec<AST>,
        r: TextRange,
    },
    Strong {
        children: Vec<AST>,
        r: TextRange,
    },
    Underline {
        children: Vec<AST>,
        r: TextRange,
    },
    Strikethrough {
        children: Vec<AST>,
        r: TextRange,
    },
    Undercover {
        children: Vec<AST>,
        r: TextRange,
    },

    MathInline {
        inner: String,
        r: TextRange,
    },
    MathDisplay {
        inner: String,
        r: TextRange,
    },

    Link {
        inner: SmartLink,
        r: TextRange,
    },

    Escaped {
        inner: char,
        r: TextRange,
    },
    //
    Command {
        cmd: String,
        args: Vec<Value>,
        kvs: HashMap<String, Value>,
        kind: CommandKind,
        r: TextRange,
    },

    String {
        inner: String,
        r: TextRange,
    },
    Integer {
        inner: String,
        r: TextRange,
    },
    Decimal {
        inner: String,
        r: TextRange,
    },
    Boolean {
        inner: bool,
        r: TextRange,
    },
    Array {
        inner: Vec<AST>,
        r: TextRange,
    },
}
/*
impl Display for AST {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            AST::None => write!(f, ""),
            AST::Newline { .. } => write!(f, "\n"),
            AST::Header { children: c, level: l, .. } => write!(f, "{} {}", "#".repeat(*l), join_span(c)),
            AST::Statements { children: e, .. } => {
                let fs: Vec<_> = e.iter().map(|ast| format!("{}", ast)).collect();
                write!(f, "{}", fs.join("\n\n"))
            }

            AST::Paragraph { children: span, .. } => write!(f, "{}", join_span(span)),

            AST::Raw { inner, .. } => write!(f, "{}", inner),
            AST::Code { inner, .. } => write!(f, "`{}`", inner),
            AST::Normal { inner, .. } => write!(f, "{}", inner),
            AST::Emphasis { children: s, .. } => write!(f, "*{}*", join_span(s)),
            AST::Strong { children: s, .. } => write!(f, "**{}**", join_span(s)),
            AST::Underline { children: s, .. } => write!(f, "~{}~", join_span(s)),
            AST::Strikethrough { children: s, .. } => write!(f, "~~{}~~", join_span(s)),
            AST::Undercover { children: s, .. } => write!(f, "~~~{}~~~", join_span(s)),

            AST::MathInline { inner, .. } => write!(f, "${}$", inner),
            AST::MathDisplay { inner, .. } => write!(f, "$${}$$", inner),
            AST::MathBlock { inner, .. } => write!(f, "$${}$$", inner),

            AST::Link { inner: link, .. } => write!(f, "{}", link),
            AST::List { inner: list, .. } => write!(f, "{}", list),
            AST::Table { inner: table, .. } => write!(f, "{}", table),
            AST::Highlight { inner: code, .. } => write!(f, "{}", code),
            AST::Command { inner: cmd, .. } => write!(f, "{}", cmd),

            AST::Escaped { inner: c, .. } => write!(f, "{}", c),
        }
    }
}

fn join_span(v: &[AST]) -> String {
    let s: Vec<String> = v.iter().map(|k| format!("{}", k)).collect();
    s.join("")
}
*/