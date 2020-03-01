use crate::NotedownMeta;
use chrono::{DateTime, Utc};

pub fn build_zola(meta: &NotedownMeta) -> String {
    let mut code = String::with_capacity(100);
    if let Some(s) = &meta.title {
        code.push_str(&format!("title = \"{}\"\n", s));
    }
    if let Some(s) = &meta.created_time {
        code.push_str(&format!("date = {}\n", DateTime::<Utc>::from(*s).to_rfc3339()));
    }

    return format!("+++\n{}+++\n\n", code);
}
