mod to_html;
mod to_markdown;

use crate::AST;
#[cfg(feature = "desktop")]
use chrono::NaiveDateTime;
use lazy_static::{self, lazy::Lazy, LazyStatic};
#[cfg(feature = "desktop")]
use std::path::PathBuf;
use std::{collections::HashMap, sync::Mutex};
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
    pub target: NotedownBackend,
}

#[derive(Debug, Clone)]
pub struct NotedownMeta {
    pub file_name: Option<String>,
    #[cfg(feature = "desktop")]
    pub file_path: Option<PathBuf>,
    #[cfg(feature = "desktop")]
    pub created_time: Option<NaiveDateTime>,
    #[cfg(feature = "desktop")]
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
}

impl Default for Context {
    fn default() -> Self {
        Self { ast: AST::None, meta: Default::default() }
    }
}

impl Default for NotedownConfig {
    fn default() -> Self {
        Self {
            // basic
            tab_size: 2,
            target: NotedownBackend::Web,
        }
    }
}

impl Default for NotedownMeta {
    fn default() -> Self {
        Self {
            // extra
            file_name: None,
            #[cfg(feature = "desktop")]
            file_path: None,
            #[cfg(feature = "desktop")]
            created_time: None,
            #[cfg(feature = "desktop")]
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

// lazy_static! { pub static ref GLOBAL_CONFIG: Mutex<NotedownConfig> = Mutex::new(NotedownConfig::default()); }

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
        fn static_ref_initialize() -> Mutex<NotedownConfig> {
            Mutex::new(NotedownConfig::default())
        }
        #[inline(always)]
        fn stability() -> &'static Mutex<NotedownConfig> {
            static LAZY: Lazy<Mutex<NotedownConfig>> = Lazy::INIT;
            LAZY.get(static_ref_initialize)
        }
        stability()
    }
}

impl LazyStatic for GlobalConfig {
    fn initialize(lazy: &Self) {
        let _ = &**lazy;
    }
}
