#[allow(unused_imports)]
use crate::{NotedownAST as AST, NotedownParser, NotedownRule as Rule, ToAST};
use colored::*;
use pest::iterators::{Pair, Pairs};
use pest::Parser;
use std::collections::HashMap;

pub fn token_print(s: &str, rule: Rule) {
    let pairs = NotedownParser::parse(rule, s).unwrap_or_else(|e| panic!("{}", e));
    // Because ident_list is silent, the iterator will contain idents
    for pair in pairs {
        // A pair is a combination of the rule which matched and a span of input
        let rule = pair.as_rule();

        match rule {
            Rule::NEWLINE => {
                continue;
            }
            _ => (),
        }

        println!("{} {:?}", "Rule:".green(), pair.as_rule());
        println!("{} {:?}", "Span:".green(), pair.as_span());

        if rule != Rule::NEWLINE {
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
    let pairs = NotedownParser::parse(Rule::program, s).unwrap_or_else(|e| panic!("{}", e));
    let mut nodes: Vec<AST> = vec![];
    for pair in pairs {
        let rule = pair.as_rule();
        let node = match rule {
            Rule::EOI | Rule::COMMENT | Rule::NEWLINE => AST::None,
            Rule::Header => parse_header(pair.into_inner()),
            _ => {
                println!("unimplemented Rule::{:?}", rule);
                AST::None
            }
        };
        nodes.push(node)
    }
    return AST::Statements(nodes);
}

macro_rules! declare {
    ($name:ident) => {
        let mut $name: Vec<Pairs<Rule>> = vec![]
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

fn parse_header(pairs: Pairs<Rule>) -> AST {
    let mut level: Vec<Pair<Rule>> = vec![];
    let mut arguments: Vec<Pair<Rule>> = vec![];
    let mut content = "";
    for pair in pairs.into_iter() {
        match pair.as_rule() {
            Rule::Sharp => level.push(pair),
            Rule::arguments => arguments.push(pair),
            Rule::RestOfLine => content = pair.as_str(),
            _ => unreachable!(),
        }
    }
    println!("{:?}", level.len());
    println!("{:?}", arguments);
    println!("{:?}", parse_text(content));

    AST::None
}

fn parse_arguments(pairs: Vec<Pair<Rule>>) -> HashMap<String, String> {
    let arguments: HashMap<String, String> = HashMap::new();
    arguments
}

fn parse_text(raw: &str) -> String {
    let s = raw.trim();
    raw.to_string()
}
