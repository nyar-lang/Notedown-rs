mod command;
mod link;

use std::fmt::{self, Debug, Display, Formatter};

pub use crate::Value;
pub use command::{Command, CommandKind};
pub use link::SmartLink;
use std::collections::HashMap;
pub use url::Url;

#[derive(Clone, Eq, PartialEq)]
pub struct TextRange {
    // pub index: u64,
    pub start: (u64, u64),
    pub end: (u64, u64),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum AST {
    /// - `None`: It doesn't look like anything to me
    None,
    Statements(Vec<AST>),
    // Blocks
    /// - `Header`: TEXT, level
    Header {
        level: usize,
        children: Vec<AST>,
        r: TextRange,
    },
    HorizontalRule {
        r: TextRange,
    },
    ///  - `Paragraph`:
    Paragraph {
        children: Vec<AST>,
        r: TextRange,
    },
    Highlight {
        lang: String,
        code: String,
        inline: bool,
        high_line: Vec<usize>,
        r: TextRange,
    },
    /// - `Math`:
    MathBlock {
        inner: String,
        r: TextRange,
    },
    TableView {
        head: Vec<AST>,
        align: Vec<Option<bool>>,
        terms: Vec<Vec<AST>>,
        column: usize,
        r: TextRange,
    },
    QuoteList {
        style: Option<String>,
        body: Vec<AST>,
        r: TextRange,
    },
    OrderedList {
        head: usize,
        body: Vec<AST>,
        r: TextRange,
    },
    OrderlessList {
        body: Vec<AST>,
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
    Italic {
        children: Vec<AST>,
        r: TextRange,
    },
    Bold {
        children: Vec<AST>,
        r: TextRange,
    },
    Emphasis {
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

impl Default for TextRange {
    fn default() -> Self {
        Self {
            // index: 0,
            start: (0, 0),
            end: (0, 0),
        }
    }
}

impl Default for AST {
    fn default() -> Self {
        Self::None
    }
}

impl Debug for TextRange {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "({}, {}) → ({}, {})", self.start.0, self.start.1, self.end.0, self.end.1)
    }
}

impl Display for AST {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            AST::None => {write!(f,"")}
            AST::Statements(v) => {
                let s:Vec<_> = v.iter().map(|e|format!("{}",e)).collect();
                write!(f, "{}", s.join("\n\n"))
            }
            AST::Header { .. } => {unimplemented!()}
            AST::HorizontalRule { .. } => {unimplemented!()}
            AST::Paragraph { .. } => {unimplemented!()}
            AST::Highlight { lang, code, inline,  .. } => {
                if *inline {
                    write!(f, "{mark}{lang}\n{body}\n{mark}", mark = "`", lang = "", body = code)
                }
                else {
                    write!(f, "{mark}{lang}\n{body}\n{mark}", mark = "`".repeat(3), lang = lang, body = code)
                }
            }
            AST::MathBlock { .. } => {unimplemented!()}
            AST::TableView { .. } => {unimplemented!()}
            AST::QuoteList { .. } => {unimplemented!()}
            AST::OrderedList { .. } => {unimplemented!()}
            AST::OrderlessList { .. } => {unimplemented!()}
            AST::Normal { .. } => {unimplemented!()}
            AST::Raw { .. } => {unimplemented!()}
            AST::Code { .. } => {unimplemented!()}
            AST::Italic { .. } => {unimplemented!()}
            AST::Bold { .. } => {unimplemented!()}
            AST::Emphasis { .. } => {unimplemented!()}
            AST::Underline { .. } => {unimplemented!()}
            AST::Strikethrough { .. } => {unimplemented!()}
            AST::Undercover { .. } => {unimplemented!()}
            AST::MathInline { .. } => {unimplemented!()}
            AST::MathDisplay { .. } => {unimplemented!()}
            AST::Link { .. } => {unimplemented!()}
            AST::Escaped { .. } => {unimplemented!()}
            AST::Command { .. } => {unimplemented!()}
            AST::String { .. } => {unimplemented!()}
            AST::Integer { .. } => {unimplemented!()}
            AST::Decimal { .. } => {unimplemented!()}
            AST::Boolean { .. } => {unimplemented!()}
            AST::Array { .. } => {unimplemented!()}
        }
    }
}

// impl Display for AST {
// fn fmt(&self, f: &mut Formatter) -> fmt::Result {
// match self {
// AST::None => write!(f, ""),
// AST::Newline { .. } => write!(f, "\n"),
// AST::Header { children: c, level: l, .. } => write!(f, "{} {}", "#".repeat(*l), join_span(c)),
// AST::Statements { children: e, .. } => {
// let fs: Vec<_> = e.iter().map(|ast| format!("{}", ast)).collect();
// write!(f, "{}", fs.join("\n\n"))
// }
//
// AST::Paragraph { children: span, .. } => write!(f, "{}", join_span(span)),
//
// AST::Raw { inner, .. } => write!(f, "{}", inner),
// AST::Code { inner, .. } => write!(f, "`{}`", inner),
// AST::Normal { inner, .. } => write!(f, "{}", inner),
// AST::Emphasis { children: s, .. } => write!(f, "*{}*", join_span(s)),
// AST::Strong { children: s, .. } => write!(f, "**{}**", join_span(s)),
// AST::Underline { children: s, .. } => write!(f, "~{}~", join_span(s)),
// AST::Strikethrough { children: s, .. } => write!(f, "~~{}~~", join_span(s)),
// AST::Undercover { children: s, .. } => write!(f, "~~~{}~~~", join_span(s)),
//
// AST::MathInline { inner, .. } => write!(f, "${}$", inner),
// AST::MathDisplay { inner, .. } => write!(f, "$${}$$", inner),
// AST::MathBlock { inner, .. } => write!(f, "$${}$$", inner),
//
// AST::Link { inner: link, .. } => write!(f, "{}", link),
// AST::List { inner: list, .. } => write!(f, "{}", list),
// AST::Table { inner: table, .. } => write!(f, "{}", table),
// AST::Highlight { inner: code, .. } => write!(f, "{}", code),
// AST::Command { inner: cmd, .. } => write!(f, "{}", cmd),
//
// AST::Escaped { inner: c, .. } => write!(f, "{}", c),
// }
// }
// }
//
// fn join_span(v: &[AST]) -> String {
// let s: Vec<String> = v.iter().map(|k| format!("{}", k)).collect();
// s.join("")
// }

impl AST {
    pub fn to_vec(self) -> Vec<AST> {
        match self {
            AST::Statements(v) => v,
            AST::Paragraph { children, .. } => children,
            _ => vec![],
        }
    }
}
