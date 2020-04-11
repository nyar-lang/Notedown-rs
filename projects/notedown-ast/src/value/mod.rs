use crate::{ Command};

mod display;
mod from;

#[derive(Clone)]
pub enum Value {
    None,
    String(String),
    // Integer(String),
    // Decimal(String),
    Boolean(bool),
    List(Vec<Value>),
    // Dict(HashMap<String, Value>),
    Command(Command),
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
    pub fn trim(&self) -> &str {
        match self {
            Value::String(s) => s.trim(),
            _ => self.as_str().trim(),
        }
    }
}
