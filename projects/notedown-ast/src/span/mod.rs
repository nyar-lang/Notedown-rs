use crate::Block;

mod display;
mod from;

pub enum Span {
    /// - `String`: Normal string with no style
    String(String),
    /// - `Bold`:
    Bold(Box<Block>),
    Italic(Box<Block>),
    /// - `Underline`:
    Underline(Box<Block>),
    Strikethrough(Box<Block>),
    Undercover(Box<Block>),
}