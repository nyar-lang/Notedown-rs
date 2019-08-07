#[allow(unused_imports)]
use crate::{NotedownAST as AST, NotedownParser, NotedownRule as Rule, ToAST};
use colored::*;
use pest::Parser;
use pest::iterators::Pairs;
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

fn parse_header(pairs: Pairs<Rule>) -> AST {
    let mut level: u8 = 0;
    let mut arguments: Vec<Pairs<Rule>> = vec![];
    let mut content = String::new();


    for pair in pairs {
        match pair.as_rule() {
            Rule::Sharp => level += 1,
            Rule::arguments => {
                arguments.push(pair.into_inner())
            }
            Rule::RestOfLine => {
                content = parse_text(pair.into_inner())
            },
            _ => unreachable!()
        }
    }


    println!("{:?}", level);
    println!("{:?}", arguments);
    println!("{:?}", content);
    AST::None
}

fn parse_arguments(pairs: Pairs<Rule>) -> HashMap<String, String> {
    let arguments:HashMap<String, String> = HashMap::new();
    arguments
}

fn parse_text(pairs: Pairs<Rule>) -> String {
    println!("{:?}", pairs);
    "".to_string()
}