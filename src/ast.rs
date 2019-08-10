use crate::{HTMLConfig, ToHTML};
use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub enum AST {
    /// - `None`: It doesn't look like anything to me
    None,
    /// - ``
    Statements(Vec<AST>),

    /// - `Header`: TEXT, level, args
    Header(Box<AST>, u8, HashMap<String, String>),

    /// - `String`: Normal string with no style
    String(String),
    /// - `Bold`:
    Bold(Box<AST>, u8),
    /// - `Italic`:
    Italic(Box<AST>, u8),
    /// - `Underline`:
    Underline(Box<AST>, u8),
    /// - `Font`:
    Font(Box<AST>, HashMap<String, String>),

    /// - `Math`:
    MathInline(String),
    MathDisplay(String),
    /// - `Code`:
    Code(String, HashMap<String, String>),

    /// - `Text`: For inline style
    Text(Box<AST>),
    /// -
    Word(String),
    Escaped(String),
    Newline,
    ///  - `Paragraph`:
    Paragraph(Box<AST>),
    /// - `Function`: input, args, kvs
    Function(String, Vec<AST>, HashMap<String, String>),
}

impl Display for AST {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            AST::Statements(ref e) => {
                let fs: Vec<String> = e.iter().map(|ast| format!("{}", ast)).collect();
                write!(f, "{}", fs.join(""))
            }
            AST::Paragraph(ref e) => {
                let fs: Vec<String> = format!("{}", e)
                    .lines()
                    .map(|k| k.trim_end().to_string())
                    .collect();
                write!(f, "{}\n\n", fs.join("\n"))
            }
            AST::Newline => write!(f, "\n"),

            AST::Word(ref s) => write!(f, "{} ", s),

            AST::MathInline(ref s) => write!(f, "${}$ ", s),

            _ => {
                let a = format!("unimplemented AST::{:?}", self);
                println!("{}", a.split("(").next().unwrap_or("Unknown"));
                write!(f, "UnimplementedError")
            }
        }
    }
}

impl From<&str> for AST {
    fn from(s: &str) -> Self {
        AST::String(s.to_string())
    }
}

/// Unwrap Box<AST>
impl ToHTML for Box<AST> {
    fn to_html(&self, cfg: HTMLConfig) -> String {
        let unbox = self.as_ref();
        unbox.to_html(cfg)
    }
}

impl ToHTML for AST {
    fn to_html(&self, cfg: HTMLConfig) -> String {
        macro_rules! unbox {
            ($e:ident) => {
                $e.to_html(cfg)
            };
        }
        match *self {
            AST::Newline => {
                //CR or LF
                "\n".to_string()
            }

            AST::Statements(ref e) => {
                let mut text = String::new();
                for node in e {
                    text += &unbox!(node)
                }
                let trimmed: Vec<_> = text.lines().map(|s| s.trim()).collect();
                trimmed.join("\n")
            }
            AST::Paragraph(ref e) => format!("<p>{}</p>", unbox!(e)),

            AST::Header(ref e, ref level, ref kv) => format!("{} {}{:?}", unbox!(e), level, kv),
            AST::String(ref s) => format!("{}", s),
            AST::Bold(ref e, _) => format!("<b>{}</b>", unbox!(e)),
            AST::Italic(ref e, _) => format!("<i>{}</i>", unbox!(e)),
            AST::Underline(ref e, _) => format!("<u>{}</u>", unbox!(e)),
            AST::Font(ref e, ref kv) => {
                let mut tags = String::new();
                for (k, v) in kv.iter() {
                    tags += &format!(" {}=\"{}\"", k, v);
                }
                format!("<font{}>{}</font>", tags, unbox!(e))
            }

            AST::Word(ref s) => format!("{} ", s),
            AST::MathInline(ref s) => format!("${}$ ", s),
            _ => {
                let a = format!("HTML unimplemented AST::{:?}", self);
                println!("{}", a.split("(").next().unwrap_or("Unknown"));
                format!("{:?}", self)
            }
        }
    }
}
