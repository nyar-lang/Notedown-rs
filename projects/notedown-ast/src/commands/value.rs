use crate::AST;
use std::{
    fmt,
    fmt::{Debug, Display, Formatter},
};

#[derive(Clone, PartialEq)]
pub enum Value {
    None,
    String(String),
    // Integer(String),
    // Decimal(String),
    Boolean(bool),
    List(Vec<Value>),
    // Dict(HashMap<String, Value>),
    Command(AST),
}

impl Debug for Value {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Value::None => write!(f, "none"),
            Value::String(s) => write!(f, "{:?}", s),
            // Value::Integer(s) => write!(f, "{}", s),
            // Value::Decimal(s) => write!(f, "{}", s),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::List(_) => unimplemented!(),
            // Value::Dict(_) => unimplemented!(),
            Value::Command(_) => unimplemented!(),
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

impl From<&str> for Value {
    fn from(s: &str) -> Self {
        Value::String(s.to_string())
    }
}

impl From<String> for Value {
    fn from(s: String) -> Self {
        Value::String(s)
    }
}

impl From<bool> for Value {
    fn from(b: bool) -> Self {
        Value::Boolean(b)
    }
}

impl Value {
    pub fn as_str(&self) -> &str {
        match self {
            Value::String(s) => s,
            Value::Boolean(b) => {
                if *b {
                    "true"
                }
                else {
                    "false"
                }
            }
            _ => unreachable!(),
        }
    }
    pub fn to_string(&self) -> String {
        match self {
            Value::String(s) => s.clone(),
            Value::Boolean(b) => format!("{}", b),
            _ => unreachable!(),
        }
    }
    pub fn trim(&self) -> String {
        match self {
            Value::String(s) => s.trim().to_string(),
            Value::Boolean(b) => format!("{}", b),
            _ => unreachable!(),
        }
    }
}
