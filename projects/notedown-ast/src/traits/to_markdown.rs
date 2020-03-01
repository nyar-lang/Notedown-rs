use crate::traits::NotedownConfig;

pub trait ToMarkdown {
    fn to_markdown_with(&self, cfg: &NotedownConfig) -> String;
    fn to_markdown(&self) -> String {
        self.to_markdown_with(&NotedownConfig::default())
    }
}
