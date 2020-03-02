mod to_html;
mod to_markdown;

use crate::AST;
use chrono::NaiveDateTime;
use lazy_static::{self, LazyStatic};
use std::{path::PathBuf, sync::Mutex};
pub use to_html::ToHTML;
pub use to_markdown::ToMarkdown;

#[derive(Debug)]
pub struct Context {
    pub ast: AST,
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
    pub created_time: Option<NaiveDateTime>,
    pub modified_time: Option<NaiveDateTime>,
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
        Context { ast: AST::None, meta: Default::default() }
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

// lazy_static! { pub static ref GlobalConfig: Mutex<NotedownConfig> = Mutex::new(NotedownConfig::default()); }
#[allow(dead_code)]
pub struct GlobalConfig {
    private_field: (),
}

#[doc(hidden)]
pub static GLOBAL_CONFIG: GlobalConfig = GlobalConfig { private_field: () };

impl lazy_static::__Deref for GlobalConfig {
    type Target = Mutex<NotedownConfig>;
    fn deref(&self) -> &Mutex<NotedownConfig> {
        #[inline(always)]
        fn __static_ref_initialize() -> Mutex<NotedownConfig> {
            Mutex::new(NotedownConfig::default())
        }
        #[inline(always)]
        fn __stability() -> &'static Mutex<NotedownConfig> {
            static LAZY: lazy_static::lazy::Lazy<Mutex<NotedownConfig>> = lazy_static::lazy::Lazy::INIT;
            LAZY.get(__static_ref_initialize)
        }
        __stability()
    }
}

impl LazyStatic for GlobalConfig {
    fn initialize(lazy: &Self) {
        let _ = &**lazy;
    }
}
