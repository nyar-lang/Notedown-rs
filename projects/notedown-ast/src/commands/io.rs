use crate::{utils::trim_split_or, Context, Value};
use chrono::{DateTime, NaiveDate, NaiveDateTime};
use std::collections::HashMap;

pub fn import(_args: &Vec<Value>, _kvs: &HashMap<String, Value>) -> Option<String> {
    let code = String::new();
    return Some(code);
}

pub fn set_title(ctx: &mut Context, args: &Vec<Value>) -> Option<String> {
    if let Value::String(s) = &args[0] {
        ctx.meta.title = Some(s.trim().to_string())
    };
    Some(String::new())
}

pub fn set_date(ctx: &mut Context, args: &Vec<Value>) -> Option<String> {
    // notice that parse no trim needed
    if let Value::String(s) = &args[0] {
        if let Ok(t) = NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S") {
            ctx.meta.created_time = Some(t)
        }
        if let Ok(t) = NaiveDate::parse_from_str(s, "%Y-%m-%d") {
            ctx.meta.created_time = Some(t.and_hms(6, 0, 0))
        }
    };
    Some(String::new())
}

pub fn set_file_name(ctx: &mut Context, args: &Vec<Value>) -> Option<String> {
    if let Value::String(s) = &args[0] {
        ctx.meta.file_name = Some(s.trim().to_string())
    };
    Some(String::new())
}

pub fn set_tags(ctx: &mut Context, args: &Vec<Value>) -> Option<String> {
    if let Value::String(s) = &args[0] {
        ctx.meta.tags = trim_split_or(s)
    };
    Some(String::new())
}

pub fn set_categories(ctx: &mut Context, args: &Vec<Value>) -> Option<String> {
    if let Value::String(s) = &args[0] {
        ctx.meta.categories = trim_split_or(s)
    };
    Some(String::new())
}

pub fn set_series(ctx: &mut Context, args: &Vec<Value>) -> Option<String> {
    if let Value::String(s) = &args[0] {
        ctx.meta.series = trim_split_or(s)
    };
    Some(String::new())
}
