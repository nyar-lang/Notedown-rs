use markdown::{Block, Span, ListItem, tokenize};
use crate::{AST, ListView, SmartLink, Highlighter};


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
            Block::Header(content, level) => AST::Header(vec![content.into()], level),
            Block::Paragraph(p) => p.into(),
            Block::CodeBlock(lang, code) => {
                let lang = match lang {
                    None => "txt",
                    Some(s) => match s.as_ref() {
                        "" => "txt",
                        // lang tokens created from the String would be available across all threads
                        _ => Box::leak(s.into_boxed_str())
                    },
                };
                let code = Highlighter {
                    lang,
                    code: code.into(),
                    inline: false,
                    high_line: vec![],
                };
                AST::Highlight(code)
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
            Span::Code(c) => { AST::Code(c.into()) }
            Span::Link(text, url, title) => {
                let link = SmartLink::Hyperlinks {
                    from: text.into(),
                    to: Some(url.into()),
                    alt: title.map(Into::into),
                    bind: None,
                };
                AST::Link(link)
            }
            Span::Image(_, src, title) => {
                let link = SmartLink::Hyperlinks {
                    from: src.into(),
                    to: None,
                    alt: title.map(Into::into),
                    bind: None,
                };
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
