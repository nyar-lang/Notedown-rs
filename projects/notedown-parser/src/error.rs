use crate::note_down::Rule;

#[derive(Debug)]
pub enum Error {
    LexerError(String),
    FileNotFound(String),
    IOError(String),
}

pub type ParserResult<T> = Result<T, Error>;

impl From<pest::error::Error<Rule>> for Error {
    fn from(e: pest::error::Error<Rule>) -> Self {
        Error::LexerError(format!("{}", e))
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::IOError(format!("{:?}", e))
    }
}
