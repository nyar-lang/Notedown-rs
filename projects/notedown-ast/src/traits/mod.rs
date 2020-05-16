mod to_html;
mod to_markdown;

use crate::AST;
use chrono::NaiveDateTime;
use lazy_static::{self, lazy::Lazy, LazyStatic};
use std::path::PathBuf;
use std::{collections::HashMap, sync::Mutex};
pub use to_html::ToHTML;
pub use to_markdown::ToMarkdown;

#[derive(Debug)]
pub struct Context {
    pub ast: AST,
    pub meta: NotedownMeta,
    pub config: NotedownConfig,
}

#[derive(Debug, Clone)]
pub struct NotedownConfig {
    pub tab_size: usize,
    pub target: NotedownBackend,
}

#[derive(Debug, Clone)]
pub struct NotedownMeta {
    pub file_name: Option<String>,
    pub file_path: Option<PathBuf>,
    pub created_time: Option<NaiveDateTime>,
    pub modified_time: Option<NaiveDateTime>,
    pub title: Option<String>,
    pub tags: Vec<String>,
    pub categories: Vec<String>,
    pub series: Vec<String>,
    pub weight: usize,
    pub references: HashMap<Box<str>, String>,
}

#[derive(Debug, Copy, Clone)]
pub enum NotedownBackend {
    Web,
    Vue,
    VSCode,
    Zola,
    Yew,
}

impl Default for Context {
    fn default() -> Self {
        Self { ast: AST::None, meta: Default::default(), config: Default::default() }
    }
}

impl Default for NotedownConfig {
    fn default() -> Self {
        Self {
            // basic
            tab_size: 2,
            target: NotedownBackend::Zola,
        }
    }
}

impl Default for NotedownMeta {
    fn default() -> Self {
        Self {
            // extra
            file_name: None,
            file_path: None,
            created_time: None,
            modified_time: None,
            title: None,
            tags: vec![],
            categories: vec![],
            series: vec![],
            weight: 0,
            references: Default::default(),
        }
    }
}
