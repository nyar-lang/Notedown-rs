use crate::{Context, AST};
use notedown_parser::NoteDownRule as Rule;
use pest::iterators::Pair;
use std::{
    collections::VecDeque,
    io::{repeat, Read},
};

pub fn unescape(s: &str, c: &str) -> String {
    let mut e = String::from("\\");
    e.push_str(c);
    s.replace(&e, c)
}

pub fn trim_escape(s: &str) -> &str {
    let s = s.trim();
    let mut out = s;
    if s.starts_with('\\') {
        out = &out[1..]
    }
    if s.ends_with('\\') {
        out = &out[..out.len() - 1]
    }
    return out;
}

pub fn maybe_math(ctx: &Context, pair: Pair<Rule>) -> AST {
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

pub fn map_escape(c: &str) -> AST {
    match c {
        "\\*" => AST::String(String::from("*")),
        "\\n" => AST::String(String::from("\n")),
        _ => {
            println!("escaping {}", c);
            unreachable!()
        }
    }
}

pub fn map_white_space(c: &str) -> AST {
    match c {
        "\t" => AST::String(String::from("    ")),
        _ => AST::Space,
    }
}
// pub fn fill_right<T>(mut input: Vec<T>, v: usize, default: Option<T>) -> Vec<T>
// where T: Clone,
// Vec<T> : From<Iterator<T>>{
// if input.len() >= v {
// return input;
// }
// let d = v - input.len();
//
// match default {
// None => {
// let value = input[input.len()].clone();
// let append: Vec<T> = vec![value].iter().cycle().take(d).collect();
// input.extend(append.iter().cloned());
// }
// Some(value) => {
// let append: Vec<T> = vec![value].iter().cycle().take(d).collect();
// input.extend(append.iter().cloned());
// }
// }
// return input;
// }
