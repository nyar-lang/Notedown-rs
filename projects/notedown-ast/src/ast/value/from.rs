use crate::Value;

impl From<String> for Value {
    fn from(s: String) -> Self {
        Value::String(s.into())
    }
}
