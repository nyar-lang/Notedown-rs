#[allow(unused_imports)]
use crate::{NotedownParser, NotedownRule, ToAST};
use colored::*;
use pest::Parser;

pub fn token_print(s: &str, rule: NotedownRule) {
    let pairs = NotedownParser::parse(rule, s).unwrap_or_else(|e| panic!("{}", e));
    // Because ident_list is silent, the iterator will contain idents
    for pair in pairs {
        // A pair is a combination of the rule which matched and a span of input
        let rule = pair.as_rule();

        match rule {
            NotedownRule::NEWLINE => {
                continue;
            }
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
