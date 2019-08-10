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

#[macro_export]
macro_rules! string_eq {
    ($text:expr,$ast:expr,$html:expr) => {
        let result = parse(TEXT);
        assert_eq!(format!("{}", result), $text);
        assert_eq!(format!("{:?}", result), $ast);
        assert_eq!(result.to_html_default(), $html);
    };
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

#[derive(Debug, Copy, Clone)]
pub struct HTMLConfig {}

impl Default for HTMLConfig {
    fn default() -> Self {
        HTMLConfig {}
    }
}

#[derive(Debug, Copy, Clone)]
pub struct MarkdownConfig {}

impl Default for MarkdownConfig {
    fn default() -> Self {
        MarkdownConfig {}
    }
}
