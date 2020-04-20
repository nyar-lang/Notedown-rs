use markdown::{Block, Span, ListItem};
use crate::{AST, Command, Value, ListView, SmartLink, CommandKind};
use std::collections::HashMap;

impl<'a> From<Vec<Block>> for AST<'a> {
    fn from(v: Vec<Block>) -> Self {
        AST::Statements(v.into_iter().map(Into::into).collect())
    }
}

impl<'a> From<Block> for AST<'a> {
    fn from(v: Block) -> Self {
        match v {
            Block::Header(content, level) => AST::Header(box content.into(), level as u8),
            Block::Paragraph(p) => p.into(),
            Block::Blockquote(_) => { unimplemented!() }
            Block::CodeBlock(_, _) => { unimplemented!() }
            Block::OrderedList(o, _) => {
                let list = ListView::Ordered {
                    head: 1,
                    body: o.into_iter().map(Into::into).collect()
                };
                AST::List(list)
            }
            Block::UnorderedList(u) => {
                let list = ListView::Orderless {
                    body: u.into_iter().map(Into::into).collect()
                };
                AST::List(list)
            }
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
            Span::Text(text) => { AST::Text(text.into()) }
            Span::Code(c) => { AST::Raw(c.into()) }
            Span::Link(a, b, None) => {
                let link = SmartLink::Hyperlinks(a.into(), Some(b.into()));
                AST::Link(link)
            },
            Span::Link(a, b, Some(s)) => {
                let mut kvs: HashMap<&str, Value> = Default::default();
                kvs.insert("title", s.into());
                let link = Command {
                    cmd: "a",
                    args: vec![],
                    kvs,
                    kind: CommandKind::SmartLink
                };
                AST::Command(link)
            }
            Span::Image(a, b, None) => { unimplemented!() }
            Span::Image(a, b, Some(s)) => { unimplemented!() }
            Span::Emphasis(e) => { AST::Emphasis(e.into_iter().map(Into::into).collect()) }
            Span::Strong(s) => { AST::Strong(s.into_iter().map(Into::into).collect()) }
        }
    }
}


impl<'a> From<ListItem> for AST<'a> {
    fn from(v: ListItem) -> Self {
        match v {
            ListItem::Simple(s) => s.into(),
            ListItem::Paragraph(p) => p.into()
        }
    }
}
