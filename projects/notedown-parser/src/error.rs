use crate::note_down::Rule;

#[derive(Debug)]
pub enum Error {
    LexerError(String)
}

pub type ParserResult<T> = Result<T, Error>;


impl From<pest::error::Error<Rule>> for Error {
    fn from(e: pest::error::
    Error<Rule>) -> Self {
        Error::LexerError(e.to_string())
    }
}