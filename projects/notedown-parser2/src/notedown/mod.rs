pub mod parser;
mod value;
pub type MarkdownText = Vec<NotedownInline>;

#[derive(Clone, Debug, PartialEq)]
pub enum Markdown {
    Heading(usize, MarkdownText),
    OrderedList(Vec<MarkdownText>),
    UnorderedList(Vec<MarkdownText>),
    Line(MarkdownText),
    Codeblock(String),
}

#[derive(Clone, Debug, PartialEq)]
pub enum NotedownInline {
    Link(String, String),
    Image(String, String),
    InlineCode(String),
    Bold(String),
    Italic(String),
    Plaintext(String),
}
