use notedown_parser::{NoteDownParser, NoteDownRule as Rule};
use pest::{
    iterators::{Pair, Pairs},
    Parser,
};
use crate::utils::{dedent_less_than, indent};


macro_rules! debug_cases {
    ($i:ident) => {{
        println!("Rule::{:?}=>continue,", $i.as_rule());
        println!("Span: {:?}", $i.as_span());
        println!("Text: {}", $i.as_str());
        unreachable!();
    }};
}

#[derive(Debug, Clone)]
pub struct Settings {
    tab_size: usize,
    end_line: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Settings { tab_size: 2, end_line: true }
    }
}

impl Settings {
    pub fn format(&self, text: &str) -> String {
        let input = text.replace("\t", &" ".repeat(self.tab_size)).replace("\n\r", "\n");
        self.format_program(&input)
    }
    pub fn format_program(&self, text: &str) -> String {
        let pairs = NoteDownParser::parse(Rule::program, text).unwrap_or_else(|e| panic!("{}", e));
        let mut codes = vec![];
        for pair in pairs {
            let code = match pair.as_rule() {
                Rule::EOI => continue,
                Rule::NEWLINE => continue,
                Rule::SPACE_SEPARATOR => codes.push(String::from(" ")),

                Rule::Header => codes.push(self.format_header(pair)),
                Rule::TextBlock => codes.push(self.format_text(pair)),
                Rule::List=>codes.push(self.format_list(pair)),

                _ => debug_cases!(pair),
            };
        }
        return codes.join("\n");
    }
    fn format_header(&self, pairs: Pair<Rule>) -> String {
        let mut level = 0;
        let mut text = String::new();
        for pair in pairs.into_inner() {
            let code = match pair.as_rule() {
                Rule::SPACE_SEPARATOR => continue,
                Rule::Sharp => level += 1,
                Rule::RestOfLine => text = self.format_text(pair),
                _ => debug_cases!(pair),
            };
        }
        println!("{0} {1}", "#".repeat(level), text);
        String::new()
    }

    fn format_text(&self, input: Pair<Rule>) -> String {
        let text = input.as_str();
        let mut spaces = 0;
        for c in text.chars() {
            match c {
                ' ' => spaces += 1,
                _ => break
            }
        }
        let mut codes = vec![];
        for pair in text_parse(dedent_less_than(text, spaces).trim_end()) {
            let code = match pair.as_rule() {
                Rule::EOI => continue,
                Rule::TextRest => codes.push(pair.as_str().to_string()),
                Rule::Style => codes.push(self.format_style(pair)),
                Rule::StyleRest => codes.push(pair.as_str().to_string()),
                Rule::NEWLINE => codes.push(String::from("\n")),
                _ => debug_cases!(pair),
            };
        }
        format!("{}", &indent(&codes.join(""), &" ".repeat(spaces)))
    }
    fn format_style(&self, pairs: Pair<Rule>) -> String {
        let mut level = 0;
        let mut text = "";
        let mut codes = String::new();
        for pair in pairs.into_inner() {
            let code = match pair.as_rule() {
                Rule::Asterisk => continue,
                Rule::StyleLevel => level += 1,
                Rule::StyleText => text = pair.as_str(),
                _ => debug_cases!(pair),
            };
        }
        format!("{0}{1}{0}", "*".repeat(level), text)
    }
}

fn text_parse(text: &str) -> Pairs<Rule> {
    NoteDownParser::parse(Rule::TextMode, text).unwrap_or_else(|e| panic!("{}", e))
}

