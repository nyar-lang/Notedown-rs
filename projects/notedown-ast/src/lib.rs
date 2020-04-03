use std::{
    collections::HashMap,
};
mod traits;

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
    Command(Box<str>, Vec<Value>, HashMap<String, Value>),

    Escaped(String),
}

pub enum Span {
    /// - `String`: Normal string with no style
    String(String),
    /// - `Bold`:
    Bold(Box<AST>),
    Italic(Box<AST>),
    /// - `Underline`:
    Underline(Box<AST>),
    Strikethrough(Box<AST>),
    Undercover(Box<AST>),
}

#[derive(Clone, PartialEq)]
pub enum Value {
    None,
    String(String),
    // Integer(String),
    // Decimal(String),
    Boolean(bool),
    List(Vec<Value>),
    // Dict(HashMap<String, Value>),
    Command(AST),
}
