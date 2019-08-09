#[allow(unused_imports)]
use crate::{
    NotedownAST as AST, NotedownParser, NotedownRule, TextModeParser, TextModeRule, ToAST,
};
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
            NotedownRule::NEWLINE => continue,
            _ => (),
        }

        println!("{} {:?}", "Rule:".green(), pair.as_rule());
        println!("{} {:?}", "Span:".green(), pair.as_span());

        if rule != NotedownRule::NEWLINE {
            println!("{} {}", "Text:".green(), pair.as_str());
        } else {
            println!();
        }

        // A pair can be converted to an iterator of the tokens which make it up:
        for inner_pair in pair.into_inner() {
            println!("    {} {:?}", "Inner:".cyan(), inner_pair.as_rule());
            println!("           {:?}", inner_pair.as_span());
        }
    }
}

pub fn parse(s: &str) -> AST {
    let pairs = NotedownParser::parse(NotedownRule::program, s).unwrap_or_else(|e| panic!("{}", e));
    let mut nodes: Vec<AST> = vec![];
    for pair in pairs {
        let rule = pair.as_rule();
        let node = match rule {
            NotedownRule::EOI | NotedownRule::COMMENT | NotedownRule::NEWLINE => AST::None,
            NotedownRule::Header => parse_header(pair.into_inner()),
            _ => {
                println!("unimplemented Rule::{:?}", rule);
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
    println!("{:?}", pairs);
    let mut nodes: Vec<AST> = vec![];
    for pair in pairs {
        let rule = pair.as_rule();
        let node = match rule {
            TextModeRule::EOI => continue,
            TextModeRule::SPACE_SEPARATOR => continue,
            TextModeRule::NEWLINE => AST::Newline,
            TextModeRule::English => AST::Word(pair.as_str().to_string()),
            _ => {
                println!("unimplemented Rule::{:?}", rule);
                AST::None
            }
        };
        nodes.push(node)
    }
    println!("{:?}", nodes);
    return AST::Statements(nodes);
}

macro_rules! declare {
    ($name:ident) => {
        let mut $name: Vec<Pairs<NotedownRule>> = vec![]
    };
    {$($x:ident),*} => {
        $(declare!($x);)*
    };
}
macro_rules! rules {
    ($rule:ident $name:ident) => {
        $rule => $name.push(pair.into_inner()),
    };
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
    arguments
}
