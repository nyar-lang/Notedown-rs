use crate::AST;

#[derive(Debug, Copy, Clone)]
pub struct HTMLConfig {}

impl Default for HTMLConfig {
    fn default() -> Self {
        HTMLConfig {}
    }
}

pub trait ToHTML {
    fn to_html(&self, cfg: HTMLConfig) -> String;
    fn to_html_default(&self) -> String {
        self.to_html(HTMLConfig::default())
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
            AST::None => String::from(""),
            AST::Space => String::from(" "),
            AST::Newline => String::from("\n"),

            AST::Statements(ref e) => {
                let mut text = String::new();
                for node in e {
                    text += &unbox!(node)
                }
                let trimmed: Vec<_> = text.lines().map(|s| s.trim()).collect();
                trimmed.join("\n")
            }

            AST::Header(ref e, ref level) => format!("{} {}", unbox!(e), level),

            AST::Paragraph(ref p) => format!("<p>{}</p>", unbox!(p)),

            AST::Text(ref v) => v
                .iter()
                .map(|s| unbox!(s))
                .collect::<Vec<String>>()
                .join(""),
            AST::Raw(ref s) => format!("<pre>{}</pre>`", s),
            AST::Code(ref s) => format!("<code>{}</code>`", s),
            AST::String(ref s) => format!("{}", s),
            AST::Bold(ref s) => format!("<b>{}</b>", unbox!(s)),
            AST::Italic(ref s) => format!("<i>{}</i>", unbox!(s)),
            AST::Underline(ref s) => format!("<u>{}</u>", unbox!(s)),
            AST::Strikethrough(ref s) => format!("<del>{}</del>", unbox!(s)),
            AST::Undercover(ref s) => format!("<span class=\"undercover\">{}</span>", unbox!(s)),

            AST::Font(ref e, ref kv) => {
                let mut tags = String::new();
                for (k, v) in kv.iter() {
                    tags += &format!(" {}=\"{}\"", k, v);
                }
                format!("<font{}>{}</font>", tags, unbox!(e))
            }
            AST::MathInline(ref s) => format!("<span class=\"math\">${}$</span> ", s),
            AST::MathDisplay(ref s) => format!("<p class=\"math\">$${}$$</span> ", s),

            AST::Command(ref s, ref keys, ref values) => {
                unimplemented!()
            }
            _ => {
                let a = format!("HTML unimplemented AST::{:?}", self);
                println!("{}", a.split("(").next().unwrap_or("Unknown"));
                format!("{:?}", self);
                unreachable!()
            }
        }
    }
}
