mod from;

use crate::{Command};
use std::borrow::Cow;
use std::fmt::{Debug, Formatter, Display};
use std::fmt;

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



impl<'a> Debug for Value<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Value::None => write!(f, "none"),
            Value::String(s) => write!(f, "{:?}", s),
            Value::Integer(s) => write!(f, "{}", s),
            Value::Decimal(s) => write!(f, "{}", s),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::List(_) => unimplemented!(),
            // Value::Dict(_) => unimplemented!(),
            Value::Command(node) => write!(f, "{}", node),
        }
    }
}

impl<'a> Display for Value<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Value::None => write!(f, ""),
            _ => write!(f, "{:?}", self),
        }
    }
}