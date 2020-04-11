use crate::Value;
use std::fmt::{Debug, Formatter, Display};
use std::fmt;

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