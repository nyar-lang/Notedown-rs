pub trait ToHTML {
    fn to_html(&self, cfg: HTMLConfig) -> String;
}

pub trait ToMarkdown {
    fn to_md(&self, cfg: MarkdownConfig) -> String;
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
