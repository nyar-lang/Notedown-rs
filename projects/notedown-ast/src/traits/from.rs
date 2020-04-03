use crate::{Value, AST};

impl From<char> for AST {
    fn from(s: char) -> Self {
        AST::String(s.to_string())
    }
}

impl From<&str> for AST {
    fn from(s: &str) -> Self {
        AST::String(s.to_string())
    }
}

impl From<String> for AST {
    fn from(s: String) -> Self {
        AST::String(s)
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

