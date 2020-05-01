use crate::ast::highlighter::Highlighter;
use crate::{Command, Value, CommandKind};
use std::collections::HashMap;

impl<'a> From<Highlighter<'a>> for Command<'a> {
    fn from(h: Highlighter<'a>) -> Self {
        let mut kvs: HashMap<&str, Value> = Default::default();
        kvs.insert("body", Value::String(h.code.into()));
        Command {
            cmd: h.lang,
            args: vec![],
            kvs,
            kind: CommandKind::SmartLink,
        }
    }
}