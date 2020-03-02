use crate::{Context, Value, AST};
use std::collections::{HashMap, VecDeque};

macro_rules! must_arg {
    ($kvs:ident, $name:literal, $args:ident, $attr:ident) => {
        match $kvs.remove($name) {
            Some(v) => {
                $attr.push(format!(concat!($name, "={:?}"), v.trim()));
            }
            None => match $args.pop_front() {
                None => return None,
                Some(v) => $attr.push(format!(concat!($name, "={:?}"), v.trim())),
            },
        }
    };
}

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

macro_rules! named_arg {
    ($kvs:ident, $name:literal, $args:ident, $attr:ident) => {
        match $kvs.remove($name) {
            Some(v) => $attr.push(format!(concat!($name, "={:?}"), v.trim())),
            None => {
                if let Some(v) = $args.pop_front() {
                    $attr.push(format!(concat!($name, "={:?}"), v.trim()))
                }
            }
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

pub fn fancy_quote(ctx: &mut Context, _args: &Vec<Value>, mut kvs: HashMap<String, Value>) -> AST {
    let by = match kvs.remove("body") {
        Some(v) => v,
        None => Value::from(""),
    };
    let ty = match kvs.remove("type") {
        Some(v) => v,
        None => Value::from(""),
    };
    match ctx.parse_program(by.as_str()) {
        AST::Statements(body) => AST::Quote { body, style: ty.to_string() },
        _ => AST::None,
    }
}

pub fn image_insert(_: &Context, mut args: VecDeque<Value>, mut kvs: HashMap<String, Value>) -> Option<String> {
    let mut attr = vec![];
    must_arg!(kvs, "src", args, attr);
    let link = match kvs.remove("link") {
        Some(v) => Some(format!("href={:?}", v.trim())),
        None => match args.pop_front() {
            Some(v) => Some(format!("href={:?}", v.trim())),
            None => None,
        },
    };
    named_arg!(kvs, "alt", args, attr);
    for (k, v) in kvs {
        attr.push(format!("{}={:?}", k, v.trim()))
    }
    let img = format!("<img {}>", attr.join(" "));
    match link {
        None => Some(img),
        Some(h) => Some(format!("<a {}>{}</a>", h, img)),
    }
}

pub fn link_insert(mut args: VecDeque<Value>, mut kvs: HashMap<String, Value>) -> Option<String> {
    let link = match kvs.remove("href") {
        Some(v) => v.trim().to_string(),
        None => match args.pop_front() {
            None => return None,
            Some(v) => v.trim().to_string(),
        },
    };
    let alt = match kvs.remove("alt") {
        Some(v) => v.trim().to_string(),
        None => match args.pop_front() {
            Some(v) => v.trim().to_string(),
            None => link.clone(),
        },
    };
    let mut attr = vec![format!("href={:?}", link)];
    for (k, v) in kvs {
        attr.push(format!("{}={:?}", k, v.trim()))
    }
    return Some(format!("<a href={:?}>{}</a>", attr.join(" "), alt));
}
