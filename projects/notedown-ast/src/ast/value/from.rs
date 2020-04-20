use crate::Value;

impl<'a> From<String> for Value<'a> {
    fn from(s: String) -> Self {
        Value::String(s.into())
    }
}