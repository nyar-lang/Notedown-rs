use markdown::{Block, Span};
use crate::AST;
use std::borrow::Cow;

impl<'a> From<Vec<Block>> for AST<'a> {
    fn from(v: Vec<Block>) -> Self {
        AST::Statements(v.into_iter().map(Into::into).collect())
    }
}

impl<'a> From<Block> for AST<'a> {
    fn from(v: Block) -> Self {
        match v {
            Block::Header(content, level) => AST::Header(box content.into(), level as u8),
            Block::Paragraph(_) => { unimplemented!() }
            Block::Blockquote(_) => { unimplemented!() }
            Block::CodeBlock(_, _) => { unimplemented!() }
            Block::OrderedList(_, _) => { unimplemented!() }
            Block::UnorderedList(_) => { unimplemented!() }
            Block::Raw(_) => { unimplemented!() }
            Block::Hr => { unimplemented!() }
        }
    }
}

impl<'a> From<Vec<Span>> for AST<'a> {
    fn from(v: Vec<Span>) -> Self {
        AST::Paragraph(v.into_iter().map(Into::into).collect())
    }
}

impl<'a> From<Span> for AST<'a> {
    fn from(v: Span) -> Self {
        match v {
            Span::Break => { unimplemented!() }
            Span::Text(text) => { AST::Text(Cow::from(text)) }
            Span::Code(_) => { unimplemented!() }
            Span::Link(_, _, _) => { unimplemented!() }
            Span::Image(_, _, _) => { unimplemented!() }
            Span::Emphasis(_) => { unimplemented!() }
            Span::Strong(_) => { unimplemented!() }
        }
    }
}


