mod to_html;
mod to_markdown;

use crate::{Value, AST};
use std::collections::HashMap;
pub use to_html::ToHTML;
pub use to_markdown::{MarkdownConfig, ToMarkdown};

#[derive(Debug, Clone)]
pub struct Context {
    pub ast: AST,
    pub cfg: Settings,
    pub meta: HashMap<String, Value>,
}

#[derive(Debug, Clone)]
pub struct Settings {
    pub tab_size: usize,
    pub template: NotedownTemplate,
    pub target: NotedownTarget,
}

#[derive(Debug, Copy, Clone)]
pub enum NotedownTemplate {
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

impl Default for Settings {
    fn default() -> Self {
        Settings { tab_size: 2, template: NotedownTemplate::Vue, target: NotedownTarget::Web }
    }
}
