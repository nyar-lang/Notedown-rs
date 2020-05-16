use notedown_ast::AST;
use pest::iterators::Pair;
use crate::note_down::Rule;
use crate::ParserConfig;

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

pub fn maybe_math(ctx: &mut ParserConfig, pair: Pair<Rule>) -> AST {
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
        _ => AST::Paragraph(vec![t]),
    }
}

pub fn map_escape(c: &str) -> AST {
    match c {
        "\\\r" => AST::None,
        "\\\n" => AST::None,
        _ => {
            unimplemented!()
            // println!("escaping {:?}=>", c);
            //AST::from(c.chars().last().unwrap())
        }
    }
}

pub fn map_white_space(c: &str) -> AST {
    unimplemented!()
    //match c {
    //    "\t" => AST::String(String::from("    ")),
   //     _ => AST::Space,
   // }
}
