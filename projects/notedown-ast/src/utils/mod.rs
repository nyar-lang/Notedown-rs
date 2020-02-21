use crate::{Context, AST};
use notedown_parser::NoteDownRule as Rule;
use pest::iterators::Pair;

pub use textwrap::dedent;

/// https://stackoverflow.com/questions/60337455/how-to-trim-space-less-than-n-times
pub fn trim_dedent(text: &str, max: usize) -> String {
    let mut new_text = text
        .lines()
        .map(|line| {
            let mut max = max;
            line.chars()
                // Skip while `c` is a whitespace and at most `max` spaces
                .skip_while(|c| {
                    if max == 0 {
                        false
                    }
                    else {
                        max -= 1;
                        c.is_whitespace()
                    }
                })
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join("\n");

    // Did the original `text` end with a `\n` then add it again
    if text.ends_with('\n') {
        new_text.push('\n');
    }

    new_text
}

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

pub fn str_escape(s: &str) -> String {
    match s.chars().next().unwrap() {
        '\'' => unescape(&s[1..s.len() - 1], "'"),
        '"' => unescape(&s[1..s.len() - 1], "\""),
        _ => s.to_string(),
    }
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
    match t {
        AST::None => AST::None,
        _ => AST::Paragraph(Box::new(t)),
    }
}

pub fn map_escape(c: &str) -> AST {
    match c {
        "\\*" => AST::String(String::from("*")),
        "\\|" => AST::String(String::from("|")),
        "\\#" => AST::String(String::from("#")),
        "\\-" => AST::String(String::from("-")),
        "\\t" => AST::String(String::from("\t")),
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
