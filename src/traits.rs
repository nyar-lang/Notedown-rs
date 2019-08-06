use crate::NotedownAST;
#[allow(unused_imports)]
use std::collections::HashMap;

#[macro_export]
macro_rules! dict (
    {$($key:expr => $value:expr),+} => {
        {
            let mut map = HashMap::new();
            $(
                map.insert($key.to_string(), $value.to_string());
            )+
            map
        }
     };
);

pub trait ToAST {
    fn to_ast(&self) -> NotedownAST;
}

pub trait ToHTML {
    fn to_html(&self, cfg: HTMLConfig) -> String;
    fn to_html_default(&self) -> String {
        self.to_html(HTMLConfig::default())
    }
}

pub trait ToMarkdown {
    fn to_md(&self, cfg: MarkdownConfig) -> String;
    fn to_md_default(&self) -> String {
        self.to_md(MarkdownConfig::default())
    }
}

pub struct HTMLConfig {}

impl Default for HTMLConfig {
    fn default() -> Self {
        HTMLConfig {}
    }
}

pub struct MarkdownConfig {}

impl Default for MarkdownConfig {
    fn default() -> Self {
        MarkdownConfig {}
    }
}
