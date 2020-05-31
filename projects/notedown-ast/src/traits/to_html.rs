use crate::{traits::ToHTML, AST};

impl ToHTML for Vec<AST> {
    fn to_html(&self) -> String {
        let s:Vec<_> = self.iter().map(ToHTML::to_html).collect();
        s.join("")
    }
}

impl ToHTML for AST {
    fn to_html(&self) -> String {
        match self {
            AST::None => {String::new()}
            AST::Statements(_) => {unimplemented!()}
            AST::Header { .. } => {unimplemented!()}
            AST::HorizontalRule { .. } => {unimplemented!()}
            AST::Paragraph { .. } => {unimplemented!()}
            AST::Highlight { .. } => {unimplemented!()}
            AST::MathBlock { .. } => {unimplemented!()}
            AST::TableView { .. } => {unimplemented!()}
            AST::QuoteList { .. } => {unimplemented!()}
            AST::OrderedList { .. } => {unimplemented!()}
            AST::OrderlessList { .. } => {unimplemented!()}
            AST::Normal { .. } => {unimplemented!()}
            AST::Raw { .. } => {unimplemented!()}
            AST::Code { .. } => {unimplemented!()}
            AST::Italic { .. } => {unimplemented!()}
            AST::Bold { .. } => {unimplemented!()}
            AST::Emphasis { .. } => {unimplemented!()}
            AST::Underline { .. } => {unimplemented!()}
            AST::Strikethrough { .. } => {unimplemented!()}
            AST::Undercover { .. } => {unimplemented!()}
            AST::MathInline { .. } => {unimplemented!()}
            AST::MathDisplay { .. } => {unimplemented!()}
            AST::Link { .. } => {unimplemented!()}
            AST::Escaped { .. } => {unimplemented!()}
            AST::Command { .. } => {unimplemented!()}
            AST::String { .. } => {unimplemented!()}
            AST::Integer { .. } => {unimplemented!()}
            AST::Decimal { .. } => {unimplemented!()}
            AST::Boolean { .. } => {unimplemented!()}
            AST::Array { .. } => {unimplemented!()}
        }
    }
}

/*
impl ToHTML for AST {
    fn to_html(&self) -> String {
        match self {
            AST::Statements(e) => {
                let mut text = String::new();
                for node in e {
                    text += &node.to_html()
                }
                let trimmed: Vec<_> = text.lines().map(|s| s.trim()).collect();
                trimmed.join("\n")
            }
            AST::Header { level, children:e,.. } => format!("<h{0}>{1}</h{0}>", level, e.to_html()),

            AST::Paragraph { children,.. } => format!("<p>{}</p>", children.to_html()),
            AST::Normal { s ,..} => s.to_owned(),
            AST::Strong { s,.. } => format!("<b>{}</b>", s.to_html()),
            AST::Emphasis { s,.. } => format!("<i>{}</i>", s.to_html()),
            AST::Underline { s,.. } => format!("<u>{}</u>", s.to_html()),
            AST::Strikethrough { s,.. } => format!("<del>{}</del>", s.to_html()),
            AST::Undercover { s,.. } => format!("<ast.span class=\"undercover\">{}</ast.span>", s.to_html()),
            /*
            AST::Table { head, align, terms, column } => {
                let align_iter = align.iter().chain(repeat(&align[align.len() - 1]));
                let thead = {
                    let mut head = head.iter().chain(repeat(&AST::Space));
                    let mut align = align_iter.clone();
                    let mut thead = String::new();
                    for _ in 0..*column {
                        let h = head.next().unwrap().to_html();
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
                        let h = head.next().unwrap().to_html();
                        let a = *align.next().unwrap();
                        thead.push_str(&format!("{}", build_td(&h, a)))
                    }
                    trs.push(format!("<tr>{}</tr>", thead))
                }
                format!("<table>{}<tbody>{}</tbody></table>", thead, trs.join(""))
            }
            AST::Quote { body, style } => {
                let quote = body.iter().map(|s| s.to_html()).collect::<Vec<String>>().join("");
                match style.as_str() {
                    "info" | "danger" | "warning" | "success" => {
                        format!("<blockquote class=\"fancyquote {}\">{}</blockquote>", style, quote)
                    }
                    _ => format!("<blockquote>{}</blockquote>", quote),
                }
            }
            AST::Ordered(v) => {
                let quote = v.iter().map(|s| format!("<li>{}</li>", s.to_html())).collect::<Vec<String>>().join("");
                format!("<ol>{}</ol>", quote)
            }
            AST::Orderless(v) => {
                let quote = v.iter().map(|s| format!("<li>{}</li>", s.to_html())).collect::<Vec<String>>().join("");
                format!("<ul>{}</ul>", quote)
            }

            AST::Command(s, keys, values) => {
                let ref cfg = GLOBAL_CONFIG.lock().unwrap();
                format!("cmd: {}\narg: {:?}\nkvs: {:?}", s, keys, values)
            }
            */
            AST::None => String::from(""),
            AST::Newline => String::from("</br>"),

            AST::Raw(s) => format!("<pre>{}</pre>", s),
            AST::Code(s) => format!("<code>{}</code>", s),

            //#[cfg(feature = "default")]
            AST::MathInline(s) => format!("<span class=\"math\">${}$</span> ", s),
            // #[cfg(feature = "desktop")]
            // AST::MathInline(s) => utils::rex_math_inline(s),
            //#[cfg(feature = "default")]
            AST::MathDisplay(s) => format!("<p class=\"math\">$${}$$</p> ", s),
            // #[cfg(feature = "desktop")]
            // AST::MathDisplay(s) => utils::rex_math_display(s),
            _ => {
                let a = format!("HTML unimplemented AST::{:?}", self);
                println!("{}", a.split("(").next().unwrap_or("Unknown"));
                format!("{:?}", self);
                unreachable!()
            }
        }
    }
}
*/