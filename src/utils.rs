#[allow(unused_imports)]
use crate::{NotedownAST as AST, NotedownParser, NotedownRule, TextModeParser, TextModeRule};
#[cfg(feature = "colored")]
use colored::*;
use pest::iterators::{Pair, Pairs};
use pest::Parser;
use std::collections::HashMap;

#[cfg(feature = "colored")]
pub fn token_print(s: &str, rule: NotedownRule) {
    let pairs: Pairs<NotedownRule> =
        NotedownParser::parse(rule, s).unwrap_or_else(|e| panic!("{}", e));
    // Because ident_list is silent, the iterator will contain idents
    for pair in pairs {
        // A pair is a combination of the rule which matched and a span of input
        let rule = pair.as_rule();

        match rule {
            NotedownRule::NEWLINE | NotedownRule::EOI => continue,
            _ => (),
        }

        println!("{} {:?}", "Rule:".green(), pair.as_rule());
        println!("{} {:?}", "Span:".green(), pair.as_span());
        println!("{} {}", "Text:".green(), pair.as_str());

        // A pair can be converted to an iterator of the tokens which make it up:
        for inner_pair in pair.into_inner() {
            println!("    {} {:?}", "Inner:".cyan(), inner_pair.as_rule());
            println!("           {:?}", inner_pair.as_span());
        }
    }
}

#[allow(unused_macros)]
macro_rules! declare {
    ($name:ident) => {
        let mut $name: Vec<Pairs<NotedownRule>> = vec![]
    };
    {$($x:ident),*} => {
        $(declare!($x);)*
    };
}

#[allow(unused_macros)]
macro_rules! rules {
    ($rule:ident $name:ident) => {
        $rule => $name.push(pair.into_inner()),
    };
}

pub fn parse(s: &str) -> AST {
    let pairs = NotedownParser::parse(NotedownRule::program, s).unwrap_or_else(|e| panic!("{}", e));
    let mut nodes: Vec<AST> = vec![];
    for pair in pairs {
        let rule = pair.as_rule();
        let node = match rule {
            NotedownRule::EOI | NotedownRule::COMMENT | NotedownRule::NEWLINE => continue,
            NotedownRule::Header => parse_header(pair.into_inner()),
            NotedownRule::TextBlock => AST::Paragraph(Box::new(parse_text(pair.as_str()))),
            _ => {
                println!("unimplemented NotedownRule::{:?}", rule);
                AST::None
            }
        };
        nodes.push(node)
    }
    return AST::Statements(nodes);
}

fn parse_text(raw: &str) -> AST {
    let s = raw.trim().replace("\t", "    ");
    let pairs: Pairs<TextModeRule> =
        TextModeParser::parse(TextModeRule::text_mode, &s).unwrap_or_else(|e| panic!("{}", e));
    let mut nodes: Vec<AST> = vec![];
    for pair in pairs {
        let rule = pair.as_rule();
        let node = match rule {
            TextModeRule::EOI => continue,
            TextModeRule::SPACE_SEPARATOR => continue,
            TextModeRule::NEWLINE => AST::Newline,
            TextModeRule::English => AST::Word(pair.as_str().to_string()),
            TextModeRule::Word => AST::Word(pair.as_str().to_string()),
            TextModeRule::Escaped => {
                let s = pair.as_str();
                let last = &s[s.len() - 1..];
                AST::Escaped(last.to_string())
            }
            TextModeRule::Math => parse_math_inline(pair),
            TextModeRule::Style => parse_style(pair),
            TextModeRule::StyleRest => AST::Word(pair.as_str().to_string()),
            _ => {
                println!("unimplemented TextModeRule::{:?}", rule);
                AST::None
            }
        };
        nodes.push(node)
    }
    println!("{:?}", nodes);
    return AST::Statements(nodes);
}

fn parse_header(pairs: Pairs<NotedownRule>) -> AST {
    let mut level: Vec<Pair<NotedownRule>> = vec![];
    let mut arguments: Vec<Pair<NotedownRule>> = vec![];
    let mut content = "";
    for pair in pairs.into_iter() {
        match pair.as_rule() {
            NotedownRule::Sharp => level.push(pair),
            NotedownRule::arguments => arguments.push(pair),
            NotedownRule::RestOfLine => content = pair.as_str(),
            _ => unreachable!(),
        }
    }
    println!("{:?}", level.len());
    println!("{:?}", arguments);
    let map: HashMap<String, String> = HashMap::new();

    AST::Header(Box::new(parse_text(content)), level.len() as u8, map)
}

fn parse_arguments(pairs: Vec<Pair<NotedownRule>>) -> HashMap<String, String> {
    let arguments: HashMap<String, String> = HashMap::new();
    println!("{:?}", pairs);
    arguments
}

fn parse_style(pair: Pair<TextModeRule>) -> AST {
    let tokens: Vec<_> = pair.clone().into_inner().into_iter().collect();
    let level = tokens[0].as_str().len() as u8;
    let text = tokens[1].as_str();
    let content = if text.trim().len() == 0 {
        Box::new(AST::from(text))
    } else {
        Box::new(parse_text(text))
    };
    match level {
        1 => AST::Italic(content, level),
        2 => AST::Bold(content, level),
        3 => {
            let i = AST::Italic(content, 0);
            AST::Bold(Box::new(i), level)
        }
        _ => {
            println!("unknow *{}", level);
            AST::Word(pair.as_str().to_string())
        }
    }
}

fn parse_math_inline(pair: Pair<TextModeRule>) -> AST {
    let tokens: Vec<_> = pair.clone().into_inner().into_iter().collect();
    let level = tokens[0].as_str().len() as u8;
    let text = tokens[1].as_str();
    let content = if text.trim().len() == 0 {
        return AST::from(text);
    };
    match level {
        1 => AST::MathInline(text.trim().to_string()),
        2 => AST::MathInline(format!("\\displaystyle{}", text.trim())),
        _ => {
            println!("unknow ${}", level);
            AST::Word(pair.as_str().to_string())
        }
    }
}
