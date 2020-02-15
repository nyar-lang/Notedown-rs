use std::fs::{read_to_string, File};
use std::io::Write;
use notedown_parser::{NoteDownParser, NoteDownRule as Rule, NoteTextParser, NoteTextRule as Text};
use pest::Parser;
use crate::AST;
use pest::iterators::Pair;
use crate::ast::AST::Header;

#[derive(Debug, Copy, Clone)]
pub struct Context {}

impl Default for Context {
    fn default() -> Self {
        Context {}
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
    pub fn parse_file(&self, path_from: &str, path_to: &str) -> Result<AST, std::io::Error> {
        let r = read_to_string(path_from)?;
        let a = self.parse(&r);
        return Ok(a);
    }
    pub fn parse(&self, text: &str) -> AST {
        let pairs = NoteDownParser::parse(Rule::program, text).unwrap_or_else(|e| panic!("{}", e));
        let mut codes = vec![];
        for pair in pairs {
            let code = match pair.as_rule() {
                Rule::EOI => continue,
                Rule::COMMENT => continue,
                Rule::NEWLINE => continue,
                Rule::TextBlock => self.parse_text(pair.as_str().trim()),
                Rule::Header => self.parse_header(pair),
                Rule::List => self.parse_list(pair.as_str().trim()),
                Rule::Table => self.parse_table(pair.as_str().trim()),
                _ => debug_cases!(pair),
            };
            println!("{}", code);
            codes.push(code);
        }

        return AST::Statements(codes);
    }
    fn parse_header(&self, pairs: Pair<Rule>) -> AST {
        let mut level = 0;
        let mut head = AST::None;
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::Sharp => { level += 1 }
                Rule::RestOfLine => { head = self.parse_text(pair.as_str().trim()) }
                _ => debug_cases!(pair),
            };
        }
        return AST::Header { 0: Box::new(head), 1: level };
    }
    fn parse_text(&self, text: &str) -> AST {
        let pairs = NoteTextParser::parse(Text::TextMode, text).unwrap_or_else(|e| panic!("{}", e));
        let mut codes: Vec<AST> = vec![];
        for pair in pairs {
            let code = match pair.as_rule() {
                Text::EOI => continue,
                Text::Style => self.parse_style(pair),
                Text::English => AST::String(pair.as_str().to_string()),
                Text::SPACE_SEPARATOR => AST::String(String::from(" ")),
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
        let mut level = 0;
        let mut text = AST::None;
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Text::StyleLevel => { level += pair.as_str().len() }
                Text::StyleText => text = self.parse_text(pair.as_str().trim()),
                _ => debug_cases!(pair),
            };
        }
        match level {
            1 => AST::Italic(Box::new(text)),
            2 => AST::Bold(Box::new(text)),
            _ => AST::String(text.to_string())
        }
    }
}


#[test]
fn test() {
    let c = Context::default();
    let f = c.parse(r#"
        **bold**, *it*
    "#);
    println!("{:#?}", f);
    unreachable!()
}