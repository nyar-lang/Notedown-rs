use crate::{AST, Value};
use notedown_parser::{NoteDownParser, NoteDownRule as Rule, NoteTextParser, NoteTextRule as Text};
use pest::iterators::Pair;
use pest::Parser;
use std::fs::read_to_string;
use std::collections::HashMap;
use crate::utils::unescape;

#[derive(Debug, Clone)]
pub struct Context {
    ast: AST
}

impl Default for Context {
    fn default() -> Self {
        Context { ast: AST::None }
    }
}

macro_rules! debug_cases {
    ($i:ident) => {{
        println!("Rule::{:?}=>continue,", $i.as_rule());
        println!("Span: {:?}", $i.as_span());
        println!("Text: {}", $i.as_str());
        unreachable!();
    }};
}

impl Context {
    pub fn free(&self) -> AST {
        self.ast.clone()
    }
}

impl Context {
    pub fn parse_file(&self, path_from: &str, path_to: &str) -> Result<Context, std::io::Error> {
        let r = read_to_string(path_from)?;
        let a = self.parse(&r);
        return Ok(a);
    }
    pub fn parse(&self, text: &str) -> Context {
        let pairs = NoteDownParser::parse(Rule::program, text).unwrap_or_else(|e| panic!("{}", e));
        let mut codes = vec![];
        for pair in pairs {
            let code = match pair.as_rule() {
                Rule::EOI => continue,
                Rule::COMMENT => continue,
                Rule::NEWLINE => continue,
                Rule::TextBlock => maybe_math(self, pair),
                Rule::Header => self.parse_header(pair),
                Rule::List => self.parse_list(pair.as_str().trim()),
                Rule::Table => self.parse_table(pair.as_str().trim()),
                Rule::CommandBlock => self.parse_command(pair),
                _ => debug_cases!(pair),
            };
            println!("{}", code);
            codes.push(code);
        }
        return Context { ast: AST::Statements(codes) };
    }
    fn parse_header(&self, pairs: Pair<Rule>) -> AST {
        let mut level = 0;
        let mut head = AST::None;
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::Sharp => level += 1,
                Rule::RestOfLine => head = self.parse_text(pair.as_str().trim()),
                _ => debug_cases!(pair),
            };
        }
        return AST::Header {
            0: Box::new(head),
            1: level,
        };
    }
    fn parse_text(&self, text: &str) -> AST {
        let pairs = NoteTextParser::parse(Text::TextMode, text).unwrap_or_else(|e| panic!("{}", e));
        let mut codes: Vec<AST> = vec![];
        for pair in pairs {
            let code = match pair.as_rule() {
                Text::EOI => continue,
                Text::Style => self.parse_style(pair),
                Text::English => AST::String(pair.as_str().to_string()),
                Text::Word => AST::String(pair.as_str().to_string()),
                Text::Line => self.parse_line(pair),
                Text::Code => self.parse_code_inline(pair),
                Text::Math => self.parse_math(pair),
                Text::SPACE_SEPARATOR => AST::Space,
                Text::NEWLINE => AST::Newline,
                _ => debug_cases!(pair),
            };
            codes.push(code);
        }
        return AST::Text(codes);
    }
    fn parse_table(&self, text: &str) -> AST {
        return AST::None;
    }
    fn parse_list(&self, text: &str) -> AST {
        return AST::None;
    }
    fn parse_style(&self, pairs: Pair<Text>) -> AST {
        let s = pairs.as_str();
        let mut level = 0;
        let mut text = AST::None;
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Text::StyleLevel => level += pair.as_str().len(),
                Text::StyleText => text = self.parse_text(pair.as_str().trim()),
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
    fn parse_line(&self, pairs: Pair<Text>) -> AST {
        let s = pairs.as_str();
        let mut level = 0;
        let mut text = AST::None;
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Text::LineLevel => level += pair.as_str().len(),
                Text::LineText => text = self.parse_text(pair.as_str().trim()),
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
    fn parse_code_inline(&self, pairs: Pair<Text>) -> AST {
        let mut text = "";
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Text::CodeLevel => continue,
                Text::CodeText => text = pair.as_str().trim(),
                _ => debug_cases!(pair),
            };
        }
        AST::Code(text.to_string())
    }
    fn parse_math(&self, pairs: Pair<Text>) -> AST {
        let s = pairs.as_str();
        let mut level = 0;
        let mut text = "";
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Text::MathLevel => level += pair.as_str().len(),
                Text::MathText => text = pair.as_str().trim(),
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
                Rule::command => cmd = pair.as_str().trim_start_matches('\\').to_string(),
                Rule::argument_literal => arg.push(Value::String(unescape(pair.as_str(), ']'))),
                Rule::key_value => {
                    let mut k = String::new();
                    let mut v = Value::None;
                    for inner in pair.into_inner() {
                        match inner.as_rule() {
                            Rule::Set => continue,
                            Rule::Comma => continue,
                            Rule::key => { k = inner.as_str().to_string() }
                            Rule::value => { v = self.parse_value(inner) }
                            _ => debug_cases!(inner),
                        };
                    }
                    kvs.insert(k, v);
                }
                _ => debug_cases!(pair),
            };
        }
        return AST::Command(cmd, arg, kvs);
    }
    fn parse_value(&self, pairs: Pair<Rule>) -> Value {
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::String => {
                    let s = pair.as_str().to_string();
                    return Value::String(s);
                }
                Rule::SYMBOL => {
                    let s = pair.as_str().to_string();
                    Value::Command(AST::Command(s, vec![], Default::default()))
                }
                _ => debug_cases!(pair),
            };
        }
        return Value::None;
    }
}

fn maybe_math(ctx: &Context, pair: Pair<Rule>) -> AST {
    let s = pair.as_str().trim();
    if s.starts_with("$$") && s.ends_with("$$") {
        let r = s[2..s.chars().count() - 2].to_string();
        if !r.contains("$$") {
            return AST::MathDisplay(r);
        }
    }
    let t = ctx.parse_text(s);
    AST::Paragraph(Box::new(t))
}
