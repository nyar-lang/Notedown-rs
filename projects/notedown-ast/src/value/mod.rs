mod from;

use crate::Command;
use std::fmt::{self, Debug, Display, Formatter};


#[derive(Clone)]
pub enum Value {
    None,
    String(String),
    Integer(String),
    Decimal(String),
    Boolean(bool),
    List(Vec<Value>),
    // Dict(HashMap<String, Value>),
    Command(Command),
}

impl Value {
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

impl Debug for Value {
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

impl Display for Value {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Value::None => write!(f, ""),
            _ => write!(f, "{:?}", self),
        }
    }
}
