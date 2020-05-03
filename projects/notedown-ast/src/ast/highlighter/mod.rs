mod from;

use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone)]
pub struct Highlighter {
    pub lang: String,
    pub code: String,
    pub inline: bool,
    pub high_line: Vec<usize>,
}

impl Display for Highlighter {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{mark}{lang}\n{body}\n{mark}", mark = "`".repeat(3), lang = self.lang, body = self.code)
    }
}
