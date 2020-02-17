use crate::AST;
use std::{
    fmt::format,
    iter::{repeat, Chain, Repeat},
    slice::Iter,
};

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
        match self {
            AST::None => String::from(""),
            AST::Space => String::from(" "),
            AST::Newline => String::from("\n"),

            AST::Statements(e) => {
                let mut text = String::new();
                for node in e {
                    text += &unbox!(node)
                }
                let trimmed: Vec<_> = text.lines().map(|s| s.trim()).collect();
                trimmed.join("\n")
            }

            AST::Header(e, level) => format!("{} {}", unbox!(e), level),

            AST::Paragraph(p) => format!("<p>{}</p>", unbox!(p)),

            AST::Text(v) => v.iter().map(|s| unbox!(s)).collect::<Vec<String>>().join(""),
            AST::Raw(s) => format!("<pre>{}</pre>", s),
            AST::Code(s) => format!("<code>{}</code>", s),
            AST::String(s) => format!("{}", s),
            AST::Bold(s) => format!("<b>{}</b>", unbox!(s)),
            AST::Italic(s) => format!("<i>{}</i>", unbox!(s)),
            AST::Underline(s) => format!("<u>{}</u>", unbox!(s)),
            AST::Strikethrough(s) => format!("<del>{}</del>", unbox!(s)),
            AST::Undercover(s) => format!("<span class=\"undercover\">{}</span>", unbox!(s)),

            AST::Font(e, kv) => {
                let mut tags = String::new();
                for (k, v) in kv.iter() {
                    tags += &format!(" {}=\"{}\"", k, v);
                }
                format!("<font{}>{}</font>", tags, unbox!(e))
            }
            AST::MathInline(s) => format!("<span class=\"math\">${}$</span> ", s),
            AST::MathDisplay(s) => format!("<p class=\"math\">$${}$$</span> ", s),

            AST::Table { head, align, terms, column } => {
                let align_iter = align.iter().chain(repeat(&align[align.len() - 1]));
                let thead = {
                    let mut head = head.iter().chain(repeat(&AST::Space));
                    let mut align = align_iter.clone();
                    let mut thead = String::new();
                    for _ in 0..*column {
                        let h = head.next().unwrap().to_html(cfg);
                        let a = *align.next().unwrap();
                        thead.push_str(&format!("{}", build_th(&h, a)))
                    }
                    format!("<thead>{}</thead>", thead)
                };
                let mut trs = vec![];
                for term in terms {
                    let mut head = term.iter().chain(repeat(&AST::Space));
                    let mut align = align_iter.clone();
                    let mut thead = String::new();
                    for _ in 0..*column {
                        let h = head.next().unwrap().to_html(cfg);
                        let a = *align.next().unwrap();
                        thead.push_str(&format!("{}", build_td(&h, a)))
                    }
                    trs.push(format!("<tr>{}</tr>", thead))
                }
                format!("<table>{}<tbody>{}</tbody></table>", thead, trs.join(""))
            }

            AST::Command(_s, _keys, _values) => unimplemented!(),
            _ => {
                let a = format!("HTML unimplemented AST::{:?}", self);
                println!("{}", a.split("(").next().unwrap_or("Unknown"));
                format!("{:?}", self);
                unreachable!()
            }
        }
    }
}

fn build_th(input: &str, e: u8) -> String {
    match e {
        1 => format!("<th align=\"left\">{}</th>", input),
        2 => format!("<th align=\"right\">{}</th>", input),
        3 => format!("<th align=\"center\">{}</th>", input),
        _ => format!("<th>{}</th>", input),
    }
}

fn build_td(input: &str, e: u8) -> String {
    match e {
        1 => format!("<td align=\"left\">{}</td>", input),
        2 => format!("<td align=\"right\">{}</td>", input),
        3 => format!("<td align=\"center\">{}</td>", input),
        _ => format!("<td>{}</td>", input),
    }
}
