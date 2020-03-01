use crate::{utils::trim_split_or, Context, Value};
use std::collections::HashMap;

pub fn import(_args: &Vec<Value>, _kvs: &HashMap<String, Value>) {
    let _arguments = vec!["path", "format"];
}

pub fn set_title(ctx: &mut Context, args: &Vec<Value>) -> Option<String> {
    match &args[0] {
        Value::String(s) => ctx.meta.title = Some(s.trim().to_string()),
        _ => (),
    };
    Some(String::new())
}

pub fn set_date(ctx: &mut Context, args: &Vec<Value>) -> Option<String> {
    let mut title: Value = Value::None;
    match &args[0] {
        Value::String(s) => {
            let t = s.trim().to_string();
            if t.len() != 0 {
                title = Value::String(t);
            }
        }
        _ => return None,
    };
    Some(String::new())
}

pub fn set_tags(ctx: &mut Context, args: &Vec<Value>) -> Option<String> {
    match &args[0] {
        Value::String(s) => ctx.meta.tags = trim_split_or(s),
        _ => (),
    };
    Some(String::new())
}

pub fn set_categories(ctx: &mut Context, args: &Vec<Value>) -> Option<String> {
    match &args[0] {
        Value::String(s) => ctx.meta.categories = trim_split_or(s),
        _ => (),
    };
    Some(String::new())
}

pub fn set_series(ctx: &mut Context, args: &Vec<Value>) -> Option<String> {
    match &args[0] {
        Value::String(s) => ctx.meta.series = trim_split_or(s),
        _ => (),
    };
    Some(String::new())
}
