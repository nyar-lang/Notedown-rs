


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
