#[derive(Debug, Copy, Clone)]
pub struct MarkdownConfig {}

impl Default for MarkdownConfig {
    fn default() -> Self {
        MarkdownConfig {}
    }
}

pub trait ToMarkdown {
    fn to_md(&self, cfg: MarkdownConfig) -> String;
    fn to_md_default(&self) -> String {
        self.to_md(MarkdownConfig::default())
    }
}
