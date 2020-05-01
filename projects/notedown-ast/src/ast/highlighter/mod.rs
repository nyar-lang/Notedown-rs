use std::borrow::Cow;

mod from;

use std::fmt::{self, Display, Formatter};


#[derive(Debug, Clone)]
pub struct Highlighter<'a> {
    pub(crate) lang: &'static str,
    pub(crate) code: Cow<'a, str>,
    pub(crate) inline: bool,
    pub(crate) high_line: Vec<usize>,
}

impl<'a> Display for Highlighter<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{mark}{lang}\n{body}\n{mark}", mark = "`".repeat(3), lang = self.lang, body = self.code)
    }
}
