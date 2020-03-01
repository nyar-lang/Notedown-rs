use crate::{Context, AST};
use notedown_parser::NoteDownRule as Rule;
use pest::iterators::Pair;

mod meta_info;

pub use meta_info::build_zola;
pub use textwrap::dedent;

/// https://stackoverflow.com/questions/60337455/how-to-trim-space-less-than-n-times
pub fn trim_dedent(input: &str, max: usize) -> String {
    let mut out: String = String::from(input);
    let mut j: usize = 0;
    let mut is_counting = true;
    let mut ws_cnt = 0;
    unsafe {
        let out_b = out.as_bytes_mut();
        for i in 0..out_b.len() {
            if is_counting == true && out_b[i] == b' ' {
                ws_cnt += 1;
                if ws_cnt == max {
                    is_counting = false;
                }
            }
            else {
                is_counting = false;
                if out_b[i] == b'\n' {
                    is_counting = true;
                    ws_cnt = 0;
                }
                out_b[j] = out_b[i];
                j += 1;
            }
        }
    }
    out.truncate(j);
    return out;
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

pub fn trim_split_or(s: &str) -> Vec<String> {
    s.split(|c| c == ',' || c == '|').map(|s| s.trim().to_string()).collect()
}

pub fn str_escape(s: &str) -> String {
    match s.chars().next().unwrap() {
        '\'' => unescape(&s[1..s.len() - 1], "'"),
        '"' => unescape(&s[1..s.len() - 1], "\""),
        _ => s.to_string(),
    }
}

pub fn maybe_math(ctx: &mut Context, pair: Pair<Rule>) -> AST {
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
        "\\*" => AST::from("*"),
        "\\|" => AST::from("|"),
        "\\#" => AST::from("#"),
        "\\-" => AST::from("-"),
        "\\\\" => AST::from("\\"),
        "\\\n" => AST::None,
        _ => {
            println!("escaping {:?}=>", c);
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
