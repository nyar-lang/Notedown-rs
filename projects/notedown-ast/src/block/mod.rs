use crate::Command;

mod display;
mod from;

#[derive(Debug, Clone)]
pub enum Block {
    /// - `None`: It doesn't look like anything to me
    None,
    Space,
    Newline,
    /// - `Statements`
    Statements(Vec<Block>),

    /// - `Header`: TEXT, level
    Header(Box<Block>, u8),

    ///  - `Paragraph`:
    Paragraph(Box<Block>),
    Text(Vec<Block>),
    /// - `String`: Normal string with no style
    String(String),
    /// - `Bold`:
    Bold(Box<Block>),
    Italic(Box<Block>),
    /// - `Underline`:
    Underline(Box<Block>),
    Strikethrough(Box<Block>),
    Undercover(Box<Block>),

    Code(String),
    Raw(String),
    /// - `Math`:
    MathInline(String),
    MathDisplay(String),

    Table {
        head: Vec<Block>,
        align: Vec<u8>,
        terms: Vec<Vec<Block>>,
        column: usize,
    },
    Quote {
        body: Vec<Block>,
        style: String,
    },
    Ordered(Vec<Block>),
    Orderless(Vec<Block>),
    /// - `Code`:
    Command(Command),
    Escaped(String),
}

