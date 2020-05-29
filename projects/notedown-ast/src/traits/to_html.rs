use crate::AST;
use crate::traits::ToHTML;

impl ToHTML for AST {
    fn to_html(&self) -> String {
        unimplemented!()
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
            AST::Header(e, level) => format!("<h{0}>{1}</h{0}>", level, e.to_html()),

            AST::Paragraph(p) => format!("<p>{}</p>", p.to_html()),
            AST::Normal(s) => s.to_owned(),
            AST::Strong(s) => format!("<b>{}</b>", s.to_html()),
            AST::Emphasis(s) => format!("<i>{}</i>", s.to_html()),
            AST::Underline(s) => format!("<u>{}</u>", s.to_html()),
            AST::Strikethrough(s) => format!("<del>{}</del>", s.to_html()),
            AST::Undercover(s) => format!("<ast.span class=\"undercover\">{}</ast.span>", s.to_html()),
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

impl ToHTML for Vec<AST> {
    fn to_html(&self) -> String {
        let v: Vec<_> = self.iter().map(ToHTML::to_html).collect();
        v.join("")
    }
}
*/