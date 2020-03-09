use crate::{utils::trim_split_or, Context, Value};
use chrono::{NaiveDate, NaiveDateTime};
use std::{
    collections::{HashMap, VecDeque},
    path::PathBuf,
};

pub fn import(ctx: &Context, mut _args: VecDeque<Value>, mut _kvs: HashMap<String, Value>) -> Option<String> {
    let code = String::new();
    let file_path = ctx.meta.file_path.clone();
    println!("{:?}", file_path);
    return Some(code);
}

pub fn set_title(ctx: &mut Context, args: &Vec<Value>) -> Option<String> {
    if let Value::String(s) = &args[0] {
        let title = s.trim();
        if title.len() != 0 {
            ctx.meta.title = Some(title.to_string())
        }
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
        let file_name = s.trim();
        if file_name.len() != 0 {
            ctx.meta.file_name = Some(file_name.to_string())
        }
    };
    Some(String::new())
}

pub fn set_tags(ctx: &mut Context, args: &Vec<Value>) -> Option<String> {
    if let Value::String(s) = &args[0] {
        let tags = s.trim();
        if tags.len() != 0 {
            ctx.meta.tags = trim_split_or(tags)
        }
    };
    Some(String::new())
}

pub fn set_categories(ctx: &mut Context, args: &Vec<Value>) -> Option<String> {
    if let Value::String(s) = &args[0] {
        let cats = s.trim();
        if cats.len() != 0 {
            ctx.meta.categories = trim_split_or(cats)
        }
    };
    Some(String::new())
}

pub fn set_series(ctx: &mut Context, args: &Vec<Value>) -> Option<String> {
    if let Value::String(s) = &args[0] {
        ctx.meta.series = trim_split_or(s)
    };
    Some(String::new())
}
