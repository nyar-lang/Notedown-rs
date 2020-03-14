use crate::NotedownMeta;
use rex::{layout::Style, svg::render_to_string, RenderSettings};

pub fn build_zola(meta: &NotedownMeta) -> String {
    let mut code = String::with_capacity(100);
    if let Some(s) = &meta.title {
        code.push_str(&format!("title = \"{}\"\n", s));
    }
    if let Some(s) = &meta.created_time {
        code.push_str(&format!("date = {}\n", s.format("%Y-%m-%d")));
    }

    code.push_str("[taxonomies]\n");
    if meta.tags.len() != 0 {
        code.push_str(&format!("tags = {:?}\n", meta.tags));
    }
    if meta.categories.len() != 0 {
        code.push_str(&format!("categories = {:?}\n", meta.categories));
    }
    return format!("+++\n{}+++", code);
}

pub fn rex_math_display(math: &str) -> String {
    let cfg = RenderSettings { strict: false, style: Style::Display, debug: false, ..RenderSettings::default() };
    match render_to_string(&cfg, math) {
        Ok(o) => o,
        Err(_) => String::from(math),
    }
}

pub fn rex_math_inline(math: &str) -> String {
    let cfg = RenderSettings { strict: false, style: Style::Text, debug: false, ..RenderSettings::default() };
    match render_to_string(&cfg, math) {
        Ok(o) => o,
        Err(_) => String::from(math),
    }
}
