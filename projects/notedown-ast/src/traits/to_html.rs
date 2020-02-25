use crate::{traits::Settings, utils::build_zola, Context, NotedownTarget, NotedownTemplate, Value, AST};
use std::{collections::HashMap, iter::repeat};

pub trait ToHTML {
    fn to_html_with(&self, cfg: &Settings) -> String;
    fn to_html(&self) -> String {
        self.to_html_with(&Settings::default())
    }
}

impl ToHTML for Context {
    fn to_html_with(&self, cfg: &Settings) -> String {
        let head = self.meta.to_html_with(cfg);
        let post = self.ast.to_html_with(cfg);
        return format!("{}{}", head, post);
    }

    fn to_html(&self) -> String {
        let head = self.meta.to_html_with(&self.cfg);
        let post = self.ast.to_html_with(&self.cfg);
        return format!("{}{}", head, post);
    }
}

impl ToHTML for HashMap<String, Value> {
    fn to_html_with(&self, cfg: &Settings) -> String {
        match cfg.target {
            NotedownTarget::Web => String::new(),
            NotedownTarget::VSCode => String::new(),
            NotedownTarget::Zola => String::new(),
        }
    }
}

impl ToHTML for AST {
    fn to_html_with(&self, cfg: &Settings) -> String {
        match self {
            AST::Statements(e) => {
                let mut text = String::new();
                for node in e {
                    text += &node.to_html_with(cfg)
                }
                let trimmed: Vec<_> = text.lines().map(|s| s.trim()).collect();
                trimmed.join("\n")
            }
            AST::Header(e, level) => format!("<h{0}>{1}</h{0}>", level, e.to_html_with(cfg)),

            AST::Paragraph(p) => format!("<p>{}</p>", p.to_html_with(cfg)),
            AST::Text(v) => v.iter().map(|s| s.to_html_with(cfg)).collect::<Vec<String>>().join(""),
            AST::Bold(s) => format!("<b>{}</b>", s.to_html_with(cfg)),
            AST::Italic(s) => format!("<i>{}</i>", s.to_html_with(cfg)),
            AST::Underline(s) => format!("<u>{}</u>", s.to_html_with(cfg)),
            AST::Strikethrough(s) => format!("<del>{}</del>", s.to_html_with(cfg)),
            AST::Undercover(s) => format!("<span class=\"undercover\">{}</span>", s.to_html_with(cfg)),

            AST::Table { head, align, terms, column } => {
                let align_iter = align.iter().chain(repeat(&align[align.len() - 1]));
                let thead = {
                    let mut head = head.iter().chain(repeat(&AST::Space));
                    let mut align = align_iter.clone();
                    let mut thead = String::new();
                    for _ in 0..*column {
                        let h = head.next().unwrap().to_html_with(cfg);
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
                        let h = head.next().unwrap().to_html_with(cfg);
                        let a = *align.next().unwrap();
                        thead.push_str(&format!("{}", build_td(&h, a)))
                    }
                    trs.push(format!("<tr>{}</tr>", thead))
                }
                format!("<table>{}<tbody>{}</tbody></table>", thead, trs.join(""))
            }
            AST::Quote { body, style } => {
                let quote = body.iter().map(|s| s.to_html_with(cfg)).collect::<Vec<String>>().join("");
                match style.as_str() {
                    "info" | "danger" | "warning" | "success" => {
                        format!("<blockquote class=\"fancyquote {}\">{}</blockquote>", style, quote)
                    }
                    _ => format!("<blockquote>{}</blockquote>", quote),
                }
            }
            AST::Ordered(v) => {
                let quote = v.iter().map(|s| format!("<li>{}</li>", s.to_html_with(cfg))).collect::<Vec<String>>().join("");
                format!("<ol>{}</ol>", quote)
            }
            AST::Orderless(v) => {
                let quote = v.iter().map(|s| format!("<li>{}</li>", s.to_html_with(cfg))).collect::<Vec<String>>().join("");
                format!("<ul>{}</ul>", quote)
            }

            AST::Command(s, keys, values) => format!("cmd: {}\narg: {:?}\nkvs: {:?}", s, keys, values),
            _ => self.to_html()
        }
    }

    fn to_html(&self) -> String {
        match self {
            AST::None => String::from(""),
            AST::Space => String::from(" "),
            AST::Newline => String::from("</br>"),

            AST::Raw(s) => format!("<pre>{}</pre>", s),
            AST::Code(s) => format!("<code>{}</code>", s),
            AST::String(s) => format!("{}", s),
            AST::MathInline(s) => format!("<span class=\"math\">${}$</span> ", s),
            AST::MathDisplay(s) => format!("<p class=\"math\">$${}$$</p> ", s),

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
