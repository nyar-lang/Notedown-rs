mod to_html;
mod to_markdown;

use crate::AST;
use std::{path::PathBuf, time::SystemTime};
pub use to_html::ToHTML;
pub use to_markdown::ToMarkdown;

#[derive(Debug)]
pub struct Context {
    pub ast: AST,
    pub cfg: NotedownConfig,
    pub meta: NotedownMeta,
}

#[derive(Debug, Clone)]
pub struct NotedownConfig {
    pub tab_size: usize,
    pub template: MissingCommand,
    pub target: NotedownTarget,
}

#[derive(Debug, Clone)]
pub struct NotedownMeta {
    pub file_name: Option<String>,
    pub file_path: Option<PathBuf>,
    pub created_time: Option<SystemTime>,
    pub modified_time: Option<SystemTime>,
    pub title: Option<String>,
    pub tags: Vec<String>,
    pub categories: Vec<String>,
    pub series: Vec<String>,
    pub weights: usize,
}

#[derive(Debug, Copy, Clone)]
pub enum MissingCommand {
    Vue,
    Zola,
}

#[derive(Debug, Copy, Clone)]
pub enum NotedownTarget {
    Web,
    VSCode,
    Zola,
}

impl Default for Context {
    fn default() -> Self {
        Context { ast: AST::None, cfg: Default::default(), meta: Default::default() }
    }
}

impl Default for NotedownConfig {
    fn default() -> Self {
        NotedownConfig {
            // basic
            tab_size: 2,
            template: MissingCommand::Vue,
            target: NotedownTarget::Web,
        }
    }
}

impl Default for NotedownMeta {
    fn default() -> Self {
        NotedownMeta {
            // extra
            file_name: None,
            file_path: None,
            created_time: None,
            modified_time: None,
            title: None,
            tags: vec![],
            categories: vec![],
            series: vec![],
            weights: 0,
        }
    }
}
