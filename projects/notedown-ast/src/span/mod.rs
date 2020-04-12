use crate::Block;

mod display;
mod from;

pub enum Span<'a> {
    /// - `String`: Normal string with no style
    String(String),
    /// - `Bold`:
    Bold(Box<Block<'a>>),
    Italic(Box<Block<'a>>),
    /// - `Underline`:
    Underline(Box<Block<'a>>),
    Strikethrough(Box<Block<'a>>),
    Undercover(Box<Block<'a>>),
}