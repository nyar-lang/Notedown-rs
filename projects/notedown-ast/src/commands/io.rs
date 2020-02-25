use crate::{Context, Value};
use std::collections::HashMap;

pub fn import(_args: &Vec<Value>, _kvs: &HashMap<String, Value>) {
    let _arguments = vec!["path", "format"];
}

pub fn set_title(ctx: &mut Context, args: &Vec<Value>) -> Option<String> {
    let title = match &args[0] {
        Value::String(s) => Value::String(s.trim().to_string()),
        _ => Value::from("Untitled"),
    };
    ctx.meta.insert(String::from("title"), title);
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
    ctx.meta.insert(String::from("title"), title);
    Some(String::new())
}

pub fn set_tags(ctx: &mut Context, args: &Vec<Value>) -> Option<String> {
    let title = match &args[0] {
        Value::String(s) => {
            let t = s.trim().to_string();
            Value::String(t)
        }
        _ => Value::None,
    };
    ctx.meta.insert(String::from("tags"), title);
    Some(String::new())
}

pub fn set_categories(ctx: &mut Context, args: &Vec<Value>) -> Option<String> {
    let title = match &args[0] {
        Value::String(s) => Value::String(s.trim().to_string()),
        _ => Value::from(""),
    };
    ctx.meta.insert(String::from("categories"), title);
    Some(String::new())
}

pub fn set_series(ctx: &mut Context, args: &Vec<Value>) -> Option<String> {
    let title = match &args[0] {
        Value::String(s) => Value::String(s.trim().to_string()),
        _ => Value::from(""),
    };
    ctx.meta.insert(String::from("series"), title);
    Some(String::new())
}
