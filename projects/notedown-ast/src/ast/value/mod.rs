use crate::{Command};
use std::borrow::Cow;

mod display;
mod from;


#[derive(Clone)]
pub enum Value<'a> {
    None,
    String(Cow<'a, str>),
    Integer(Cow<'a, str>),
    Decimal(Cow<'a, str>),
    Boolean(bool),
    List(Vec<Value<'a>>),
    // Dict(HashMap<String, Value>),
    Command(Command<'a>),
}

impl<'a> Value<'a> {
    #[rustfmt::skip]
    pub fn as_str(&self) -> &str {
        match self {
            Value::String(s) => s,
            Value::Boolean(b) => {
                if *b { "true" } else { "false" }
            }
            _ => unreachable!(),
        }
    }
    pub fn trim(&self) -> &str {
        match self {
            Value::String(s) => s.trim(),
            _ => self.as_str().trim(),
        }
    }
}
