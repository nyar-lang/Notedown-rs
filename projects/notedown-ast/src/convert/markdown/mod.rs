use markdown::{Block, Span, ListItem, tokenize};
use crate::{AST, Command, Value, ListView, SmartLink, CommandKind};
use std::collections::HashMap;


pub fn markdown_parse<'a>(input: &str) -> AST<'a> {
    AST::from(tokenize(input))
}

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
            Block::CodeBlock(lang, code) => {
                let mut kvs: HashMap<&str, Value> = Default::default();
                kvs.insert("body", code.into());
                let lang = match lang {
                    None => "txt",
                    // lang tokens created from the String would be available across all threads
                    Some(s) => Box::leak(s.into_boxed_str())
                };
                let code = Command {
                    cmd: lang,
                    args: vec![],
                    kvs,
                    kind: CommandKind::SmartLink,
                };
                AST::Command(code)
            }
            Block::Blockquote(q) => {
                let list = ListView::Quote {
                    style: None,
                    body: q.into_iter().map(Into::into).collect(),
                };
                AST::List(list)
            }
            Block::OrderedList(o, _) => {
                let list = ListView::Ordered {
                    head: 1,
                    body: o.into_iter().map(Into::into).collect(),
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
            Span::Link(text, url, None) => {
                let link = SmartLink::Hyperlinks(text.into(), Some(url.into()));
                AST::Link(link)
            }
            Span::Link(text, url, Some(title)) => {
                let mut kvs: HashMap<&str, Value> = Default::default();
                kvs.insert("title", title.into());
                kvs.insert("href", url.into());
                kvs.insert("body", text.into());
                let link = Command {
                    cmd: "a",
                    args: vec![],
                    kvs,
                    kind: CommandKind::SmartLink,
                };
                AST::Command(link)
            }
            Span::Image(a, b, None) => {
                let link = SmartLink::Image(a.into(), Some(b.into()));
                AST::Link(link)
            }
            Span::Image(text, url, Some(title)) => {
                let link = SmartLink::Image(a.into(), Some(b.into()));
                AST::Link(link)
            }
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
