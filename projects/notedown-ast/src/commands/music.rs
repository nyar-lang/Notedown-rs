use crate::{Context, Value, AST};
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
            Some(v) => v,
            None => Value::from($default),
        }
    };
}

pub fn meting_js(server: &str, args: Vec<Value>, kvs: HashMap<String, Value>) -> Option<String> {
    let ty = required_arg!(kvs, "type", args, 0);
    let id = required_arg!(kvs, "id", args, 1);
    let mut args = vec![];
    args.push(format!("type={}", ty));
    args.push(format!("id={}", id));
    match kvs.get("autoplay") {
        None => (),
        Some(v) => args.push(format!("autoplay=\"{}\"", v))
    }
    match kvs.get("fixed") {
        None => (),
        Some(v) => args.push(format!("fixed=\"{}\"", v))
    }
    match kvs.get("mini") {
        None => (),
        Some(v) => args.push(format!("mini=\"{}\"", v))
    }
    match kvs.get("order") {
        None => (),
        Some(v) => args.push(format!("order={}", v))
    }
    let out = format!("<meting-js server={:?} {}></meting-js>", server, args.join(" "));
    return Some(out);
}


/// require `APlayer.js`, `melting.js`
pub fn netease(_: &Context, args: Vec<Value>, kvs: HashMap<String, Value>) -> Option<String> {
    meting_js("netease", args, kvs)
}

/// require `APlayer.js`, `melting.js`
pub fn netease(_: &Context, args: Vec<Value>, kvs: HashMap<String, Value>) -> Option<String> {
    meting_js("netease", args, kvs)
}

/// require `APlayer.js`, `melting.js`
pub fn netease(_: &Context, args: Vec<Value>, kvs: HashMap<String, Value>) -> Option<String> {
    meting_js("netease", args, kvs)
}

/// require `APlayer.js`, `melting.js`
pub fn netease(_: &Context, args: Vec<Value>, kvs: HashMap<String, Value>) -> Option<String> {
    meting_js("netease", args, kvs)
}

/// require `APlayer.js`, `melting.js`
pub fn netease(_: &Context, args: Vec<Value>, kvs: HashMap<String, Value>) -> Option<String> {
    meting_js("netease", args, kvs)
}


#[test]
fn test() {
    let ctx = Context::default();

    let args = vec![Value::from("album"), Value::from("19525")];
    let mut hash = HashMap::default();
    hash.insert("autoplay".to_string(), Value::from(false));

    let s = netease(&ctx, args, hash);
    println!("{:?}", s);
    unreachable!()
}
