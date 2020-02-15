mod to_html;

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
