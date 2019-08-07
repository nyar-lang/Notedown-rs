use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};
use {HTMLConfig, ToHTML};

#[derive(Debug, Clone)]
pub enum AST {
    /// - `Header`
    Header(Box<AST>, HashMap<String, String>),

    /// - `String`: Normal string with no style
    String(String),
    /// - `Bold`:
    Bold(Box<AST>),
    /// - `Italic`:
    Italic(Box<AST>),
    /// - `Underline`:
    Underline(Box<AST>),
    /// - `Font`:
    Font(Box<AST>, HashMap<String, String>),
    /// - `Math`:
    Math(String, HashMap<String, String>),
    /// - `Code`:
    Code(String, HashMap<String, String>),

    /// - `Node`: For unknown structural
    Node(Box<AST>),
    /// - `Function`:
    Function(String, Vec<AST>, HashMap<String, String>),
}

impl Display for AST {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            AST::Header(ref e, ref kv) => write!(f, "{} {:?}", e, kv),
            AST::String(ref s) => write!(f, "{}", s),
            AST::Bold(ref e) => write!(f, "{}", e),
            AST::Italic(ref e) => write!(f, "{}", e),
            AST::Font(ref e, ref kv) => write!(f, "{} {:?}", e, kv),
            AST::Underline(ref e) => write!(f, "{}", e),
            _ => write!(f, "unknown"),
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
            AST::Header(ref e, ref kv) =>{
                format!("{} {:?}", unbox!(e), kv)
            }
            AST::String(ref s) => format!("{}", s),
            AST::Bold(ref e) => format!("<b>{}</b>", unbox!(e)),
            AST::Italic(ref e) => format!("<i>{}</i>", unbox!(e)),
            AST::Font(ref e, ref kv) => {
                let mut tags = String::new();
                for (k, v) in kv.iter() {
                    tags += &format!(" {}=\"{}\"", k, v);
                }
                format!("<font{}>{}</font>", tags, unbox!(e))
            }
            AST::Underline(ref e) => format!("<u>{}</u>", unbox!(e)),
            _ => format!(""),
        }
    }
}
