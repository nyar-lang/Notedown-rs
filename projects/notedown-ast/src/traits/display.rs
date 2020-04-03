use crate::{Span, AST, Value};
use std::fmt::{Display, Formatter, Debug};
use std::fmt;

impl Display for AST {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            AST::None => write!(f, ""),
            AST::Space => write!(f, " "),
            AST::Newline => write!(f, "\n"),

            AST::Header(a, l) => write!(f, "{} {}", "#".repeat(*l as usize), a),

            AST::Statements(e) => {
                let fs: Vec<String> = e.iter().map(|ast| format!("{}", ast)).collect();
                write!(f, "{}", fs.join(""))
            }

            AST::Paragraph(t) => write!(f, "{}\n", t),

            AST::Text(t) => {
                let fs: Vec<String> = t.iter().map(|k| format!("{}", k)).collect();
                write!(f, "{}", fs.join(""))
            }
            AST::Raw(s) => write!(f, "{}", s),
            AST::Code(s) => write!(f, "`{}`", s),
            AST::String(s) => write!(f, "{}", s),
            AST::Italic(s) => write!(f, "*{}*", s),
            AST::Bold(s) => write!(f, "**{}**", s),
            AST::Underline(s) => write!(f, "~{}~", s),
            AST::Strikethrough(s) => write!(f, "~~{}~~", s),
            AST::Undercover(s) => write!(f, "~~~{}~~~", s),

            AST::MathInline(s) => write!(f, "${}$", s),
            AST::MathDisplay(s) => write!(f, "$${}$$", s),

            AST::Quote { body, .. } => {
                let s: Vec<_> = body.iter().map(|a| format!("> {}", a)).collect();
                write!(f, "{}", s.join("\n"))
            }
            AST::Orderless(v) => {
                let s: Vec<_> = v.iter().map(|a| format!("- {}", a)).collect();
                write!(f, "{}", s.join("\n"))
            }
            AST::Command(s, args, kvs) => {
                let a: Vec<String> = args.iter().map(|v| format!("{}", v)).collect();
                let kv: Vec<String> = kvs.iter().map(|(k, v)| format!("{} = {}", k, v)).collect();

                write!(f, "\\{}{{{}}}", s, [&a[..], &kv[..]].concat().join(", "))
            }
            _ => {
                let a = format!("unimplemented AST::{:?}", self);
                write!(f, "{}", a.split("(").next().unwrap_or("Unknown"))
            }
        }
    }
}

impl Display for Span {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Span::String(s) => write!(f, "{}", s),
            Span::Bold(s) => {unimplemented!()}
            Span::Italic(s) => write!(f, "*{}*", s),
            Span::Underline(s) => {unimplemented!()}
            Span::Strikethrough(s) => {unimplemented!()}
            Span::Undercover(s) => {unimplemented!()}
        }
    }
}

impl Debug for Value {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Value::None => write!(f, "none"),
            Value::String(s) => write!(f, "{:?}", s),
            // Value::Integer(s) => write!(f, "{}", s),
            // Value::Decimal(s) => write!(f, "{}", s),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::List(_) => unimplemented!(),
            // Value::Dict(_) => unimplemented!(),
            Value::Command(node) => write!(f, "{}", node),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Value::None => write!(f, ""),
            _ => write!(f, "{:?}", self),
        }
    }
}