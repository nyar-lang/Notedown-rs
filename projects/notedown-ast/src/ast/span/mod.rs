use crate::AST;
use std::borrow::Cow;

mod display;
mod from;

pub enum Span<'a> {
    /// - `String`: Normal string with no style
    String(Cow<'a, str>),
    /// - `Bold`:
    Bold(Box<Span<'a>>),
    Italic(Box<Span<'a>>),
    /// - `Underline`:
    Underline(Box<Span<'a>>),
    Strikethrough(Box<Span<'a>>),
    Undercover(Box<Span<'a>>),
    MathInline(Cow<'a, str>),
    MathDisplay(Cow<'a, str>),
}