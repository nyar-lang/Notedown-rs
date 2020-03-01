use crate::{
    utils::{map_escape, map_white_space, maybe_math, str_escape, trim_dedent, unescape},
    Context, Value, AST,
};
use notedown_parser::{NoteDownParser, NoteDownRule as Rule};
use pest::{
    iterators::{Pair, Pairs},
    Parser,
};
use std::collections::{HashMap, VecDeque};

macro_rules! debug_cases {
    ($i:ident) => {{
        println!("Rule::{:?}=>continue,", $i.as_rule());
        println!("Span: {:?}", $i.as_span());
        println!("Text: {}", $i.as_str());
        unreachable!();
    }};
}

impl Context {}

impl Context {
    pub fn parse(&mut self, text: &str) {
        let input = text.replace("\t", &" ".repeat(self.cfg.tab_size)).replace("\n\r", "\n");
        self.ast = self.parse_program(&input)
    }
    pub fn parse_program(&mut self, text: &str) -> AST {
        let pairs = NoteDownParser::parse(Rule::program, text).unwrap_or_else(|e| panic!("{}", e));
        let mut codes = vec![];
        for pair in pairs {
            let code = match pair.as_rule() {
                Rule::EOI => continue,
                Rule::COMMENT => continue,
                Rule::NEWLINE => continue,
                Rule::SPACE_SEPARATOR => continue,
                Rule::HorizontalRule => AST::from("<hr/>"),
                Rule::TextBlock => maybe_math(self, pair),
                Rule::Header => self.parse_header(pair),
                Rule::List => self.parse_list(pair.as_str().trim_end()),
                Rule::Table => self.parse_table(pair.as_str().trim()),
                Rule::Code => self.parse_code(pair),
                Rule::CommandBlock => {
                    let cmd = self.parse_command(pair);
                    self.execute_cmd(cmd)
                }
                Rule::CommandLine => {
                    let cmd = self.parse_command(pair);
                    self.execute_cmd(cmd)
                }
                _ => debug_cases!(pair),
            };
            // println!("{:?}", code);
            codes.push(code);
        }
        AST::Statements(codes)
    }
    fn parse_header(&mut self, pairs: Pair<Rule>) -> AST {
        let mut level = 0;
        let mut head = AST::None;
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::SPACE_SEPARATOR => continue,
                Rule::Sharp => level += 1,
                Rule::RestOfLine => head = self.parse_text(pair.as_str().trim()),
                _ => debug_cases!(pair),
            };
        }
        return AST::Header(Box::new(head), level);
    }
    fn parse_code(&self, pairs: Pair<Rule>) -> AST {
        let mut level = "";
        let mut cmd = "";
        let mut body = "";
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::SPACE_SEPARATOR => continue,
                Rule::CodeMark => continue,
                Rule::CodeLevel => level = pair.as_str(),
                Rule::SYMBOL => cmd = pair.as_str(),
                Rule::CodeText => body = pair.as_str().trim_matches('\n'),
                _ => debug_cases!(pair),
            };
        }
        let code = format!("\n\n{0}{1}{2}{0}\n\n", level, cmd, body);
        return AST::String(code);
    }
    pub fn parse_text(&mut self, text: &str) -> AST {
        let pairs = NoteDownParser::parse(Rule::TextMode, text).unwrap_or_else(|e| panic!("{}", e));
        let mut codes = vec![];
        for pair in pairs {
            let code = match pair.as_rule() {
                Rule::EOI => continue,
                Rule::NEWLINE => AST::Newline,
                Rule::SPACE_SEPARATOR => map_white_space(pair.as_str()),
                Rule::Escaped => map_escape(pair.as_str()),

                Rule::Style => self.parse_style(pair),
                Rule::Line => self.parse_line(pair),
                Rule::Raw => self.parse_code_inline(pair),
                Rule::Math => self.parse_math(pair),

                Rule::TextRest => AST::String(pair.as_str().to_string()),
                Rule::RawRest => AST::String(pair.as_str().to_string()),
                Rule::StyleRest => AST::String(pair.as_str().to_string()),
                Rule::MathRest => AST::String(pair.as_str().to_string()),
                Rule::CommandLine => {
                    let cmd = self.parse_command(pair);
                    self.execute_cmd(cmd)
                }
                Rule::CommandBlock => {
                    let cmd = self.parse_command(pair);
                    self.execute_cmd(cmd)
                }
                _ => debug_cases!(pair),
            };
            codes.push(code);
        }
        if codes.len() == 0 { AST::None } else { AST::Text(codes) }
    }
    fn parse_style(&mut self, pairs: Pair<Rule>) -> AST {
        let s = pairs.as_str();
        let mut level = 0;
        let mut text = AST::None;
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::Asterisk => continue,
                Rule::StyleLevel => level += pair.as_str().len(),
                Rule::StyleText => text = self.parse_text(pair.as_str().trim()),
                _ => debug_cases!(pair),
            };
        }
        match level {
            1 => AST::Italic(Box::new(text)),
            2 => AST::Bold(Box::new(text)),
            3 => AST::Bold(Box::new(AST::Italic(Box::new(text)))),
            _ => AST::Raw(s.to_string()),
        }
    }
    fn parse_line(&mut self, pairs: Pair<Rule>) -> AST {
        let s = pairs.as_str();
        let mut level = 0;
        let mut text = AST::None;
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::Tilde => continue,
                Rule::LineLevel => level += pair.as_str().len(),
                Rule::LineText => text = self.parse_text(pair.as_str().trim()),

                _ => debug_cases!(pair),
            };
        }
        match level {
            1 => AST::Underline(Box::new(text)),
            2 => AST::Strikethrough(Box::new(text)),
            3 => AST::Undercover(Box::new(text)),
            _ => AST::Raw(s.to_string()),
        }
    }
    fn parse_code_inline(&self, pairs: Pair<Rule>) -> AST {
        let mut text = "";
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::RawLevel => continue,
                Rule::Accent => continue,
                Rule::RawText => text = pair.as_str().trim(),
                _ => debug_cases!(pair),
            };
        }
        AST::Code(text.to_string())
    }
    fn parse_math(&self, pairs: Pair<Rule>) -> AST {
        let s = pairs.as_str();
        let mut level = 0;
        let mut text = "";
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::Dollar => continue,
                Rule::MathLevel => level += pair.as_str().len(),
                Rule::MathText => text = pair.as_str().trim(),
                _ => debug_cases!(pair),
            };
        }
        match level {
            1 => AST::MathInline(text.to_string()),
            2 => AST::MathInline(format!("\\displaystyle{{{}}}", text)),
            _ => AST::Raw(s.to_string()),
        }
    }
    fn parse_command(&self, pairs: Pair<Rule>) -> AST {
        let mut cmd = String::default();
        let mut arg = Vec::default();
        let mut kvs = HashMap::default();
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::Colon => continue,
                Rule::command => cmd = pair.as_str().trim_start_matches('\\').to_string(),
                Rule::argument_literal => arg.push(Value::String(unescape(pair.as_str(), "]"))),
                Rule::argument => {
                    let mut v = Value::None;
                    for inner in pair.into_inner() {
                        match inner.as_rule() {
                            Rule::SPACE_SEPARATOR => continue,
                            Rule::Comma => continue,
                            Rule::value => v = self.parse_value(inner),
                            _ => debug_cases!(inner),
                        };
                    }
                    arg.push(v);
                }
                Rule::key_value => {
                    let mut k = String::new();
                    let mut v = Value::None;
                    for inner in pair.into_inner() {
                        match inner.as_rule() {
                            Rule::SPACE_SEPARATOR => continue,
                            Rule::Comma | Rule::Set => continue,
                            Rule::key => k = str_escape(inner.as_str()),
                            Rule::value => v = self.parse_value(inner),
                            _ => debug_cases!(inner),
                        };
                    }
                    kvs.insert(k, v);
                }
                Rule::RestOfLine => arg.push(Value::from(pair.as_str())),
                _ => debug_cases!(pair),
            };
        }
        return AST::Command(cmd, arg, kvs);
    }
    fn parse_value(&self, pairs: Pair<Rule>) -> Value {
        let mut value = Value::None;
        for pair in pairs.into_inner() {
            value = match pair.as_rule() {
                Rule::String => Value::String(str_escape(pair.as_str())),
                Rule::Integer => Value::String(pair.as_str().to_string()),
                Rule::Keywords => match pair.as_str() {
                    "null" => Value::None,
                    "true" => Value::Boolean(true),
                    "false" => Value::Boolean(false),
                    _ => unreachable!(),
                },
                Rule::SYMBOL => {
                    let s = pair.as_str().to_string();
                    Value::Command(AST::Command(s, vec![], Default::default()))
                }
                _ => debug_cases!(pair),
            };
        }
        return value;
    }
    fn parse_table(&mut self, text: &str) -> AST {
        let mut lines: VecDeque<String> = VecDeque::new();
        for i in text.lines() {
            let mut l = String::from(i.trim());
            if i.starts_with('|') {
                l = (&l[1..]).to_string()
            }
            if !i.ends_with('|') || i.ends_with("\\|") {
                l.push('|')
            }
            lines.push_back(l)
        }
        if lines.len() < 3 {
            return AST::Paragraph(Box::new(AST::Raw(text.to_string())));
        }
        let head = self.parse_table_line(&lines.pop_front().unwrap());
        let align = parse_table_align(&lines.pop_front().unwrap());
        let mut terms = vec![];
        for i in lines {
            terms.push(self.parse_table_line(&i));
        }
        let mut column = vec![head.len(), align.len()];
        column.extend(terms.iter().map(Vec::len).collect::<Vec<usize>>());
        let column = *column.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
        return AST::Table { head, align, terms, column };
    }
    fn parse_table_line(&mut self, input: &str) -> Vec<AST> {
        let pairs = NoteDownParser::parse(Rule::TableMode, input).unwrap_or_else(|e| panic!("{}", e));
        let mut codes = vec![];
        let mut text = String::new();
        for pair in pairs {
            match pair.as_rule() {
                Rule::EOI => continue,
                Rule::SPACE_SEPARATOR => text.push(' '),
                Rule::TableMark => {
                    codes.push(self.parse_text(&text));
                    text = String::new();
                }
                _ => text.push_str(pair.as_str().trim()),
            };
        }
        return codes;
    }
    fn parse_list(&mut self, text: &str) -> AST {
        let (n, ty) = List::get_type(text.lines().next().unwrap());
        let mut codes: Vec<String> = vec![];
        let mut code: Vec<String> = vec![];
        for line in trim_dedent(text, n).lines() {
            let (b, t) = List::trim_indent(line, n, &ty);
            if b && code.len() != 0 {
                codes.push(code.join("\n"));
                code = vec![t]
            }
            else {
                code.push(t);
            }
        }
        codes.push(code.join("\n"));
        let ast: Vec<_> = codes.iter().map(|c| self.parse_program(&c)).collect();
        match ty {
            List::Quote => AST::Quote { body: ast, style: String::new() },
            List::Ordered => AST::Ordered(ast),
            List::Orderless => AST::Orderless(ast),
        }
    }
}

fn parse_table_align(input: &str) -> Vec<u8> {
    let pairs = NoteDownParser::parse(Rule::TableMode, input).unwrap_or_else(|e| panic!("{}", e));
    let mut codes = vec![];
    let mut text = String::new();
    for pair in pairs {
        match pair.as_rule() {
            Rule::EOI => continue,
            Rule::SPACE_SEPARATOR => text.push(' '),
            Rule::TableRest => text.push_str(pair.as_str()),
            Rule::TableMark => {
                let mut code = 0;
                if text.contains(":-") {
                    code += 1 << 0
                }
                if text.contains("-:") {
                    code += 1 << 1
                }
                codes.push(code);
                text = String::new();
            }
            _ => debug_cases!(pair),
        };
    }
    return codes;
}

#[derive(Debug)]
enum List {
    Quote,
    Ordered,
    Orderless,
}

impl List {
    pub fn get_type(input: &str) -> (usize, List) {
        let pairs = List::parse_pairs(input);
        let mut i = 0;
        let mut m = List::Quote;
        for pair in pairs {
            match pair.as_rule() {
                Rule::SPACE_SEPARATOR => i += 1,
                Rule::ListMark => match pair.as_str() {
                    ">" => m = List::Quote,
                    "-" => m = List::Orderless,
                    _ => m = List::Ordered,
                },
                _ => return (i, m),
            };
        }
        return (i, m);
    }
    pub fn trim_indent(line: &str, _indent: usize, ty: &List) -> (bool, String) {
        let mut new = false;
        let mut vec: VecDeque<_> = List::parse_pairs(line).into_iter().collect();
        match ty {
            List::Quote => match vec[0].as_rule() {
                Rule::ListMark => match vec[0].as_str() {
                    ">" => {
                        vec.pop_front();
                    }
                    _ => (),
                },
                _ => (),
            },
            List::Ordered => match vec[0].as_rule() {
                Rule::ListMark => match vec[0].as_str() {
                    "-" | ">" => (),
                    _ => {
                        vec.pop_front();
                        new = true
                    }
                },
                _ => (),
            },
            List::Orderless => match vec[0].as_rule() {
                Rule::ListMark => match vec[0].as_str() {
                    "-" => {
                        vec.pop_front();
                        new = true
                    }
                    _ => (),
                },
                _ => (),
            },
        }
        let v: Vec<&str> = vec.iter().map(|x| x.as_str()).collect();
        return (new, v.join(""));
    }
    fn parse_pairs(input: &str) -> Pairs<Rule> {
        let p = NoteDownParser::parse(Rule::ListMode, input).unwrap_or_else(|e| panic!("{}", e));
        p.into_iter().next().unwrap().into_inner()
    }
}
