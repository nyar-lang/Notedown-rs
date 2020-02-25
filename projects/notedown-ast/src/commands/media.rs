use crate::{Value, Context, AST};
use std::collections::HashMap;

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
    match kvs.get("autoplay") {
        None => (),
        Some(v) => args.push(format!("autoplay=\"{}\"", v)),
    }
    match kvs.get("fixed") {
        None => (),
        Some(v) => args.push(format!("fixed=\"{}\"", v)),
    }
    match kvs.get("mini") {
        None => (),
        Some(v) => args.push(format!("mini=\"{}\"", v)),
    }
    match kvs.get("order") {
        None => (),
        Some(v) => args.push(format!("order={}", v)),
    }
    let out = format!("<meting-js server={:?} {}></meting-js>", server, args.join(" "));
    return Some(out);
}

pub fn fancy_quote(ctx: &mut Context, args: &Vec<Value>, kvs: &HashMap<String, Value>) -> AST {
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

pub fn image_insert(ctx: &mut Context, args: &Vec<Value>, kvs: &HashMap<String, Value>) -> Option<String> {
    let src = required_arg!(kvs, "src", args, 0);
    let ty = optional_arg!(kvs,"alt","");
    let out = format!("<img src={}>", src);
    return Some(out);
}

