use crate::{Context, Value, AST};
use std::collections::{HashMap, VecDeque};

macro_rules! required_arg {
    ($kvs:ident, $name:literal, $args:ident, $index:literal) => {
        match $kvs.get($name) {
            Some(v) => v,
            None => match $args.get($index) {
                None => {
                    return None;
                }
                Some(i) => i,
            },
        }
    };
}

macro_rules! optional_arg {
    ($kvs:ident, $name:literal, $default:expr) => {
        match $kvs.get($name) {
            Some(v) => v.clone(),
            None => Value::from($default),
        }
    };
}

pub fn meting_js(server: &str, args: &Vec<Value>, kvs: &HashMap<String, Value>) -> Option<String> {
    let ty = required_arg!(kvs, "type", args, 0);
    let id = required_arg!(kvs, "id", args, 1);
    let mut args = vec![];
    args.push(format!("type={}", ty));
    args.push(format!("id={}", id));
    if let Some(v) = kvs.get("autoplay") {
        args.push(format!("autoplay={:?}", v))
    }
    if let Some(v) = kvs.get("fixed") {
        args.push(format!("fixed={:?}", v))
    }
    if let Some(v) = kvs.get("mini") {
        args.push(format!("mini={:?}", v))
    }
    if let Some(v) = kvs.get("order") {
        args.push(format!("order={:?}", v))
    }
    return Some(format!("<meting-js server={:?} {}></meting-js>", server, args.join(" ")));
}

pub fn fancy_quote(ctx: &mut Context, _args: &Vec<Value>, kvs: &HashMap<String, Value>) -> AST {
    let by = match kvs.get("body") {
        Some(v) => v.clone(),
        None => Value::from(""),
    };
    let ty = match kvs.get("type") {
        Some(v) => v.clone(),
        None => Value::from(""),
    };
    match ctx.parse_program(by.as_str()) {
        AST::Statements(body) => AST::Quote { body, style: ty.to_string() },
        _ => AST::None,
    }
}

pub fn image_insert(_: &Context, args: &Vec<Value>, kvs: &HashMap<String, Value>) -> Option<String> {
    let mut args = VecDeque::from(args.clone());
    let mut dict = vec![];
    match kvs.get("src") {
        Some(v) => dict.push(format!("src={:?}", v.trim())),
        None => match args.pop_front() {
            None => return None,
            Some(v) => dict.push(format!("src={:?}", v.trim())),
        },
    }
    match kvs.get("alt") {
        Some(v) => dict.push(format!("alt={:?}", v.trim())),
        None => {
            if let Some(v) = args.pop_front() {
                dict.push(format!("alt={:?}", v.trim()))
            }
        }
    }
    let img = format!("<img {}>", dict.join(" "));
    match kvs.get("link") {
        Some(v) => Some(format!("<a href={:?}>{}</a>", v.trim(), img)),
        None => match args.pop_front() {
            Some(v) => Some(format!("<a href={:?}>{}</a>", v.trim(), img)),
            None => Some(img),
        },
    }
}

pub fn link_insert(_: &Context, args: &Vec<Value>, kvs: &HashMap<String, Value>) -> Option<String> {
    let mut args = VecDeque::from(args.clone());
    let link = match kvs.get("href") {
        Some(v) => v.trim(),
        None => match args.pop_front() {
            None => return None,
            Some(v) => v.trim(),
        },
    };
    let alt = match kvs.get("alt") {
        Some(v) => v.trim(),
        None => match args.pop_front() {
            Some(v) => v.trim(),
            None => link.clone(),
        },
    };
    return Some(format!("<a href={:?}>{}</a>", link, alt));
}
