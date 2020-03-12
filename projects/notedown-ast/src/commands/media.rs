use crate::{Context, Value, AST};
use std::collections::HashMap;

#[cfg(feature = "desktop")]
use carbon_lib::{utils::CarbonHTML, CarbonError, Config};

#[cfg(feature = "desktop")]
pub fn try_render_code(cmd: String, args: &Vec<Value>, kvs: &HashMap<String, Value>) -> Option<String> {
    let body = match kvs.remove("body") {
        Some(s) => s.trim().to_string(),
        None => return None,
    };
    let mut title: Option<String> = None;
    match kvs.remove("title") {
        Some(s) => title = Some(s.to_string()),
        None => {
            if let Some(s) = args.pop_front() {
                title = Some(s.to_string())
            }
        }
    };
    let mut carbon = Config::default();
    carbon.html_type = CarbonHTML::Raw;
    carbon.syntax = cmd;
    carbon.file_title = title;
    match carbon.render_html(&body) {
        Err(_) => return None,
        Ok(o) => Some(o),
    }
}

#[cfg(feature = "default")]
pub fn try_render_code(cmd: String, args: &Vec<Value>, kvs: &HashMap<String, Value>) -> Option<String> {
    Some(format!("\\{}{:?}{:?}", cmd, args, kvs))
}

pub fn meting_js(server: &str, args: &Vec<Value>, kvs: &HashMap<String, Value>) -> Option<String> {
    let (mut ty, mut id) = Default::default();
    // <a href=""><img src=""> xx </a>
    match &args[..] {
        [t, i] => {
            ty = t.to_string();
            id = i.to_string();
        }
        _ => return None,
    }
    let mut attr = format!("type={:?} id={:?}", ty, id);
    for (k, v) in kvs {
        attr.push_str(&format!(" {}={:?}", k, v.trim()))
    }
    return Some(format!("<meting-js server={:?} {}></meting-js>", server, attr));
}

pub fn fancy_quote(ctx: &mut Context, args: &Vec<Value>, kvs: &HashMap<String, Value>) -> AST {
    let (mut body, mut ty) = Default::default();
    match &args[..] {
        [by] => {
            body = by.to_string();
        }
        [by, t] => {
            body = by.to_string();
            ty = t.to_string();
        }
        _ => (),
    }
    match ctx.parse_program(body.as_str()) {
        AST::Statements(body) => AST::Quote { body, style: ty.to_string() },
        _ => AST::None,
    }
}

pub fn image_insert(_: &Context, args: &Vec<Value>, kvs: &HashMap<String, Value>) -> Option<String> {
    let (mut text, mut src, mut href) = Default::default();
    // <a href=""><img src=""> xx </a>
    match &args[..] {
        [image] => {
            src = image.to_string();
            text = image.to_string();
        }
        [alt, image] => {
            src = image.to_string();
            text = alt.to_string();
        }
        [alt, image, link] => {
            src = image.to_string();
            text = alt.to_string();
            href = Some(link)
        }
        _ => return None,
    }
    let mut attr = vec![format!("src={:?}", src)];
    for (k, v) in kvs {
        attr.push(format!("{}={:?}", k, v.trim()))
    }
    let img = format!("<img {}>", attr.join(" "));
    match href {
        None => Some(img),
        Some(h) => Some(format!("<a href={:?}>{}</a>", h, img)),
    }
}

pub fn link_insert(_: &Context, args: &Vec<Value>, kvs: &HashMap<String, Value>) -> Option<String> {
    let (mut link, mut alt) = Default::default();
    match &args[..] {
        [a] => {
            link = a.to_string();
            alt = a.to_string();
        }
        [l, a] => {
            link = l.to_string();
            alt = a.to_string();
        }
        _ => return None,
    }
    let mut attr = vec![format!("href={:?}", link)];
    for (k, v) in kvs {
        attr.push(format!("{}={:?}", k, v.trim()))
    }
    return Some(format!("<a {}>{}</a>", attr.join(" "), alt));
}
