use notedown_ast::{TextRange, Url};
use pest::Span;

pub struct ParserConfig {
    pub file_url: Option<Url>,
    pub tab_size: usize,
}

impl Default for ParserConfig {
    fn default() -> Self {
        Self { file_url: None, tab_size: 4 }
    }
}

impl ParserConfig {
    pub fn get_position(&self, s: Span) -> TextRange {
        let us = s.start_pos().line_col();
        let es = s.end_pos().line_col();
        TextRange {
            // index: s.start_pos().pos() as u64,
            start: (us.0 as u64, us.1 as u64),
            end: (es.0 as u64, es.1 as u64),
        }
    }
}
