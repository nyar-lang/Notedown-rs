use std::borrow::Cow;

mod from;

use std::fmt::{self, Display, Formatter};


#[derive(Debug, Clone)]
pub struct Highlighter<'a> {
    lang: &'static str,
    code: Cow<'a, str>,
    inline: bool,
    high_line: Vec<usize>,
}

impl<'a> Display for Highlighter<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{mark}\n{body}\n{mark}", mark = "`".repeat(3), body = self.code)
    }
}
